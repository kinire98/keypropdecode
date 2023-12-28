#[cfg(not(windows))]
compile_error!("This library is only supports Windows!");
#[cfg(test)]
mod tests; 
mod implementations;
mod error;
use error::*;
use std::{os::windows::prelude::*, path::PathBuf, fs::Metadata};
/*
https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
*/
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Props {
    read_only: bool,             // 1 -> bit 1
    hidden: bool,                // 2 -> bit 2
    system: bool,                // 4 -> bit 3
    directory: bool,             // 16 -> bit 5
    archive: bool,               // 32 -> bit 6
    device: bool,                // 64 -> bit 7
    normal: bool,                // 128 -> bit 8
    temporary: bool,             // 256 -> bit 9
    sparse: bool,                // 512 -> bit 10
    reparse: bool,               // 1024 -> bit 11
    compressed: bool,            // 2048 -> bit 12
    offline: bool,               // 4096 -> bit 13
    not_content_indexed: bool,   // 8192 -> bit 14
    encrypted: bool,             // 16384 -> bit 15
    integrity_stream: bool,      // 32768 -> bit 16
    virtual_file: bool,          //For system reserved use 65536 -> bit 17
    no_scrub_data: bool,         // 131072 -> bit 18
    extended_attributes: bool,   // 262144 -> bit 19
    pinned: bool,                // 524288 -> bit 20
    unpinned: bool,              // 1048576 -> bit 21
    recall_on_open: bool,        // 262144 -> bit 22
    recall_on_data_access: bool, // 4194304 -> bit 23
}
impl Props {
    pub fn new() -> Self {
        Props {
            read_only: false,
            hidden: false,
            system: false,
            directory: false,
            archive: false,
            device: false,
            normal: false,
            temporary: false,
            sparse: false,
            reparse: false,
            compressed: false,
            offline: false,
            not_content_indexed: false,
            encrypted: false,
            integrity_stream: false,
            virtual_file: false,
            no_scrub_data: false,
            extended_attributes: false,
            pinned: false,
            unpinned: false,
            recall_on_open: false,
            recall_on_data_access: false,
        }
    }
    pub fn from_number(props: u32) -> Self {
        /*
            All of this left/right shift operations because
            Windows gives files properties in integer form
            Here is needed to check if a specific bit is a 1
            I make a copy of the properties, if the number 
            after a right shift and then a left one is equal to the
            clone that means there was a 0 there. Otherwise, there was a 1
         */
        let mut props = props;
        let mut clone = props;
        let mut read_only = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            read_only = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut hidden = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            hidden = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut system = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            system = true;
        }
        props >>= 2;
        clone >>= 2;
        let mut directory = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            directory = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut archive = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            archive = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut device = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            device = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut normal = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            normal = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut temporary = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            temporary = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut sparse = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            sparse = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut reparse = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            reparse = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut compressed = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            compressed = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut offline = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            offline = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut not_content_indexed = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            not_content_indexed = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut encrypted = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            encrypted = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut integrity_stream = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            integrity_stream = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut virtual_file = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            virtual_file = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut no_scrub_data = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            no_scrub_data = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut extended_attributes = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            extended_attributes = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut pinned = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            pinned = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut unpinned = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            unpinned = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut recall_on_open = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            recall_on_open = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut recall_on_data_access = false;
        props >>= 1;
        props <<= 1;
        if clone != props {
            recall_on_data_access = true;
        }
        Props {
            read_only,
            hidden,
            system,
            directory,
            archive,
            device,
            normal,
            temporary,
            sparse,
            reparse,
            compressed,
            offline,
            not_content_indexed,
            encrypted,
            integrity_stream,
            virtual_file,
            no_scrub_data,
            extended_attributes,
            pinned,
            unpinned,
            recall_on_open,
            recall_on_data_access,
        }
    }
    pub fn from_file(file: &PathBuf) -> Result<Self> {
        let metadata: Metadata ;
        match std::fs::metadata(file.clone()) {
            Ok(obtained_metadata) => metadata = obtained_metadata,
            Err(_) => return Err(Error { kind: ErrorKind::FileNotFound })
        };
        Ok(Self::from_number(metadata.file_attributes()))
    }
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }
    pub fn read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }
    pub fn hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }
    pub fn is_system(&self) -> bool {
        self.system
    }
    pub fn system(&mut self, system: bool) {
        self.system = system;
    }
    pub fn is_archive(&self) -> bool {
        self.archive
    }
    pub fn archive(&mut self, archive: bool) -> Result<()> {
        match (archive, self.directory) {
            (true, true) => Err(Error { kind: ErrorKind::ConflictingFlags }),
            (true, false) => {
                self.archive = true;
                Ok(())
            },
            (false, _) => {
                self.archive = false;
                Ok(())
            }
        }
    }
    pub fn is_directory(&self) -> bool {
        self.read_only
    }
    pub fn directory(&mut self, directory: bool) -> Result<()> {
        match (directory, self.archive) {
            (true, true) => Err(Error { kind: ErrorKind::ConflictingFlags }),
            (true, false) => {
                self.directory = true;
                Ok(())
            },
            (false, _) => {
                self.directory = false;
                Ok(())
            }
        }
    }
    pub fn is_device(&self) -> bool {
        self.device
    }
    pub fn device(&mut self, device: bool) {
        self.device = device;
    }
    pub fn is_normal(&self) -> bool {
        self.normal
    }
    pub fn normal(&mut self, normal: bool) {
        self.normal = normal;
    }
    pub fn is_temporary(&self) -> bool {
        self.temporary
    }
    pub fn temporary(&mut self, temporary: bool) {
        self.temporary = temporary;
    }
    pub fn is_sparse(&self) -> bool {
        self.sparse
    }
    pub fn sparse(&mut self, sparse: bool) {
        self.sparse = sparse;
    }
    pub fn is_reparse(&self) -> bool {
        self.reparse
    }
    pub fn reparse(&mut self, reparse: bool) {
        self.reparse = reparse;
    }
    pub fn is_compressed(&self) -> bool {
        self.compressed
    }
    pub fn compressed(&mut self, compressed: bool) {
        self.compressed = compressed;
    }
    pub fn is_offline(&self) -> bool {
        self.offline
    }
    pub fn offline(&mut self, offline: bool) {
        self.offline = offline;
    }
    pub fn is_not_content_indexed(&self) -> bool {
        self.not_content_indexed
    }
    pub fn not_content_indexed(&mut self, not_content_indexed: bool) {
        self.not_content_indexed = not_content_indexed;
    }
    pub fn is_encrypted(&self) -> bool {
        self.encrypted
    }
    pub fn encrypted(&mut self, encrypted: bool) {
        self.encrypted = encrypted;
    }
    pub fn is_integrity_stream(&self) -> bool {
        self.integrity_stream
    }
    pub fn integrity_stream(&mut self, integrity_stream: bool) {
        self.integrity_stream = integrity_stream;
    }
    pub fn is_virtual_file(&self) -> bool {
        self.virtual_file
    }
    pub fn virtual_file(&mut self, virtual_file: bool) {
        self.virtual_file = virtual_file;
    }
    pub fn is_no_scrub_data(&self) -> bool {
        self.no_scrub_data
    }
    pub fn no_scrub_data(&mut self, no_scrub_data: bool) {
        self.no_scrub_data = no_scrub_data;
    }
    pub fn is_extended_attributes(&self) -> bool {
        self.extended_attributes
    }
    pub fn extended_attributes(&mut self, extended_attributes: bool) {
        self.extended_attributes = extended_attributes;
    }
    pub fn is_pinned(&self) -> bool {
        self.pinned
    }
    pub fn pinned(&mut self, pinned: bool) {
        self.pinned = pinned;
    }
    pub fn is_unpinned(&self) -> bool {
        self.unpinned
    }
    pub fn unpinned(&mut self, unpinned: bool) {
        self.unpinned = unpinned;
    }
    pub fn is_recall_on_open(&self) -> bool {
        self.recall_on_open
    }
    pub fn recall_on_open(&mut self, recall_on_open: bool) {
        self.recall_on_open = recall_on_open;
    }
    pub fn is_recall_on_data_access(&self) -> bool {
        self.recall_on_data_access
    }
    pub fn recall_on_data_access(&mut self, recall_on_data_access: bool) {
        self.recall_on_data_access = recall_on_data_access;
    }
    pub fn as_number(&self) -> u32 {
        let mut result = 0;
        if self.read_only {
            result += 0b1;
        }
        if self.hidden {
            result += 1 << 1;
        }
        if self.system {
            result += 1 << 2;
        }
        if self.directory {
            result += 1 << 4;
        }
        if self.archive {
            result += 1 << 5;
        }
        if self.device {
            result += 1 << 6;
        }
        if self.normal {
            result += 1 << 7;
        }
        if self.temporary {
            result += 1 << 8;
        }
        if self.sparse {
            result += 1 << 9;
        }
        if self.reparse {
            result += 1 << 10;
        }
        if self.compressed {
            result += 1 << 11;
        }
        if self.offline {
            result += 1 << 12;
        }
        if self.not_content_indexed {
            result += 1 << 13;
        }
        if self.encrypted {
            result += 1 << 14;
        }
        if self.integrity_stream {
            result += 1 << 15;
        }
        if self.virtual_file {
            result += 1 << 16;
        }
        if self.no_scrub_data {
            result += 1 << 17;
        }
        if self.extended_attributes {
            result += 1 << 18;
        }
        if self.pinned {
            result += 1 << 19;
        }
        if self.unpinned {
            result += 1 << 20;
        }
        if self.recall_on_open {
            result += 1 << 21;
        }
        if self.recall_on_data_access {
            result += 1 << 22;
        }
        result
    }
}

