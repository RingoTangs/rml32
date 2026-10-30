//! A directory-only, byte-preserving model. Never calls the host filesystem.
use super::{ProbeFilesystemEntry, ProbeFilesystemSnapshot, error};
use crate::{ByteString, VmError, VmErrorKind};

pub(super) const MAX_NODES: usize = 16;
pub(super) const MAX_PATH_BYTES: usize = 255;

pub(super) struct MemoryDirectories {
    pub(super) nodes: Vec<(Vec<u8>, bool)>,
    pub(super) max_nodes: usize,
}
impl MemoryDirectories {
    pub(super) fn new() -> Result<Self, VmError> {
        let mut nodes = Vec::new();
        nodes
            .try_reserve_exact(MAX_NODES)
            .map_err(|_| error(VmErrorKind::Limit, "virtual directory allocation failed"))?;
        for path in [b"/".as_slice(), b"/aaa", b"/ccs", b"/csa", b"/dba", b"/gs"] {
            nodes.push((path.to_vec(), true));
        }
        Ok(Self {
            nodes,
            max_nodes: MAX_NODES,
        })
    }
    pub(super) fn normalize(raw: &[u8]) -> Result<Vec<u8>, VmError> {
        let raw = raw.split(|b| *b == 0).next().unwrap_or_default();
        if !raw.starts_with(b"/") {
            return Err(error(
                VmErrorKind::UnsupportedFeature,
                "virtual mkdir requires an absolute path",
            ));
        }
        let mut path = Vec::new();
        path.try_reserve_exact(MAX_PATH_BYTES)
            .map_err(|_| error(VmErrorKind::Limit, "virtual path allocation failed"))?;
        for component in raw.split(|b| *b == b'/').filter(|c| !c.is_empty()) {
            if matches!(component, b"." | b"..") {
                return Err(error(
                    VmErrorKind::UnsupportedFeature,
                    "dot components are outside the virtual mkdir subset",
                ));
            }
            if path
                .len()
                .checked_add(1 + component.len())
                .is_none_or(|n| n > MAX_PATH_BYTES)
            {
                return Err(error(VmErrorKind::Bounds, "virtual path exceeds 255 bytes"));
            }
            path.push(b'/');
            path.extend_from_slice(component);
        }
        if path.is_empty() {
            path.push(b'/');
        }
        // Classifiers see the regular name before the backend removes its trailing
        // separator. Include that separator when checking a volume root itself.
        let mut classified = path.clone();
        classified.push(b'/');
        if classified.starts_with(b"/v/")
            || classified.windows(5).any(|s| s == b".pak/")
            || classified.windows(6).any(|s| s == b".dbfs/")
        {
            return Err(error(
                VmErrorKind::UnsupportedFeature,
                "native volume/mount paths are not supported",
            ));
        }
        Ok(path)
    }
    pub(super) fn mkdir(&mut self, path: &[u8]) -> Result<i32, VmError> {
        if self.nodes.iter().any(|(p, _)| p == path) {
            return Ok(0);
        }
        let split = path.iter().rposition(|b| *b == b'/').unwrap_or(0);
        let parent = if split == 0 {
            b"/".as_slice()
        } else {
            &path[..split]
        };
        if !self.nodes.iter().any(|(p, d)| *d && p == parent) {
            return Ok(0);
        }
        if self.nodes.len() >= self.max_nodes {
            return Err(error(
                VmErrorKind::Limit,
                "virtual directory node limit exceeded",
            ));
        }
        // Reserve and build the node before publishing the directory effect.
        crate::copy_plan::reserve(&mut self.nodes, 1)?;
        let mut owned = Vec::new();
        crate::copy_plan::reserve(&mut owned, path.len())?;
        owned.extend_from_slice(path);
        self.nodes.push((owned, true));
        Ok(1)
    }
    pub(super) fn snapshot(&self) -> ProbeFilesystemSnapshot {
        let mut entries: Vec<_> = self
            .nodes
            .iter()
            .map(|(p, d)| ProbeFilesystemEntry {
                path: ByteString::new(p),
                directory: *d,
            })
            .collect();
        entries.sort_by(|a, b| a.path.bytes.cmp(&b.path.bytes));
        ProbeFilesystemSnapshot { entries }
    }
}
