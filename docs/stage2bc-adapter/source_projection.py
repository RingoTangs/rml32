"""Compare non-test token streams; not a general Rust preprocessor.

The supported syntax is checked against the four pinned source files. Unknown
cfg expressions fail closed. The sole normalization is an equivalent temporary
for constructing an OwnedPayload before inserting it into a workspace vector.
"""
import re
TOKEN=re.compile(r'//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])\'|[A-Za-z_][A-Za-z_0-9]*|[0-9]+|\S',re.S)
def tokens(text):return [x for x in TOKEN.findall(text) if not x.startswith(('//','/*'))]
def production(text):
    t=tokens(text);out=[];i=0
    marker=['#','[','cfg','(','test',')',']']
    while i<len(t):
        if t[i:i+7]!=marker:
            if t[i:i+3]==['#','[','cfg']:raise AssertionError('unsupported cfg expression')
            out.append(t[i]);i+=1;continue
        i+=7;stack=[];angle=0
        while i<len(t):
            x=t[i];i+=1
            if x in ('(','[','{'):stack.append(x)
            elif x in (')',']','}'):
                assert stack,'unbalanced test item';opening=stack.pop()
                assert (opening,x) in [('(',')'),('[',']'),('{','}')]
                if not stack and x=='}':break
            elif not stack and x=='<':angle+=1
            elif not stack and x=='>':angle=max(0,angle-1)
            elif not stack and not angle and x in (';',','):break
        else:raise AssertionError('unterminated test item')
    old=tokens('prepared.push((index(&c), OwnedPayload::new(SlotPayload::Value(v))?));')
    new=tokens('let payload = OwnedPayload::new(SlotPayload::Value(v))?; prepared.push((index(&c), payload));')
    for i in range(len(out)-len(new)+1):
        if out[i:i+len(new)]==new:out=out[:i]+old+out[i+len(new):];break
    return out

def test_bodies(text):
    t=tokens(text);result={}
    for i in range(len(t)-5):
        if t[i:i+4]!=['#','[','test',']']:continue
        j=i+4
        if t[j]!='fn':continue
        name=t[j+1];start=j
        while t[j]!='{':j+=1
        level=1;j+=1
        while level:
            if t[j]=='{':level+=1
            if t[j]=='}':level-=1
            j+=1
        if name in result:raise AssertionError('ambiguous legacy test name')
        result[name]=t[start:j]
    return result
