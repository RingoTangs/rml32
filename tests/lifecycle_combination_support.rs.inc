// Shared production-facing scenarios. No expected data is accepted by the protocol.
include!("boundary_support.rs.inc");
fn q(j:&serde_json::Value)->LifecycleLimits {
    LifecycleLimits{max_objects:j["max_objects"].as_u64().unwrap()as usize,max_function_heads:j["max_function_heads"].as_u64().unwrap()as usize,max_ordinary_refs_per_object:j["max_ordinary_refs_per_object"].as_u64().unwrap()as usize,max_exec_refs_per_object:j["max_exec_refs_per_object"].as_u64().unwrap()as usize,max_owners_per_header:j["max_owners_per_header"].as_u64().unwrap()as usize}
}
fn view(v:ValueSnapshot)->serde_json::Value {
    use serde_json::json;
    match v {ValueSnapshot::Int(n)=>json!(["int",n]),ValueSnapshot::Undefined=>json!(["undefined"]),ValueSnapshot::Object(_)=>json!(["object"]),ValueSnapshot::Array{elements,..}=>json!(["array",elements.into_iter().map(view).collect::<Vec<_>>()]),ValueSnapshot::Void=>json!(["void"]),other=>panic!("unlisted snapshot {other:?}")}
}
type Step=fn(&mut Vm,Option<usize>)->(Result<RunOutcome,VmError>,usize);
type Probe=fn(&ObjectStore)->Box<dyn Fn()->serde_json::Value>;
fn mapping_case(t:&serde_json::Value,step:Step)->serde_json::Value {
    use serde_json::json;
    let mut phase="setup";let mut checkpoints=Vec::new();
    let result=(||->Result<serde_json::Value,VmError>{
        let mut s=ObjectStore::with_bounded_lifecycle(ValueLimits::default(),q(&t["limits"]))?;
        let reuse=t["reuse"].as_bool().unwrap();let caught=t["caught"].as_bool().unwrap();
        let mut nodes=vec![(-1,0,0x39),(0,0,0x37),(997,0,0x3b),(-2,0,0x23),(1,-2,0x12),(-1,-2,0x50),(-1,0,0x37)];
        if reuse{nodes.splice(0..0,[(1,0,0x37),(-1,0,0x4d),(0,0,0x33)]);}
        let leaf=fun(if caught{1}else{0},2,2,255,&nodes);
        let functions=if caught {
            let body=[(1,0,0x37),(0,0,0x37),(1,2,7),(9,0,0x3b),(0,0,0x3b),(0,0,0x19)];
            let mut n=vec![((body.len()+2)as i16,0,0x2c)];n.extend(body);n.extend([(0,0,0x1a),(1,0,0x1f),(0,0,0x0e)]);
            vec![fun(0,2,0,255,&n),leaf]
        }else{vec![leaf]};
        let p=s.register_program(prepare_program(image(&functions,0,&[])).unwrap())?;
        let active=s.create(p,&[])?;let payload_object=s.create(p,&[])?;
        let payload=s.value(ValueInput::Object(payload_object))?;
        if t["payload"]=="shell"{s.edit().remove(payload_object)?;}
        let nested=if t["payload"]=="nested"{Some(s.array(&[ValueInput::Handle(&payload)])?)}else{None};
        let value=nested.as_ref().unwrap_or(&payload);
        let map=s.mapping(&[(ValueInput::Int(0),ValueInput::Handle(value)),(ValueInput::Int(1),ValueInput::Handle(value))])?;
        let alias=s.array(&[ValueInput::Int(9);3])?;
        let mut vm=Vm::new(s);phase="start";
        vm.start(active,0,&[ValueInput::Handle(&map),if reuse{ValueInput::Handle(&alias)}else{ValueInput::Undefined}])?;
        if caught{for _ in 0..32{if vm.frame_depth()==2{break;}vm.run(1)?;}assert_eq!(vm.frame_depth(),2);}
        phase="prefix";vm.run(6+if reuse{3}else{0})?;
        if t["iteration"]==1{phase="prior_iteration";vm.run(1)?;checkpoints.push(json!({"prior_alias":view(alias.snapshot()?)}));}
        phase="target";
        let before=vm.stack()?;let alias_before=alias.snapshot()?;
        let source_before=map.snapshot()?;
        let fail=t.get("fail_after").and_then(serde_json::Value::as_u64).map(|n|n as usize);
        let (out,calls)=step(&mut vm,fail);
        if let Err(e)=out {
            assert_eq!(e.kind,VmErrorKind::Limit,"unexpected target error");
            assert_eq!(vm.stack()?,before,"partial stack publication");
            assert_eq!(alias.snapshot()?,alias_before,"partial reused output publication");
            assert_eq!(map.snapshot()?,source_before,"failed step normalized a source");
            assert_eq!(vm.run(100).unwrap_err(),e,"fatal step replayed");
            checkpoints.push(json!({"target_error":"Limit","atomic":true,"calls":calls,"frames":vm.frame_depth()}));
            return Ok(json!({"target":"rejected","checkpoints":checkpoints}));
        }
        checkpoints.push(json!({"target_error":null,"calls":calls}));
        phase="finish";assert_eq!(vm.run(100)?,RunOutcome::Finished(CompletionMeta::Returned));
        let returned=vm.take_returned()?;let snap=returned.snapshot()?;
        if caught{assert_eq!(snap,ValueSnapshot::Int(1));}
        else {
            let ValueSnapshot::Array{id,elements}=snap.clone()else{panic!("expected mapping output")};
            assert_eq!(elements.len(),3);assert!(matches!(elements[0],ValueSnapshot::Int(0|1)));assert_eq!(elements[2],ValueSnapshot::Int(1));
            match t["payload"].as_str().unwrap(){"object"=>assert_eq!(elements[1],ValueSnapshot::Object(payload_object)),"shell"=>assert_eq!(elements[1],ValueSnapshot::Undefined),"nested"=>assert_eq!(elements[1],value.snapshot()?),_=>unreachable!()}
            if reuse{let ValueSnapshot::Array{id:old,..}=alias_before else{unreachable!()};assert_eq!(id,old);assert_eq!(alias.snapshot()?,snap);}
        }
        assert_eq!(vm.frame_depth(),0);
        Ok(json!({"target":"accepted","returned":view(snap),"frames":0,"checkpoints":checkpoints}))
    })();
    match result{Ok(v)=>json!({"id":t["id"],"group":"mapping","result":v}),Err(e)=>json!({"id":t["id"],"group":"mapping","phase":phase,"error":format!("{:?}",e.kind),"message":e.message})}
}
fn close_case(t:&serde_json::Value,probe:Probe)->serde_json::Value {
    use serde_json::json;
    let mut phase="setup";let mut rows=Vec::new();let mut inspect:Option<Box<dyn Fn()->serde_json::Value>>=None;
    let result=(||->Result<(),VmError>{
        let mut s=ObjectStore::with_bounded_lifecycle(ValueLimits::default(),q(&t["limits"]))?;inspect=Some(probe(&s));
        let p=s.register_program(prepare_program(image(&[fun(0,0,0,2,&[(9,0,0x3b)])],0,&[])).unwrap())?;
        let o0=s.create(p,&[])?;let o1=s.create(p,&[])?;
        let mut roots=[Some(s.value(ValueInput::Object(o0))?),Some(s.array(&[ValueInput::Object(o1)])?)];
        let mut vm=Vm::new(s);phase="queue";
        for id in if t["state"]=="queued"{vec![o0,o1]}else{vec![o0]}{
            vm.start(id,0,&[])?;vm.run(1)?;vm.objects_mut().remove(id)?;
            if t["state"]=="queued"{assert_eq!(vm.run(64)?,RunOutcome::Finished(CompletionMeta::Returned));drop(vm.take_returned()?);}
        }
        let mut live=Some(vm);let mut closed=false;
        for action in t["actions"].as_array().unwrap(){
            phase="actions";
            match action.as_str().unwrap(){
                "drop0"=>drop(roots[0].take()),"drop1"=>drop(roots[1].take()),
                "maintain"=>{let b=t["budget"].as_u64().unwrap()as usize;let p=live.as_mut().unwrap().maintain_lifecycle(LifecycleBudget{host_roots:b,objects:b,metadata_entries:b})?;assert!(p.retired_roots<=b&&p.drained_objects<=b&&p.metadata_examined<=b);rows.push(json!({"op":"maintain","retired":p.retired_roots,"drained":p.drained_objects,"examined":p.metadata_examined,"queued":p.queued_objects,"stop":format!("{:?}",p.stop)}));}
                "close"=>{drop(live.take());closed=true;}
                _=>panic!("close action"),
            }
            if closed{for root in roots.iter().flatten(){assert_eq!(root.retain().unwrap_err().kind,VmErrorKind::InvalidState);assert_eq!(root.snapshot().unwrap_err().kind,VmErrorKind::InvalidState);}}
            let observation=inspect.as_ref().unwrap()();rows.push(json!({"op":action,"closed":closed,"roots":roots.iter().filter(|r|r.is_some()).count(),"observation":observation}));
        }
        Ok(())
    })();
    let final_observation=inspect.map(|f|f()).unwrap_or(json!(null));
    match result{Ok(())=>json!({"id":t["id"],"group":"close","checkpoints":rows,"final":final_observation}),Err(e)=>json!({"id":t["id"],"group":"close","phase":phase,"error":format!("{:?}",e.kind),"message":e.message,"final":final_observation})}
}
fn replay_cases(step:Step,probe:Probe){
    use std::io::{BufRead,Write};
    let input=std::fs::File::open(std::env::var_os("RML32_COMBINATIONS_IN").unwrap()).unwrap();let mut out=std::io::BufWriter::new(std::fs::File::create(std::env::var_os("RML32_COMBINATIONS_OUT").unwrap()).unwrap());let mut count=0;
    for line in std::io::BufReader::new(input).lines(){let t:serde_json::Value=serde_json::from_str(&line.unwrap()).unwrap();assert!(t.get("expected").is_none());let row=if t["group"]=="mapping"{mapping_case(&t,step)}else{close_case(&t,probe)};writeln!(out,"{row}").unwrap();count+=1;}
    out.flush().unwrap();println!("COMBINATION_PUBLIC_REPLAY {count}");
}
