"""Render deterministic human views; --write updates only 2AR generated files."""
import argparse
import json
from pathlib import Path
from verify_evidence import ROOT, HERE, load, digest, verify, require


def build(root=ROOT):
    base='docs/stage2ar-evidence/'
    summary=verify(root)
    findings=load(root,base+'findings.json');anchors=load(root,base+'anchors.json')
    cases=load(root,base+'expected-cases.json');questions=load(root,base+'questions.json')
    manifest=load(root,base+'function-manifest.json')
    source_paths=[str(p.relative_to(root)) for p in (root/base).iterdir()
                  if p.is_file() and (p.suffix=='.py' or p.name in ['baseline-inputs.json','function-manifest.json',
                  'reused-manifest.json','rea.json.gz','reused.json.gz','session.json','excluded-navigation.json',
                  'findings.json','anchors.json','expected-cases.json','questions.json','method.md','runtime-design.md'])]
    source_paths += ['docs/vm-capture-ownership-semantics.md','src/vm.rs','src/value.rs','src/container.rs',
                     'src/prepared.rs','src/host.rs','src/object.rs','tests/vm_core.rs','Cargo.toml','Cargo.lock']
    summary.update(runtime_changed=False,public_api_changed=False,dependencies_changed=False,
                   implementation_ready=False,stage2_complete=False,selected_next='2AS-ownership-design',
                   supported_opcodes=75,corpus_intersection=73,
                   source_sha256={p:digest((root/p).read_bytes()) for p in sorted(source_paths)})
    ftext='# 2AR结论账本\n\n观察/静态推导/设计/未知分开；没有原版运行观测。完整记录见函数清单及压缩证据。\n'
    for f in findings:
        links=', '.join(f'[{a}](anchors.md#{a.lower()})' for a in f['anchors'])
        ftext+=f"\n## {f['id']}\n\n{f['kind']} · {f['status']}\n\n{f['claim']}\n\n边界：{f['limits']}\n\n锚点：{links}\n\nEvidence："+', '.join('`'+x+'`' for x in f['evidence'])+'\n'
    atext='# 2AR机器码锚点\n\n逐指令字节取自原ELF，汇编原文来自对应完整记录；不是原程序执行轨迹。\n'
    for a in anchors:
        atext+=f"\n## {a['id']}\n\n{a['name']} · `{a['evidence_id']}` · file offset {a['file_offset']}\n\n```text\n{a['instruction']}\nbytes: {a['bytes_hex']}\n```\n"
    qtext='# 2AR问题状态\n\nanswered仅针对明确限定的接点；partial不代表完整兼容。\n'
    for q in questions:
        qtext+=f"\n## {q['id']}：{q['question']}\n\n{q['status']} · "+', '.join(f"[{x}](findings.md#{x.lower()})" for x in q['findings'])+f"\n\n剩余：{q['remaining']}\n\n下一步：{q['next_action']}\n"
    ctext='# 2AR静态/设计期望案例\n\n所有案例runtime_executed=false；不把静态推导当作原版运行oracle。\n'
    for c in cases:
        ctext+=f"\n## {c['id']} · {c['group']}\n\n{c['basis']}\n\n输入：{c['input']}\n\n期望：{c['expected']}\n\n边界：{c['boundary']}\n\n依据："+', '.join(f"[{x}](findings.md#{x.lower()})" for x in c['findings'])+'\n'
    mtext='# 2AR函数与记录清单\n\n符号范围不是Ghidra CFG范围；完整inclusive分段和哈希见function-manifest.json。所有provider限制保留在完整记录与清单。\n\n| 函数 | 地址/符号字节数 | 记录来源与操作 | Evidence |\n|---|---|---|---|\n'
    for m in manifest:
        mtext+=f"| {m['name']} | {m['address']} / {m['size']} | {m['bundle']} · {m['operation']} | `{m['evidence_id']}` |\n"
    return {'findings.md':ftext,'anchors.md':atext,'questions.md':qtext,'expected-cases.md':ctext,'functions.md':mtext,
            'summary.json':json.dumps(summary,ensure_ascii=False,sort_keys=True,indent=2)+'\n'},summary


def check_outputs(outputs,here):
    for name,output in outputs.items():
        require((here/name).is_file() and (here/name).read_text()==output,'stale generated output: '+name)


def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--write',action='store_true');args=parser.parse_args()
    outputs,summary=build()
    if args.write:
        for name,output in outputs.items():(HERE/name).write_text(output)
    else:check_outputs(outputs,HERE)
    print(json.dumps({k:v for k,v in summary.items() if k!='source_sha256'},ensure_ascii=False,sort_keys=True))


if __name__=='__main__':main()
