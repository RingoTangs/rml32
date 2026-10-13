//! Static package-local inheritance validation, not runtime object linking.
use crate::{ByteString, ProgramImage, program::Inherit};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ProgramDescriptor {
    pub source: String,
    pub counts: [i16; 3],
    pub inherits: Vec<Inherit>,
}
impl ProgramDescriptor {
    pub fn from_image(source: String, image: &ProgramImage) -> Self {
        Self {
            source,
            counts: [
                image.constitution_count.unwrap_or(0),
                image.max_functions,
                image.max_vars,
            ],
            inherits: image.inherits.clone(),
        }
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct InheritanceCheck {
    pub child: String,
    pub parent: Option<ByteString>,
    pub status: &'static str,
    pub declared_counts: [i16; 3],
    pub resolved_counts: Option<[i16; 3]>,
}
#[derive(Debug, Serialize)]
pub struct ValidationReport {
    pub checks: Vec<InheritanceCheck>,
    pub matches: usize,
    pub errors: usize,
}
pub fn validate_inheritance(programs: &[ProgramDescriptor]) -> ValidationReport {
    let mut by_source = BTreeMap::<&str, Vec<&ProgramDescriptor>>::new();
    for program in programs {
        by_source.entry(&program.source).or_default().push(program);
    }
    let mut report = ValidationReport {
        checks: Vec::new(),
        matches: 0,
        errors: 0,
    };
    for program in programs {
        let package = program
            .source
            .split_once(".pak/")
            .map(|(p, _)| format!("{p}.pak/"));
        for parent in &program.inherits {
            // The shipped object paths are ASCII. Unrepresentable names remain unresolved,
            // never silently replaced by a guessed text encoding.
            let target = parent
                .name
                .as_ref()
                .and_then(|s| std::str::from_utf8(s.c_bytes()).ok())
                .and_then(|s| {
                    package.as_ref().map(|p| {
                        let name = s.trim_start_matches('/');
                        format!(
                            "{p}{}",
                            name.strip_suffix(".c")
                                .map_or_else(|| name.to_owned(), |n| format!("{n}.o"))
                        )
                    })
                });
            let candidates = target.as_deref().and_then(|s| by_source.get(s));
            let resolved = candidates
                .filter(|entries| entries.len() == 1)
                .map(|entries| entries[0].counts);
            let expected = [parent.offsets[3], parent.offsets[4], parent.offsets[5]];
            let status = if candidates.is_some_and(|entries| entries.len() > 1) {
                "ambiguous"
            } else if resolved == Some(expected) {
                "match"
            } else if resolved.is_some() {
                "count-mismatch"
            } else {
                "missing"
            };
            if status == "match" {
                report.matches += 1;
            } else {
                report.errors += 1;
            }
            report.checks.push(InheritanceCheck {
                child: program.source.clone(),
                parent: parent.name.clone(),
                status,
                declared_counts: expected,
                resolved_counts: resolved,
            });
        }
    }
    report.checks.sort_by(|a, b| {
        a.child.cmp(&b.child).then_with(|| {
            a.parent
                .as_ref()
                .map(|p| &p.bytes)
                .cmp(&b.parent.as_ref().map(|p| &p.bytes))
        })
    });
    report
}
