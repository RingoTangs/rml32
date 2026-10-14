#!/usr/bin/env python3
"""Read existing decoded records; verify report by default, regenerate explicitly."""
import argparse
from collections import Counter, defaultdict
import gzip
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
INPUTS = (
    "docs/compatibility-evidence/programs.json.gz",
    "docs/compatibility-evidence/compatibility-summary.json",
    "docs/evidence/actions.csv",
    "docs/evidence/magic_Linux32-rea.json.gz",
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def prototype(f):
    return {k: f[k] for k in ("name", "index", "offset", "attrib", "return_type",
                              "min_args", "max_args", "arguments") if k in f}


def compute():
    programs = json.loads(gzip.decompress((ROOT / INPUTS[0]).read_bytes()))
    baseline = json.loads((ROOT / INPUTS[1]).read_text())["totals"]
    totals = Counter()
    combinations = Counter()
    combination_samples = defaultdict(list)
    service_combinations = defaultdict(Counter)
    package_combinations = defaultdict(Counter)
    reference_prototypes = []
    services = defaultdict(Counter)
    packages = defaultdict(Counter)
    calls = defaultdict(lambda: {"count": 0, "argc": Counter(), "raw_b": Counter(),
                                 "samples": []})
    resolved = {}
    function_shapes = Counter()
    reference_shapes = Counter()
    descriptor_diagnostics = Counter()
    for key in ("functions_with_nonsequential_argument_indices",
                "reference_functions_with_nonsequential_argument_indices",
                "reference_functions_with_any_unknown_flag_bits",
                "reference_functions_with_any_unknown_attrib_bits"):
        descriptor_diagnostics[key] = 0
    call_service = defaultdict(Counter)
    call_package = defaultdict(Counter)
    calls["os_function_0x09"]

    def group_add(service, package, key, n=1):
        services[service][key] += n
        packages[package][key] += n

    for p in programs:
        source = p["source"]
        service, package = source.split("/", 1)[0], source.split(".pak/", 1)[0] + ".pak"
        totals["programs"] += 1
        group_add(service, package, "programs")
        if any(a["flag"] & 1 for f in p["functions"] + p["hidden"] for a in f["arguments"]):
            totals["reference_programs"] += 1
            group_add(service, package, "reference_programs")
        by_index = defaultdict(list)
        for f in p["functions"]:
            by_index[f.get("index")].append(f)
        segments = [("function", f, f["code"]) for f in p["functions"]]
        segments += [("hidden", f, f["code"]) for f in p["hidden"]]
        segments.append(("init", None, p["init"]))
        for kind, f, code in segments:
            totals["segments"] += 1
            totals["instructions"] += len(code["nodes"])
            if f is not None:
                totals["functions" if kind == "function" else "hidden_functions"] += 1
                group_add(service, package, "functions")
                refs = sum(bool(a["flag"] & 1) for a in f["arguments"])
                first_opcode = code["nodes"][0][2] if code["nodes"] else None
                sequential_indices = [a["index"] for a in f["arguments"]] == list(range(len(f["arguments"])))
                descriptor_diagnostics["functions_checked"] += 1
                descriptor_diagnostics["functions_with_sequential_argument_indices" if sequential_indices
                                       else "functions_with_nonsequential_argument_indices"] += 1
                descriptor_diagnostics["functions_starting_check_args_0x0f"] += first_opcode == 15
                descriptor_diagnostics["arguments_with_unknown_flag_bits"] += sum(
                    bool(a["flag"] & ~1) for a in f["arguments"])
                descriptor_diagnostics["arguments_with_unknown_attrib_bits"] += sum(
                    bool(a["attrib"] & ~(2 | 8)) for a in f["arguments"])
                function_shapes[(f["attrib"], f["min_args"], f["max_args"], refs)] += 1
                totals["arguments"] += len(f["arguments"])
                group_add(service, package, "arguments", len(f["arguments"]))
                for a in f["arguments"]:
                    combinations[(a["flag"], a["type"], a["attrib"])] += 1
                    sample_key = (a["flag"], a["type"], a["attrib"])
                    if len(combination_samples[sample_key]) < 3:
                        combination_samples[sample_key].append({
                            "source": source, "member_sha256": p["sha256"],
                            "segment": kind, "prototype": prototype(f), "argument": a})
                    service_combinations[service][(a["flag"], a["type"], a["attrib"])] += 1
                    package_combinations[package][(a["flag"], a["type"], a["attrib"])] += 1
                if refs:
                    reference_shapes[(f["min_args"] == f["max_args"], f["attrib"] & 3,
                                      first_opcode, sequential_indices)] += 1
                    descriptor_diagnostics["reference_functions_starting_check_args_0x0f"] += first_opcode == 15
                    descriptor_diagnostics["reference_functions_with_nonsequential_argument_indices"] += not sequential_indices
                    descriptor_diagnostics["reference_functions_with_any_unknown_flag_bits"] += any(
                        a["flag"] & ~1 for a in f["arguments"])
                    descriptor_diagnostics["reference_functions_with_any_unknown_attrib_bits"] += any(
                        a["attrib"] & ~(2 | 8) for a in f["arguments"])
                    totals["reference_functions"] += 1
                    totals["reference_arguments"] += refs
                    group_add(service, package, "reference_functions")
                    group_add(service, package, "reference_arguments", refs)
                    reference_prototypes.append({"source": source, "service": service,
                                                 "package": package, "segment": kind,
                                                 "member_sha256": p["sha256"],
                                                 "code_offset": code["offset"],
                                                 "node_count": len(code["nodes"]),
                                                 "first_opcode": first_opcode,
                                                 "prototype": prototype(f)})
            for pc, (a, b, op) in enumerate(code["nodes"]):
                if op not in (5, 6, 7, 8, 9):
                    continue
                raw_b = b & 255
                target = None
                if op == 5:
                    status = "dynamic_function_pointer_0x05"
                elif op == 6:
                    status = "builtin_efun_0x06"
                elif op == 8:
                    status = "cross_object_0x08"
                elif op == 9:
                    status = "os_function_0x09"
                elif raw_b & 128:
                    status = "global_0x07_unresolved_runtime_global_program"
                elif p["mode"] != "object":
                    status = "local_0x07_unknown_global_execution_context"
                elif p["constitution_count"] != 1 or p["inherits"]:
                    status = "local_0x07_unknown_constitution_base_offset"
                elif a < 0 or a >= p["max_functions"]:
                    status = "local_0x07_outside_declared_function_range"
                elif len(by_index[a]) != 1:
                    status = ("local_0x07_duplicate_definition" if by_index[a]
                              else "local_0x07_no_serialized_definition")
                else:
                    # No inheritance means constitution base offset zero. Resolve by
                    # unique serialized definition index, never a name or proximity.
                    target = by_index[a][0]
                    status = "local_0x07_resolved_single_constitution"
                sample = {"source": source, "segment": kind,
                          "caller_index": f.get("index") if f else None,
                          "caller_name": f["name"] if f else None,
                          "caller_function_offset": f["offset"] if f else None,
                          "code_offset": code["offset"], "pc": pc,
                          "member_node_offset": code["nodes_offset"] + 4 * pc,
                          "raw_operands": [a, b], "opcode": op}
                item = calls[status]
                item["count"] += 1
                item["raw_b"][str(raw_b)] += 1
                if op == 7:
                    item["argc"][str(raw_b & 63)] += 1
                if len(item["samples"]) < 5:
                    item["samples"].append(sample)
                call_service[service][status] += 1
                call_package[package][status] += 1
                if target is not None:
                    key = (source, target["index"])
                    if key not in resolved:
                        resolved[key] = {"source": source, "member_sha256": p["sha256"],
                                         "prototype": prototype(target), "calls": 0,
                                         "argc": Counter(), "argc_range_relation": Counter(),
                                         "samples": []}
                    entry = resolved[key]
                    argc = raw_b & 63
                    relation = ("below_min" if argc < target["min_args"] else
                                "above_max" if argc > target["max_args"] else "within_bounds")
                    entry["calls"] += 1
                    entry["argc"][str(argc)] += 1
                    entry["argc_range_relation"][relation] += 1
                    if len(entry["samples"]) < 3:
                        entry["samples"].append(sample)

    totals["hidden_functions"] += 0
    for key in ("programs", "functions", "hidden_functions", "instructions"):
        assert totals[key] == baseline[key], (key, totals[key], baseline[key])
    assert totals["programs"] == 7159 and totals["functions"] == 101592
    assert totals["instructions"] == 6474773 and totals["segments"] == 108751
    expected_ops = json.loads((ROOT / INPUTS[1]).read_text())["opcodes"]
    for op in (5, 6, 7, 8, 9):
        actual = sum(c["count"] for name, c in calls.items() if f"0x{op:02x}" in name)
        assert actual == expected_ops.get(hex(op), 0), (op, actual)
    return {
        "schema": "stage2g.reference-argument-statistics.v1",
        "inputs_sha256": {s: digest(ROOT / s) for s in INPUTS},
        "method_script_sha256": digest(Path(__file__)),
        "call_decoding_evidence": "ev_1c856bf5b985ed94a90def645280c43cba5e562297da9520994866c02af54409",
        "totals": totals,
        "parameter_combinations": [dict(flag=k[0], type=k[1], attrib=k[2],
                                         reference=bool(k[0] & 1), nullable=bool(k[2] & 8),
                                         const=bool(k[2] & 2), count=v,
                                         samples=combination_samples[k])
                                   for k, v in sorted(combinations.items())],
        "function_shapes": [dict(attrib=k[0], min_args=k[1], max_args=k[2],
                                  reference_arguments=k[3], count=v)
                            for k, v in sorted(function_shapes.items())],
        "reference_function_shapes": [dict(fixed_arity=k[0], attrib_low2=k[1],
                                            first_opcode=k[2], sequential_argument_indices=k[3], count=v)
                                      for k, v in sorted(reference_shapes.items(),
                                                         key=lambda item: tuple(-1 if x is None else x for x in item[0]))],
        "descriptor_diagnostics": dict(descriptor_diagnostics),
        "by_service": dict(services), "by_package": dict(packages),
        "parameter_combinations_by_service": {
            name: [dict(flag=k[0], type=k[1], attrib=k[2], count=v) for k, v in sorted(c.items())]
            for name, c in service_combinations.items()},
        "parameter_combinations_by_package": {
            name: [dict(flag=k[0], type=k[1], attrib=k[2], count=v) for k, v in sorted(c.items())]
            for name, c in package_combinations.items()},
        "reference_prototypes": reference_prototypes,
        "calls": dict(calls), "call_counts_by_service": dict(call_service),
        "call_counts_by_package": dict(call_package),
        "resolved_local_targets": list(resolved.values()),
        "resolved_local_summary": {
            "targets": len(resolved),
            "targets_with_reference_parameters": sum(
                any(a["flag"] & 1 for a in entry["prototype"]["arguments"])
                for entry in resolved.values()),
            "calls_to_prototypes_with_reference_parameters": sum(
                entry["calls"] for entry in resolved.values()
                if any(a["flag"] & 1 for a in entry["prototype"]["arguments"])),
            "argc_range_relation": {
                key: sum(entry["argc_range_relation"].get(key, 0) for entry in resolved.values())
                for key in ("below_min", "within_bounds", "above_max")},
        },
        "limitations": [
            "Occurrences count serialized nodes, not execution, reachability, or unique source bodies.",
            "Reference flag bit0 is counted; nullable8/const2 are raw attrib masks, not claims of runtime enforcement.",
            "No stack provenance is inferred; resolved call targets do not prove passed arguments are references.",
            "Multi-constitution and global runtime bindings remain unknown, even when names or indices match.",
            "Only opcode07 argc is decoded here; raw b is preserved for other call kinds.",
            "argc range relation is a raw min/max comparison, not a vararg/default argument validity verdict.",
            "Samples are bounded, but every occurrence contributes to exact aggregate counts.",
            "Offsets are within uncompressed PAK members, never ELF addresses or compressed package offsets.",
        ],
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true", help="write only adjacent prototypes.json.gz and summary.json")
    args = parser.parse_args()
    result = compute()
    output = HERE / "prototypes.json.gz"
    encoded = (json.dumps(result, ensure_ascii=False, indent=2, sort_keys=True) + "\n").encode()
    compressed = gzip.compress(encoded, compresslevel=9, mtime=0)
    summary = {key: result[key] for key in (
        "schema", "inputs_sha256", "method_script_sha256", "totals", "by_service",
        "by_package", "resolved_local_summary", "reference_function_shapes",
        "descriptor_diagnostics", "limitations")}
    summary["full_report"] = {"path": output.name,
                              "sha256": hashlib.sha256(compressed).hexdigest(),
                              "compressed_bytes": len(compressed), "uncompressed_bytes": len(encoded)}
    summary["parameter_combinations"] = [
        {k: v for k, v in row.items() if k != "samples"}
        for row in result["parameter_combinations"]]
    summary["calls"] = {name: {k: v for k, v in row.items() if k != "samples"}
                        for name, row in result["calls"].items()}
    summary_output = HERE / "summary.json"
    if args.write:
        output.write_bytes(compressed)
        summary_output.write_text(json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True) + "\n")
    else:
        actual = output.read_bytes()
        assert actual == compressed, "gzip report differs from deterministic recomputed bytes"
        assert json.loads(gzip.decompress(actual)) == result, "report differs from recomputed statistics"
        assert json.loads(summary_output.read_text()) == summary, "summary differs from recomputed statistics"
    print(json.dumps({"verified": not args.write, "totals": result["totals"],
                      "call_counts": {k: v["count"] for k, v in result["calls"].items()}},
                     sort_keys=True))


if __name__ == "__main__":
    main()
