"""Render/check deterministic 2AX views; no analysis or target execution."""
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DIRECTORY = 'docs/stage2ax-evidence/'


def build(root=ROOT):
    here = root / DIRECTORY
    def read(name):
        return json.loads((here / name).read_text())

    findings, anchors = read('findings.json'), read('anchors.json')
    cases, questions = read('expected-cases.json'), read('questions.json')
    manifest, mapping = read('function-manifest.json'), read('case-map.json')
    summary = dict(stage='2AX', new_records=sum(m['bundle'] == 'rea.json.gz' for m in manifest),
                   reused_records=sum(m['bundle'] == 'reused.json.gz' for m in manifest),
                   unique_records=len(manifest), functions=len({m['name'] for m in manifest}),
                   anchors=len(anchors), findings=len(findings), expected_cases=len(cases),
                   case_mappings=len(mapping['items']),
                   question_status={q['id']: q['status'] for q in questions},
                   subquestion_status={s['id']: s['status'] for q in questions
                                       for s in q.get('subquestions', [])},
                   target_execution='none')
    ftext = ('# 2AX 结论总账\n\n观察、静态推导、Rust 设计和未知分开；没有原程序运行观测。'
             'F01—F23 保留 2AR 历史结论原文；本轮重评见后续结论及案例映射，不能据历史未知项忽略新增的限定结论。\n')
    for f in findings:
        links = ', '.join(f'[{a}](anchors.md#{a.lower()})' for a in f['anchors'])
        ftext += (f"\n## {f['id']}\n\n{f['kind']} · {f['status']}\n\n{f['claim']}\n\n"
                  f"边界：{f['limits']}\n\n锚点：{links}\n\nEvidence："
                  + ', '.join('`' + e + '`' for e in f['evidence']) + '\n')
    atext = '# 2AX 机器码锚点\n\n字节来自原 ELF；汇编来自对应完整记录，不是执行轨迹。\n'
    for a in anchors:
        atext += (f"\n## {a['id']}\n\n{a['name']} · `{a['evidence_id']}` · file offset {a['file_offset']}\n\n"
                  f"```text\n{a['instruction']}\nbytes: {a['bytes_hex']}\n```\n")
    def finding_links(ids):
        return ', '.join(f'[{f}](findings.md#{f.lower()})' for f in ids)

    qtext = '# 2AX 问题与退出域\n\nanswered 仅限命名子域；普通线程的局部结论不关闭 hook、其他线程或完整 GC。\n'
    for q in questions:
        qtext += (f"\n## {q['id']}：{q['question']}\n\n{q['status']} · {finding_links(q['findings'])}\n\n"
                  f"剩余：{q['remaining']}\n\n下一步：{q['next_action']}\n")
        for s in q.get('subquestions', []):
            qtext += (f"\n### {s['id']}\n\n{s['status']} · {finding_links(s['findings'])}\n\n"
                      f"边界：{s['remaining']}\n\n下一步：{s['next_action']}\n")
    ctext = ('# 2AX 静态／设计案例\n\n所有案例 runtime_executed=false；不是原程序运行 oracle。'
             'C01—C51 保留 2AR 原文，后续案例单列本轮重评，关联见案例映射。\n')
    for c in cases:
        ctext += (f"\n## {c['id']}\n\n{c['group']} · {c['basis']}\n\n输入：{c['input']}\n\n"
                  f"期望：{c['expected']}\n\n边界：{c['boundary']}\n\n依据：{finding_links(c['findings'])}\n")
    mtext = ('# 2AX 函数与记录\n\n符号范围不等于 CFG 范围；完整分段、哈希和 provider 限制见 function-manifest.json。\n\n'
             '| 函数 | 地址／符号字节数 | 来源与操作 | Evidence |\n|---|---|---|---|\n')
    for m in manifest:
        mtext += (f"| {m['name']} | {m['address']} / {m['size']} | {m['bundle']} · "
                  f"{m['operation']} | `{m['evidence_id']}` |\n")
    maptext = '# 2AX 历史案例映射\n\n保留 51 项原始 basis 与 findings；重评结果单列，不改写历史收据。\n'
    for row in mapping['items']:
        maptext += (f"\n## {row['source_case_id']}\n\n{row['source_basis']} · {row['status']}\n\n"
                    f"原依据：{', '.join(row['source_findings'])}\n\n本轮案例："
                    + ', '.join(f'[{c}](expected-cases.md#{c.lower()})' for c in row['case_ids'])
                    + f"\n\n限制：{row['limitations']}\n")
    return {'findings.md': ftext, 'anchors.md': atext, 'questions.md': qtext,
            'expected-cases.md': ctext, 'functions.md': mtext, 'case-map.md': maptext,
            'summary.json': json.dumps(summary, ensure_ascii=False, sort_keys=True, indent=2) + '\n'}, summary


def check(root=ROOT):
    outputs, summary = build(root)
    for name, expected in outputs.items():
        path = root / DIRECTORY / name
        if not path.is_file() or path.read_text() != expected:
            raise ValueError('stale or missing generated view: ' + name)
    return summary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    if args.write:
        outputs, summary = build()
        for name, output in outputs.items():
            (ROOT / DIRECTORY / name).write_text(output)
    else:
        summary = check()
    print(json.dumps(summary, ensure_ascii=False, sort_keys=True))


if __name__ == '__main__':
    main()
