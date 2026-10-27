"""Fail-closed projection of the pinned test hooks and four exact factories."""
import re
TOKEN=re.compile(r'//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])\'|[A-Za-z_][A-Za-z_0-9]*|[0-9]+|\S',re.S)
def tokens(s):return [v for v in TOKEN.findall(s) if not v.startswith(('//','/*'))]
def end_item(t,i):
    stack=[];angle=0
    while i<len(t):
        x=t[i];i+=1
        if x in ('(','[','{'):stack.append(x)
        elif x in (')',']','}'):
            assert stack,'unbalanced item';a=stack.pop();assert(a,x)in [('(',')'),('[',']'),('{','}')]
            if not stack and x=='}':return i
        elif not stack and x=='<':angle+=1
        elif not stack and x=='>':angle=max(0,angle-1)
        elif not stack and not angle and x in (';',','):return i
    raise AssertionError('unterminated item')
def cfg(t):
    out=[];i=0;yes=tokens('#[cfg(test)]');no=tokens('#[cfg(not(test))]')
    while i<len(t):
        if t[i:i+len(yes)]==yes:i=end_item(t,i+len(yes))
        elif t[i:i+len(no)]==no:i+=len(no)
        elif t[i:i+3]==['#','[','cfg']:raise AssertionError('unsupported cfg')
        else:out.append(t[i]);i+=1
    return out
TARGETS={'life_cell':'crate::cell::Cell::new','life_plan':'crate::copy_plan::CopyPlan::default','life_array':'crate::Value::array','life_mapping':'crate::Value::mapping'}
def replace_all(t,a,b):
    out=[];i=0
    while i<len(t):
        if t[i:i+len(a)]==a:out+=b;i+=len(a)
        else:out.append(t[i]);i+=1
    return out

def production(s):
    t=cfg(tokens(s));out=[];i=0
    while i<len(t):
        if t[i:i+2]==['macro_rules','!']:
            name=t[i+2];assert name in TARGETS,'unreviewed macro definition';end=end_item(t,i)
            body='($owner:expr) => { '+TARGETS[name]+'() };' if name=='life_plan' else '($owner:expr, $value:expr $(,)?) => { '+TARGETS[name]+'($value) };'
            assert t[i:end]==tokens('macro_rules! '+name+' { '+body+' }'),'factory implementation changed'
            i=end;continue
        if t[i] in TARGETS and t[i+1:i+3]==['!','(']:
            name=t[i];j=i+3;stack=[];args=[[]]
            while True:
                x=t[j];j+=1
                if not stack and x==')':break
                if not stack and x==',':args.append([]);continue
                args[-1].append(x)
                if x in ('(','[','{'):stack.append(x)
                elif x in (')',']','}'):assert stack;stack.pop()
            assert args[0]==['self'],'factory owner expression changed'
            value=[] if name=='life_plan' else args[1];assert len(args)<4 and (len(args)<3 or not args[2])
            out+=tokens(TARGETS[name]+'(')+value+[')'];i=j;continue
        out.append(t[i]);i+=1
    for short,full in [('Cell::new','crate::cell::Cell::new'),('CopyPlan::default','crate::copy_plan::CopyPlan::default'),('Value::array','crate::Value::array'),('Value::mapping','crate::Value::mapping')]:
        # Qualify only unqualified uses, not an already absolute suffix.
        a=tokens(short);b=tokens(full);normal=[];i=0
        while i<len(out):
            if out[i:i+len(a)]==a and (i<2 or out[i-2:i]!=[':',':']):normal+=b;i+=len(a)
            else:normal.append(out[i]);i+=1
        out=normal
    # A trailing comma in these known function argument lists is optional.
    for target in TARGETS.values():
        marker=tokens(target+'(');i=0
        while i<len(out):
            if out[i:i+len(marker)]==marker:
                j=i+len(marker);level=1
                while level:
                    if out[j]=='(':level+=1
                    if out[j]==')':level-=1
                    j+=1
                if out[j-2]==',':out.pop(j-2)
            i+=1
    out=replace_all(out,tokens('.map(|v| crate::cell::Cell::new(v))'),tokens('.map(crate::cell::Cell::new)'))
    return out

def test_bodies(s):
    t=tokens(s);out=[]
    for i in range(len(t)-5):
        if t[i:i+5]!=['#','[','test',']','fn']:continue
        j=i+5
        while t[j]!='{':j+=1
        end=end_item(t,j);out.append(t[i+4:end])
    return out
