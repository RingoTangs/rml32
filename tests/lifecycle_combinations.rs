use rml32::*;
include!("lifecycle_combination_support.rs.inc");
fn public_step(vm: &mut Vm, fail: Option<usize>) -> (Result<RunOutcome, VmError>, usize) {
    assert!(fail.is_none());
    (vm.run(1), 0)
}
fn public_probe(_: &ObjectStore) -> Box<dyn Fn() -> serde_json::Value> {
    Box::new(|| serde_json::json!(null))
}
#[test]
#[ignore = "2BK normal-library combinations"]
fn replay_public_combinations() {
    replay_cases(public_step, public_probe);
}
#[test]
fn mapping_success_control_covers_payloads_reuse_and_catch() {
    for payload in ["object", "shell", "nested"] {
        for reuse in [false, true] {
            for caught in [false, true] {
                let t = serde_json::json!({"id":"ordinary","limits":{"max_objects":2,"max_function_heads":2,"max_ordinary_refs_per_object":6,"max_exec_refs_per_object":2,"max_owners_per_header":8},"payload":payload,"reuse":reuse,"iteration":0,"caught":caught});
                let r = mapping_case(&t, public_step);
                assert_eq!(r["result"]["target"], "accepted", "{r}");
            }
        }
    }
}
#[test]
fn close_success_control_covers_both_states_and_drop_orders() {
    for state in ["queued", "active"] {
        for order in [
            ["drop0", "maintain", "close", "drop1"],
            ["maintain", "close", "drop1", "drop0"],
        ] {
            let t = serde_json::json!({"id":"ordinary","limits":{"max_objects":2,"max_function_heads":2,"max_ordinary_refs_per_object":6,"max_exec_refs_per_object":2,"max_owners_per_header":8},"state":state,"budget":1,"actions":order});
            let r = close_case(&t, public_probe);
            assert!(r.get("error").is_none(), "{r}");
        }
    }
}
