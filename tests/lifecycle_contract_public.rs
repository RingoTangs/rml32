//! The 2BJ public projection runs against the normal, non-cfg(test) library.
use rml32::*;
use serde_json::{Value as Json, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{BufRead, Write},
};
include!("boundary_support.rs.inc");
fn invalid() -> VmError {
    panic!("unrepresentable public trace action")
}
struct Driver {
    store: Option<ObjectStore>,
    vm: Option<Vm>,
    program: ProgramId,
    objects: BTreeMap<String, ObjectId>,
    roots: BTreeMap<String, HostValue>,
    headers: BTreeMap<u64, String>,
    known: BTreeSet<String>,
}
impl Driver {
    fn new(q: &Json) -> Self {
        let get = |key: &str| q[key].as_u64().unwrap() as usize;
        let mut s = ObjectStore::with_bounded_lifecycle(
            ValueLimits::default(),
            LifecycleLimits {
                max_objects: get("max_objects"),
                max_function_heads: get("max_function_heads"),
                max_ordinary_refs_per_object: get("max_ordinary_refs_per_object"),
                max_exec_refs_per_object: get("max_exec_refs_per_object"),
                max_owners_per_header: get("max_owners_per_header"),
            },
        )
        .unwrap();
        let program = s
            .register_program(
                prepare_program_with_callable_values(
                    image(
                        &[
                            fun(0, 0, 0, 2, &[(9, 0, 0x3b)]),
                            fun(1, 0, 0, 9, &[(0, 0, 0x17)]),
                        ],
                        1,
                        &[],
                    ),
                    GlobalBankSchema { slots: 0 },
                )
                .unwrap(),
            )
            .unwrap();
        Self {
            store: Some(s),
            vm: None,
            program,
            objects: BTreeMap::new(),
            roots: BTreeMap::new(),
            headers: BTreeMap::new(),
            known: BTreeSet::new(),
        }
    }
    fn view(&self) -> StoreRef<'_> {
        match &self.vm {
            Some(v) => v.objects(),
            None => self.store.as_ref().unwrap().view(),
        }
    }
    fn event(&mut self, a: &Json) -> Result<Json, VmError> {
        let text = |key: &str| a[key].as_str().unwrap();
        match text("op") {
            "object" => {
                let p = self.program;
                let o = self
                    .store
                    .as_mut()
                    .unwrap()
                    .create(p, &[ValueInput::Undefined])?;
                self.objects.insert(text("id").into(), o);
            }
            "head" => {
                let v = if text("kind") == "function" {
                    let owner = self.objects[a["creator"].as_str().unwrap()];
                    let mut vm = Vm::new(self.store.take().unwrap());
                    let result = (|| {
                        vm.start(owner, 1, &[])?;
                        vm.run(64)?;
                        vm.take_returned()
                    })();
                    self.store = Some(vm.into_objects());
                    result?
                } else {
                    self.store
                        .as_ref()
                        .unwrap()
                        .array(&[ValueInput::Undefined])?
                };
                let id = match v.snapshot()? {
                    ValueSnapshot::Array { id, .. } => id,
                    ValueSnapshot::Function(f) => f.id,
                    _ => return Err(invalid()),
                };
                self.headers.insert(id, text("id").into());
                self.roots.insert(text("dest").into(), v);
                self.known.insert(text("dest").into());
            }
            "put" => {
                let value = match a["value"][0].as_str().unwrap() {
                    "object" => ValueInput::Object(self.objects[a["value"][1].as_str().unwrap()]),
                    _ => ValueInput::Undefined,
                };
                let v = self.view().value(value)?;
                self.roots.insert(text("dest").into(), v);
                self.known.insert(text("dest").into());
            }
            "copy" => {
                let v = self
                    .roots
                    .get(text("source"))
                    .ok_or_else(invalid)?
                    .retain()?;
                self.roots.insert(text("dest").into(), v);
                self.known.insert(text("dest").into());
            }
            "move" => {
                let v = self.roots.remove(text("source")).ok_or_else(invalid)?;
                self.roots.insert(text("dest").into(), v);
                self.known.insert(text("dest").into());
            }
            "drop" => {
                self.roots.remove(text("slot"));
            }
            "exec" => {
                let mut vm = Vm::new(self.store.take().unwrap());
                let result = vm
                    .start(self.objects[text("object")], 0, &[])
                    .and_then(|_| vm.run(1).map(|_| ()));
                self.vm = Some(vm);
                result?;
            }
            "exit" => {
                self.store = Some(self.vm.take().ok_or_else(invalid)?.into_objects());
            }
            "delete" => {
                let id = self.objects[text("object")];
                if let Some(vm) = &mut self.vm {
                    vm.objects_mut().remove(id)?;
                } else {
                    self.store.as_mut().unwrap().remove(id)?;
                }
            }
            "maintain" => {
                let b = &a["budget"];
                let budget = LifecycleBudget {
                    host_roots: b["host_roots"].as_u64().unwrap() as usize,
                    objects: b["objects"].as_u64().unwrap() as usize,
                    metadata_entries: b["metadata_entries"].as_u64().unwrap() as usize,
                };
                let p = match &mut self.vm {
                    Some(vm) => vm.maintain_lifecycle(budget)?,
                    None => self.store.as_mut().unwrap().maintain_lifecycle(budget)?,
                };
                return Ok(
                    json!({"retired_roots":p.retired_roots,"drained_objects":p.drained_objects,"reaped_objects":p.reaped_objects,"queued_objects":p.queued_objects,"pending_roots":p.pending_roots}),
                );
            }
            "transfer" => {
                self.store = Some(Vm::new(self.store.take().unwrap()).into_objects());
            }
            "close" => {
                drop(self.store.take());
                for root in self.roots.values() {
                    assert_eq!(root.retain().unwrap_err().kind, VmErrorKind::InvalidState);
                }
                return Ok(json!({"closed":true}));
            }
            _ => return Err(invalid()),
        }
        Ok(Json::Null)
    }
    fn observe(&self) -> Json {
        if self.store.is_none() && self.vm.is_none() {
            return json!({"closed":true});
        }
        let objects = self
            .objects
            .iter()
            .map(|(n, id)| (n.clone(), self.view().get(*id).is_ok()))
            .collect::<BTreeMap<_, _>>();
        let mut functions = BTreeMap::new();
        let slots = self
            .known
            .iter()
            .map(|n| {
                let v = if let Some(root) = self.roots.get(n) {
                    match root.snapshot().unwrap() {
                        ValueSnapshot::Object(id) => json!([
                            "object",
                            self.objects.iter().find(|(_, o)| **o == id).unwrap().0
                        ]),
                        ValueSnapshot::Function(f) => {
                            let label = &self.headers[&f.id];
                            functions.insert(label.clone(), f.owners);
                            json!(["header", label])
                        }
                        ValueSnapshot::Array { id, .. } | ValueSnapshot::Mapping { id, .. } => {
                            json!(["header", self.headers[&id]])
                        }
                        _ => json!(["scalar", null]),
                    }
                } else {
                    json!(["scalar", null])
                };
                (n.clone(), v)
            })
            .collect::<BTreeMap<_, _>>();
        json!({"objects":objects,"slots":slots,"function_owners":functions,"frames":self.vm.as_ref().map_or(0,Vm::frame_depth)})
    }
}
#[test]
#[ignore = "run by 2BJ public_diff.py against the normal library"]
fn replay_public_contract_traces() {
    let input = std::env::var_os("RML32_D08_TRACES").unwrap();
    let output = std::env::var_os("RML32_D08_OBSERVATIONS").unwrap();
    let reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
    let mut writer = std::io::BufWriter::new(std::fs::File::create(output).unwrap());
    let (mut traces, mut steps) = (0, 0);
    for line in reader.lines() {
        let t: Json = serde_json::from_str(&line.unwrap()).unwrap();
        let mut d = Driver::new(&t["limits"]);
        for (step, a) in t["actions"].as_array().unwrap().iter().enumerate() {
            let result = d.event(a);
            let accepted = result.is_ok();
            writeln!(writer,"{}",json!({"id":t["id"],"step":step,"accepted":accepted,"error_kind":result.as_ref().err().map(|e|format!("{:?}",e.kind)),"progress":result.ok(),"projection":d.observe()})).unwrap();
            steps += 1;
            if !accepted
                && !a
                    .get("continue_on_error")
                    .and_then(Json::as_bool)
                    .unwrap_or(false)
            {
                break;
            }
        }
        traces += 1;
    }
    writer.flush().unwrap();
    println!(
        "PUBLIC_CONTRACT_REPLAY {}",
        json!({"traces":traces,"steps":steps})
    );
}
