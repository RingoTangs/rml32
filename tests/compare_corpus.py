"""Compare Rust JSONL to existing static evidence; never execute reference artifacts.

Run after cargo build:
    python3 tests/compare_corpus.py
Only the newly built rml32-inspect is started. Existing analysis tools are not rerun.
"""
import argparse
import csv
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "docs/compatibility-evidence"


def load_gzip(name):
    with gzip.open(EVIDENCE / name, "rt", encoding="utf-8") as stream:
        return json.load(stream)


def normalize(value):
    if isinstance(value, dict):
        if set(value) == {"bytes_hex"}:
            raw = bytes.fromhex(value["bytes_hex"]).split(b"\0", 1)[0]
            try:
                return raw.decode("utf-8")
            except UnicodeDecodeError:
                return raw.decode("gb18030", errors="replace")
        return {key: normalize(item) for key, item in value.items()}
    if isinstance(value, list):
        return [normalize(item) for item in value]
    return value


def compare(expected, actual, context):
    """All baseline fields must match; extra retained raw fields are permitted."""
    if isinstance(expected, dict):
        if not isinstance(actual, dict):
            raise AssertionError(f"{context}: expected object")
        for key, value in expected.items():
            if key not in actual:
                raise AssertionError(f"{context}: missing {key}")
            compare(value, actual[key], f"{context}.{key}")
    elif isinstance(expected, list):
        if not isinstance(actual, list) or len(expected) != len(actual):
            raise AssertionError(f"{context}: array length differs")
        for i, (a, b) in enumerate(zip(expected, actual)):
            compare(a, b, f"{context}[{i}]")
    elif expected != actual:
        raise AssertionError(f"{context}: {expected!r} != {actual!r}")


def run(binary, report):
    manifest = json.loads((EVIDENCE / "input-manifest.json").read_text())
    originals = [entry for entry in manifest if entry["path"] != "AGENTS.md"]
    for entry in originals:
        digest = hashlib.sha256((ROOT / entry["path"]).read_bytes()).hexdigest()
        if digest != entry["sha256"]:
            raise AssertionError(f"original changed: {entry['path']}")
    members = {(m["package"], m["path"]): m for m in load_gzip("members.json.gz")}
    programs = {p["source"]: p for p in load_gzip("programs.json.gz")}
    package_summary = json.loads((EVIDENCE / "packages-summary.json").read_text())
    packages = {p["package"]: p for p in package_summary["packages"]}
    with (EVIDENCE / "inheritance.csv").open(newline="") as stream:
        inheritance = {(r["child"], r["parent"]): r for r in csv.DictReader(stream)}
    expected_errors = {
        p["source"] for p in package_summary["parse_issues"]
        if p.get("kind") == "member-decompression"
    }
    errors = set()
    checked = {"packages": 0, "members": 0, "programs": 0, "inheritance": 0}
    summary = None
    # stderr remains visible; stdout is streamed, not a giant tracked dump.
    with subprocess.Popen([str(binary), "corpus", "--root", str(ROOT)],
                          stdout=subprocess.PIPE, text=True, encoding="utf-8") as process:
        try:
            for line in process.stdout:
                event = json.loads(line)
                kind, data = event["kind"], normalize(event["data"])
                if kind == "package":
                    baseline = packages.pop(data["package"])
                    compare({key: baseline[key] for key in ("package", "sha256", "header")}, data, data["package"])
                    checked["packages"] += 1
                elif kind == "member":
                    key = (data["package"], data["path"])
                    baseline = members.pop(key)
                    if "decode_error" in baseline:
                        baseline = {k: v for k, v in baseline.items() if k != "decode_error"}
                        if "decode_error" not in data:
                            raise AssertionError(f"{key}: known anomaly silently accepted")
                        if data["decode_error"]["kind"] != "compression":
                            raise AssertionError(f"{key}: wrong error category")
                    compare(baseline, data, str(key))
                    checked["members"] += 1
                elif kind == "program":
                    compare(programs.pop(data["source"]), data, data["source"])
                    checked["programs"] += 1
                elif kind == "inheritance":
                    baseline = inheritance.pop((data["child"], data["parent"]))
                    compare({"status": baseline["status"],
                             "declared_counts": json.loads(baseline["declared_counts"]),
                             "resolved_counts": json.loads(baseline["resolved_counts"]) if baseline["resolved_counts"] else None},
                            data, data["child"])
                    checked["inheritance"] += 1
                elif kind == "error":
                    if data["source"] in errors:
                        raise AssertionError(f"duplicate error: {data['source']}")
                    errors.add(data["source"])
                elif kind == "summary":
                    if summary is not None:
                        raise AssertionError("duplicate summary")
                    summary = data
                else:
                    raise AssertionError(f"unexpected event kind: {kind}")
            status = process.wait()
        except BaseException:
            process.kill()
            process.wait()
            raise
    if packages or members or programs or inheritance:
        raise AssertionError("incomplete Rust output")
    if errors != expected_errors or status != 1:
        raise AssertionError(f"expected only known resource error and exit 1; got {errors}, exit {status}")
    compare({"packages": 12, "members": 9527, "files": 8922, "decoded_files": 8921,
             "programs": 7159, "functions": 101592, "instructions": 6474773,
             "inheritance_matches": 9933, "errors": 1}, summary, "summary")
    for entry in originals:
        if hashlib.sha256((ROOT / entry["path"]).read_bytes()).hexdigest() != entry["sha256"]:
            raise AssertionError(f"original changed during check: {entry['path']}")
    result = {"status": "pass", "checked": checked, "summary": summary,
              "originals_unchanged": len(originals), "known_errors": sorted(errors),
              "target_execution": "none", "scope": "static structure; not execution equivalence"}
    if report:
        report.parent.mkdir(parents=True, exist_ok=True)
        report.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n")
    print(json.dumps(result, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/debug/rml32-inspect")
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()
    try:
        run(args.binary.resolve(), args.report)
    except (AssertionError, KeyError, ValueError) as exc:
        print(f"FAIL: {exc}", file=sys.stderr)
        sys.exit(1)
