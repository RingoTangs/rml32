"""Structural audit checks. These cannot prove native semantics or test adequacy."""
from pathlib import Path
import re


def local_file(root, name):
    path = Path(name)
    if path.is_absolute() or '..' in path.parts or not (root / path).resolve().is_relative_to(root.resolve()):
        raise ValueError('invalid local path: ' + name)
    if not (root / path).is_file():
        raise ValueError('missing file: ' + name)
    return root / path


def source_check(root, ref):
    lines = local_file(root, ref['path']).read_text().splitlines()
    line = ref['line']
    if not isinstance(line, int) or line < 1 or line > len(lines) or lines[line - 1].strip() != ref['excerpt']:
        raise ValueError('stale source location: ' + ref['path'])
    if not ref['excerpt']:
        raise ValueError('empty source excerpt')


def test_registry(root):
    registry = {}
    for parent in ('src', 'tests'):
        for path in sorted((root / parent).rglob('*.rs')):
            text = path.read_text()
            matches = list(re.finditer(r'#\[test\]\s*fn\s+(\w+)\s*\(', text))
            for i, match in enumerate(matches):
                # A locator window, not a Rust parser or a proof of assertion coverage.
                end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
                registry.setdefault(match[1], []).append(dict(
                    path=str(path.relative_to(root)), line=text[:match.start()].count('\n') + 1,
                    window=text[match.start():end]))
    return registry


def references(item, root, tests, require_tests=True):
    if not item['sources'] or not item['evidence']:
        raise ValueError('missing source/evidence')
    for ref in item['sources']:
        source_check(root, ref)
    for path in item['evidence']:
        local_file(root, path)
    if require_tests and not item['tests']:
        raise ValueError('missing related tests')
    for name in item['tests']:
        if name not in tests:
            raise ValueError('unknown Rust test: ' + name)


def validate(gaps_doc, caps_doc, priorities_doc, rows, root, tests):
    gaps = gaps_doc['gaps']
    ids = [g['id'] for g in gaps]
    if len(ids) != len(set(ids)):
        raise ValueError('duplicate gap id')
    by_id = {g['id']: g for g in gaps}
    for g in gaps:
        if g['status'] not in ('rejected', 'difference', 'evidence_gap'):
            raise ValueError('invalid gap status')
        if g['stage'] not in (2, 3, 4, 5, 6):
            raise ValueError('invalid stage')
        for field in ('id', 'title', 'domain', 'current_behavior', 'unknowns', 'exit_criteria'):
            if not g[field]:
                raise ValueError('empty gap field: ' + field)
        references(g, root, tests, require_tests=g['stage'] == 2)
        for dependency in g['depends_on']:
            if dependency not in by_id:
                raise ValueError('dangling dependency')
            if by_id[dependency]['stage'] > g['stage']:
                raise ValueError('dependency points to later stage')
    visited, active = set(), set()

    def visit(key):
        if key in active:
            raise ValueError('cyclic gap dependency')
        if key in visited:
            return
        active.add(key)
        for dependency in by_id[key]['depends_on']:
            visit(dependency)
        active.remove(key)
        visited.add(key)
    for key in ids:
        visit(key)
    linked = set()
    for row in rows:
        source_check(root, row['sources'][0])
        for ref in row['sources'][1:]:
            source_check(root, ref)
        if [x['text'] for x in row['exclusion_links']] != row['excluded_cases']:
            raise ValueError('unmapped exclusion')
        if not row['assertion_review']:
            raise ValueError('missing human review boundary')
        for exclusion in row['exclusion_links']:
            if not exclusion['gap_ids'] or any(g not in by_id for g in exclusion['gap_ids']):
                raise ValueError('dangling exclusion gap')
            linked.update(exclusion['gap_ids'])
    caps = caps_doc['capabilities']
    if len({c['id'] for c in caps}) != len(caps):
        raise ValueError('duplicate capability')
    covered = set()
    for cap in caps:
        if cap['status'] not in ('open', 'outside_stage2') or not cap['exit_criteria']:
            raise ValueError('invalid capability gate')
        if not cap['gap_ids'] or any(g not in by_id for g in cap['gap_ids']):
            raise ValueError('dangling capability gap')
        if cap['status'] == 'outside_stage2' and any(by_id[g]['stage'] == 2 for g in cap['gap_ids']):
            raise ValueError('stage2 gap hidden outside scope')
        if cap['status'] == 'open' and any(by_id[g]['stage'] != 2 for g in cap['gap_ids']):
            raise ValueError('later-stage gap in stage2 gate')
        references(cap, root, tests, require_tests=cap['status'] == 'open')
        covered.update(cap['gap_ids'])
    if covered != set(ids):
        raise ValueError('gap without capability gate')
    if not linked.issubset(covered):
        raise ValueError('uncovered exclusion')
    if caps_doc['stage2_complete'] is not False or gaps_doc['claim'] != 'bounded':
        raise ValueError('unsupported complete compatibility claim')
    candidates = priorities_doc['candidates']
    if len(candidates) < 3 or [c['rank'] for c in candidates] != list(range(1, len(candidates) + 1)):
        raise ValueError('invalid priority order')
    if len({c['id'] for c in candidates}) != len(candidates) or priorities_doc['selected'] != candidates[0]['id']:
        raise ValueError('ambiguous selected priority')
    for candidate in candidates:
        if candidate['next_action'] != 'evidence' or not all(candidate[f] for f in ('reason','readiness','static_relevance')):
            raise ValueError('unsupported readiness claim')
        if not candidate['gap_ids'] or any(g not in by_id or by_id[g]['stage'] != 2 for g in candidate['gap_ids']):
            raise ValueError('invalid priority gap')
    return dict(gaps=len(gaps), stage2_gaps=sum(g['stage'] == 2 for g in gaps),
                outside_stage2_gaps=sum(g['stage'] != 2 for g in gaps),
                capability_gates=len(caps), stage2_complete=False,
                selected_next=priorities_doc['selected'], exclusion_items=sum(len(r['excluded_cases']) for r in rows))


def render(gaps, caps, priorities):
    def links(refs):
        return ', '.join(f"[{r['path']}:{r['line']}](../../{r['path']}#L{r['line']})" for r in refs)
    def evidence(paths):
        return ', '.join(f'[{p}](../../{p})' for p in paths)
    gap_text = '# 2AH 缺口清单\n\nrejected=当前明确拒绝/缺失；difference=已知语义差异；evidence_gap=证据不足。均不代表原版拒绝。依赖箭头指向前置工作，不是原生调用图。\n'
    for g in gaps['gaps']:
        gap_text += f"\n## {g['id']}：{g['title']}\n\n阶段 {g['stage']} · {g['status']} · {g['domain']}\n\n当前：{g['current_behavior']}\n\n定位：{links(g['sources'])}\n\n证据：{evidence(g['evidence'])}\n\n相关测试：{', '.join('`'+t+'`' for t in g['tests']) or '无运行适配测试；仅有静态调查'}\n\n待闭合：{'；'.join(g['unknowns'])}\n\n前置：{', '.join(g['depends_on']) or '无已登记硬前置'}\n\n退出：{g['exit_criteria']}\n"
    cap_text = '# 2AH 当前能力与退出门槛\n\n状态基于2AG源码；不是2P历史表的原地改写。每个open门槛仍含未解决项，故阶段2未完成。测试为相关合成场景，存在性校验不证明断言充分或原生等价。\n'
    for c in caps['capabilities']:
        cap_text += f"\n## {c['id']}：{c['title']}\n\n状态：{c['status']}；缺口：{', '.join(c['gap_ids'])}。\n\n当前：{c['current_behavior']}\n\n定位：{links(c['sources'])}\n\n相关测试：{', '.join('`'+t+'`' for t in c['tests']) or '无运行适配测试'}\n\n证据：{evidence(c['evidence'])}\n\n退出：{c['exit_criteria']}\n"
    priority_text = '# 2AH 后续优先级\n\n'+priorities['policy']+'\n\n首选：'+priorities['selected']+'。所有候选下一步均为定向取证；不由本批审计直接放宽Rust准备器。\n'
    for p in priorities['candidates']:
        priority_text += f"\n## {p['rank']}. {p['id']}：{p['title']}\n\n缺口：{', '.join(p['gap_ids'])}；行动：{p['next_action']}。\n\n理由：{p['reason']}\n\n静态相关性：{p['static_relevance']}\n\n证据成熟度：{p['readiness']}\n"
    return {'gaps.md':gap_text,'capabilities.md':cap_text,'priorities.md':priority_text}


def verify_hashes(root, entries):
    import hashlib
    for name, expected in entries.items():
        if hashlib.sha256(local_file(root, name).read_bytes()).hexdigest() != expected:
            raise ValueError('hash mismatch: ' + name)


def validate_changes(changes, previous, current):
    old = {r['opcode']: r for r in previous}
    new = {r['opcode']: r for r in current}
    if [r['opcode'] for r in changes] != [r['opcode'] for r in current] or set(old) != set(new):
        raise ValueError('change ledger partition mismatch')
    for entry in changes:
        a, b = old[entry['opcode']], new[entry['opcode']]
        for before, after, field in [
                ('previous_exclusions', 'current_exclusions', 'excluded_cases'),
                ('previous_tests', 'current_tests', 'related_tests'),
                ('previous_implemented_cases', 'current_implemented_cases', 'implemented_cases')]:
            if entry[before] != a[field] or entry[after] != b[field]:
                raise ValueError('change ledger differs from source reviews')
        if not entry['reason']:
            raise ValueError('change ledger missing reason')
