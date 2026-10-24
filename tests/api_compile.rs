//! Compile the real public API and check each rejection's diagnostic, not just status.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn compile(dir: &Path, library: &Path, source: &str) -> std::process::Output {
    fs::write(dir.join("case.rs"), source).unwrap();
    Command::new("rustc")
        .args(["--edition=2024", "--crate-type=lib", "--emit=metadata"])
        .arg(dir.join("case.rs"))
        .arg("--extern")
        .arg(format!("rml32={}", library.display()))
        .arg("-L")
        .arg(format!(
            "dependency={}",
            library.parent().unwrap().display()
        ))
        .arg("--out-dir")
        .arg(dir)
        .output()
        .unwrap()
}
#[test]
fn public_api_positive_and_negative_contracts_compile_against_current_library() {
    let deps = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let dir = std::env::temp_dir().join(format!("rml32-api-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let positive = r#"
        use rml32::*;
        use std::rc::Rc;
        pub fn roots() -> Result<(),VmError> {
            let store=ObjectStore::new()?;
            let root=store.array(&[ValueInput::Int(3)])?;
            let copy=root.retain()?;let snapshot=root.snapshot()?;
            let _frozen=snapshot.clone();let _import=ValueInput::Handle(&copy);Ok(())
        }
        pub fn callback() -> HostCallback { Rc::new(|_,scope,args| {
            let _snapshot=args[0].snapshot().unwrap();
            Ok(HostResult::Owned(scope.retain(&args[0]).unwrap()))
        }) }
    "#;
    // A positive counterpart selects a current compatible artifact even if old
    // Cargo hashes remain in deps. Every negative uses that exact library.
    let mut libraries: Vec<PathBuf> = fs::read_dir(deps)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("librml32-")
                && p.extension().is_some_and(|e| e == "rlib")
        })
        .collect();
    libraries.sort_by_key(|p| std::cmp::Reverse(fs::metadata(p).unwrap().modified().unwrap()));
    let library = libraries
        .into_iter()
        .find(|p| compile(&dir, p, positive).status.success())
        .expect("positive contract must compile");
    let cases = [
        (
            "snapshot input",
            r#"use rml32::*;pub fn bad(vm:&mut Vm,id:ObjectId,snapshot:ValueSnapshot) { vm.start(id,0,&[snapshot]); }"#,
            "E0308",
            "ValueInput",
        ),
        (
            "implicit clone",
            r#"pub fn bad(v:&rml32::HostValue)->rml32::HostValue { v.clone() }"#,
            "E0308",
            "does not implement",
        ),
        (
            "forged lease",
            r#"pub fn bad()->rml32::HostValue { rml32::HostValue { id:1 } }"#,
            "private",
            "HostValue",
        ),
        (
            "borrow escape",
            r#"use rml32::*;use std::{rc::Rc,cell::RefCell};pub fn bad()->HostCallback { let saved:Rc<RefCell<Option<BorrowedValue<'static>>>>=Rc::new(RefCell::new(None)); Rc::new(move |_,_,args| { *saved.borrow_mut()=Some(args[0]); Ok(HostResult::Void) }) }"#,
            "E0521",
            "escapes",
        ),
        (
            "scope reentry",
            r#"pub fn bad(scope:&rml32::HostScope<'_>) { scope.run(1); }"#,
            "E0599",
            "run",
        ),
        (
            "raw escape",
            r#"pub fn bad()->rml32::Value { rml32::Value::Int(1) }"#,
            "private",
            "Value",
        ),
    ];
    for (name, source, first, second) in cases {
        let output = compile(&dir, &library, source);
        let diagnostic = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{name} unexpectedly compiled");
        assert!(
            diagnostic.contains(first) && diagnostic.contains(second),
            "{name}: {diagnostic}"
        );
    }
    fs::remove_dir_all(dir).unwrap();
}
