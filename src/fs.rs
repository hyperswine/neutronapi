// VFS traits
// std wraps around these functions

use alloc::{string::String, vec::Vec};
use itertools::Itertools;

/// Contains rootfs hierarchy and any mounted fs
/// FS like nefs can be dynamically loaded
pub struct VFS {
    // the first one is always the root and mounted at "/"
    fs: Vec<SupportedFS>,
}

impl VFS {
    pub fn new(fs: Vec<SupportedFS>) -> Self {
        Self { fs }
    }

    /// If all mount points are valid and not overlapping, the vfs hierarchy is valid
    pub fn check_valid(&self) -> bool {
        let res: Vec<&SupportedFS> = self.fs.iter().filter(|p| p.mount_point == "/").collect();
        // there must be exactly 1 root
        if res.len() != 1 {
            return false;
        }

        // there must be no mount points that are the same
        // sort and compare
        let res = self.fs.iter().sorted().collect_vec();
        // if any dupes, return false
        let res = res.iter().dedup().collect_vec();
        if res.len() != 0 {
            return false;
        }

        // there must be no mount points that are weird or invalidly named like "//hi". Maybe just convert to pathbuf first

        true
    }
}

/// Contains metadata and handles to each filesystem loaded on neutron
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SupportedFS {
    mount_point: String,
}

pub trait Readable {
    /// Read entire file to string
    fn read_all(&self) -> String;

    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize, &'static str>;
    fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> Result<(), &'static str>;
}

pub trait Writable {
    /// Rewrite file with new string
    fn rewrite(&mut self, buf: &[u8]);

    fn write_at(&self, buf: &[u8], offset: u64) -> Result<usize, &'static str>;
    fn write_all_at(&self, buf: &[u8], offset: u64) -> Result<(), &'static str>;
}

pub struct NeutronFile {}
