"""Validate design provenance/model coverage and render views; no target execution."""
import argparse
import hashlib
import json
from pathlib import Path
import re

ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent


def require(condition,message):
    if not condition:raise ValueError(message)


def digest(data):return hashlib.sha256(data).hexdigest()


def canonical(value):return json.dumps(value,ensure_ascii=False,sort_keys=True,separators=(',',':')).encode()


def safe_path(root,name):
    p=Path(name)
    require(bool(name) and not p.is_absolute() and '..' not in p.parts and '\\' not in name,'unsafe path')
    result=root/p
    require(result.resolve().is_relative_to(root.resolve()),'path escapes root')
    return result


def load(root,name):return json.loads(safe_path(root,name).read_text())


def source_check(root,source):
    lines=safe_path(root,source['path']).read_text().splitlines();line=source['line']
    require(isinstance(line,int) and 1<=line<=len(lines) and lines[line-1].strip()==source['excerpt'],'stale source reference')


def test_registry(root):
    return {name for p in (root/'docs/stage2as-design').glob('test_*.py')
            for name in re.findall(r'^    def (test_\w+)\(',p.read_text(),re.M)}


def validate(decisions,apis,mapping,plan,ar_cases,ar_findings,tests,root):
    require([d['id'] for d in decisions]==[f'D{i:02}' for i in range(1,16)],'decision inventory differs')
    for d in decisions:
        require(d['status']=='adopted-design' and d['decision'] and d['limits'] and d['authority']=='Rust design policy; not new native evidence','invalid decision status')
        require(d['ar_findings'] and set(d['ar_findings'])<=ar_findings,'dangling finding reference')
        require(d['sources'],'missing decision sources')
        for source in d['sources']:source_check(root,source)
    require([a['id'] for a in apis]==[f'API{i:02}' for i in range(1,17)],'API inventory differs')
    for a in apis:
        require(a['role'] in ('observation','host-ownership','host-construction','vm-input','borrowed-call','lifecycle-transfer')
                and a['future'] and a['notes'] and a['migration']=='T01','invalid API migration')
        if a['role']=='observation':require(a['owner_effect'] in ('none','explicit-transfer-only'),'observation creates implicit owner')
        source_check(root,a['source'])
    require([m['case_id'] for m in mapping]==[c['id'] for c in ar_cases],'case map partition differs')
    byid={c['id']:c for c in ar_cases}
    for m in mapping:
        require(m['source']=='docs/stage2ar-evidence/expected-cases.json','unexpected case source')
        raw=safe_path(root,m['source']).read_bytes();case=byid[m['case_id']]
        require(digest(raw)==m['source_sha256'] and digest(canonical(case))==m['case_sha256'],'case source changed')
        require(m['ar_basis']==case['basis'],'case authority changed')
        require(m['coverage'] in ('modeled','partial-model','native-boundary','unresolved') and m['note'],'invalid case coverage')
        if case['basis']=='unresolved':require(m['coverage']=='unresolved','native unknown hidden by model')
        if m['coverage'] in ('modeled','partial-model'):require(m['tests'],'modeled case lacks tests')
        require(set(m['tests'])<=tests,'unknown model test')
    require(plan['runtime_changed'] is False and plan['public_api_changed'] is False and plan['non_scalar_ready'] is False,'unsupported implementation claim')
    batches=plan['batches'];ids=[b['id'] for b in batches]
    require(ids==['T01','T02','T03','T04','T05'] and plan['selected_next']==batches[0]['stage_name']=='2AT-host-boundary','batch order differs')
    for i,b in enumerate(batches):
        require(set(b['depends_on'])<=set(ids[:i]),'invalid/cyclic batch dependency')
        require(b['status'] in ('specified','gated','blocked-native-contract') and b['scope'] and b['interfaces'] and b['acceptance'] and b['excluded'],'incomplete batch')
    require(batches[3]['status']=='gated' and batches[4]['status']=='blocked-native-contract','capture gate removed')


def check_enumeration(report,root):
    require(report['stage']=='2AS' and report['kind']=='independent-design-model' and report['native_execution'] is False
            and report['rust_execution'] is False and report['full_state_space_proof'] is False,'unsupported enumeration claim')
    require(report['input_sha256']=={name:digest((root/'docs/stage2as-design'/name).read_bytes()) for name in ('model.py','explore.py')},'stale enumeration inputs')
    require([p['profile'] for p in report['profiles']]==['ownership','graph','source-copy'],'enumeration profile inventory differs')
    from explore import BOUNDS
    for p in report['profiles']:
        require(p['bounds']==BOUNDS,'enumeration bounds differ')
        require(p['states']>=1 and p['accepted_transitions']>=0 and p['outside_profile']>=0 and p['limitations'],'invalid enumeration counts')
        require(p['cutoff'] is not p['completed_within_profile'],'cutoff falsely claimed complete')
    require(report['complete_for_declared_profiles']==all(p['completed_within_profile'] for p in report['profiles']),'enumeration completion differs')


def build(root=ROOT):
    base='docs/stage2as-design/'
    decisions=load(root,base+'decisions.json')['decisions'];apis=load(root,base+'api-migration.json')['items']
    mapping=load(root,base+'case-map.json')['items'];plan=load(root,base+'implementation-plan.json')
    ar_cases=load(root,'docs/stage2ar-evidence/expected-cases.json');ar_findings={f['id'] for f in load(root,'docs/stage2ar-evidence/findings.json')}
    validate(decisions,apis,mapping,plan,ar_cases,ar_findings,test_registry(root),root)
    enumeration=load(root,base+'enumeration.json');check_enumeration(enumeration,root)
    coverage={name:sum(m['coverage']==name for m in mapping) for name in ['modeled','partial-model','native-boundary','unresolved']}
    paths=[str(p.relative_to(root)) for p in (root/base).iterdir() if p.is_file() and (p.suffix=='.py' or p.name in
           ['decisions.json','api-migration.json','case-map.json','implementation-plan.json','enumeration.json','baseline-inputs.json','design.md','api-contract.md','method.md'])]
    summary=dict(stage='2AS',runtime_changed=False,public_api_changed=False,dependencies_changed=False,new_native_analysis=False,
                 target_execution='none',non_scalar_ready=False,stage2_complete=False,selected_next='2AT-host-boundary',
                 decisions=len(decisions),api_boundaries=len(apis),mapped_cases=len(mapping),coverage=coverage,
                 enumeration_states=sum(p['states'] for p in enumeration['profiles']),
                 enumeration_accepted=sum(p['accepted_transitions'] for p in enumeration['profiles']),
                 complete_for_declared_profiles=enumeration['complete_for_declared_profiles'],full_state_space_proof=False,
                 source_sha256={p:digest((root/p).read_bytes()) for p in sorted(paths)})
    dtext='# 2AS设计决策\n\n采用的Rust设计政策，不是原生新增观察；当前运行时未修改。\n'
    for d in decisions:
        dtext+=f"\n## {d['id']}：{d['title']}\n\n{d['decision']}\n\n边界：{d['limits']}\n\n依据："+', '.join(f"[{f}](../stage2ar-evidence/findings.md#{f.lower()})" for f in d['ar_findings'])+'\n'
    atext='# 2AS后续API迁移表\n\n本批不改API。以下接口摘要与api-contract.md共同构成T01契约，签名省略的错误均为Result/VmError。\n\n| 当前接口 | 角色 | 后续契约 | owner变化 | 说明 |\n|---|---|---|---|---|\n'
    for a in apis:atext+='| '+' | '.join(str(a[k]).replace('|','\\|') for k in ['current','role','future','owner_effect','notes'])+' |\n'
    ctext='# 2AS对2AR案例的映射\n\nmodeled仅指独立模型内的指定不变量；partial-model明确不覆盖全部原生语义；未解决项仍未解决。\n\n| 案例 | 原依据 | 模型范围 | 测试 | 限制 |\n|---|---|---|---|---|\n'
    for m in mapping:ctext+='| '+' | '.join([f"[{m['case_id']}](../stage2ar-evidence/expected-cases.md#{m['case_id'].lower()}--"+bygroup(ar_cases,m['case_id'])+')',m['ar_basis'],m['coverage'],', '.join(m['tests']) or '—',m['note']])+' |\n'
    ptext='# 2AS后续实施任务\n\n当前非标量捕获仍关闭；T01为下一批2AT。\n'
    for b in plan['batches']:
        ptext+=f"\n## {b['id']} · {b['stage_name']}\n\n{b['status']} · 前置："+(', '.join(b['depends_on']) or '无')+f"\n\n{b['scope']}\n\n接口："+'；'.join(b['interfaces'])+'\n\n验收：'+'；'.join(b['acceptance'])+'\n\n排除：'+'；'.join(b['excluded'])+'\n'
    return {'decisions.md':dtext,'api-migration.md':atext,'case-map.md':ctext,'implementation-plan.md':ptext,
            'summary.json':json.dumps(summary,ensure_ascii=False,sort_keys=True,indent=2)+'\n'},summary


def bygroup(cases,identity):return next(c['group'] for c in cases if c['id']==identity)


def check_outputs(outputs,here):
    for name,text in outputs.items():require((here/name).is_file() and (here/name).read_text()==text,'stale generated output: '+name)


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--write',action='store_true');args=p.parse_args()
    outputs,summary=build()
    if args.write:
        for name,text in outputs.items():(HERE/name).write_text(text)
    else:check_outputs(outputs,HERE)
    print(json.dumps({k:v for k,v in summary.items() if k!='source_sha256'},ensure_ascii=False,sort_keys=True))


if __name__=='__main__':main()
