use std::fs::File;
/*
https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
*/
pub struct Props {
    read_only: bool, // 1 -> bit 1
    hidden: bool, // 2 -> bit 2
    system: bool, // 4 -> bit 3
    directory: bool, // 16 -> bit 5
    archive: bool, // 32 -> bit 6
    device: bool, // 64 -> bit 7 
    normal: bool, // 128 -> bit 8
    temporary: bool, // 256 -> bit 9
    sparse: bool, // 512 -> bit 10
    reparse: bool, // 1024 -> bit 11
    compressed: bool, // 2048 -> bit 12
    offline: bool, // 4096 -> bit 13
    not_content_indexed: bool, // 8192 -> bit 14
    encrypted: bool, // 16384 -> bit 15
    integrity_stream: bool, // 32768 -> bit 16
    virtual_file: bool, //For system reserved use 65536 -> bit 17
    no_scrub_data: bool, // 131072 -> bit 18
    extended_attributes: bool, // 262144 -> bit 19
    pinned: bool, // 524288 -> bit 20
    unpinned: bool, // 1048576 -> bit 21
    recall_on_open: bool, // 262144 -> bit 22
    recall_on_data_access: bool, // 4194304 -> bit 23
    summarized: bool
}
impl Props {
    pub fn new() -> Self {
        Props { read_only: false, hidden: false, system: false, directory: false, archive: false, device: false, normal: false, temporary: false, sparse: false, reparse: false, compressed: false, offline: false, not_content_indexed: false, encrypted: false, integrity_stream: false, virtual_file: false, no_scrub_data: false, extended_attributes: false, pinned: false, unpinned: false, recall_on_open: false, recall_on_data_access: false, summarized: false }
    }
    pub fn from_number(props: u32) -> Self {
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
        Props { read_only,
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
            summarized: false }
    }
    pub fn from_file(file: &File) -> Self {
        Self::from_number(1)
    }
}
   
   
   
   
   



