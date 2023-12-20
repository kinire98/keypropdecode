use std::fs::File;

pub struct Props {
    read_only: bool,
    hidden: bool,
    system: bool,
    directory: bool,
    archive: bool,
    device: bool,
    normal: bool,
    temporary: bool,
    sparse: bool,
    reparse: bool,
    compressed: bool,
    offline: bool,
    not_content_indexed: bool,
    encrypted: bool,
    integrity_stream: bool,
    virtual_file: bool, //For system reserved use
    extended_attributes: bool,
    pinned: bool,
    unpinned: bool,
    recall_on_open: bool,
    recall_on_data_access: bool,
    summarized: bool
}
impl Props {
    pub fn from_number(props: u32) -> Self {
        Props { read_only: false, hidden: false, system: false, directory: false, archive: false, device: false, normal: false, temporary: false, sparse: false, reparse: false, compressed: false, offline: false, not_content_indexed: false, encrypted: false, integrity_stream: false, virtual_file: false, extended_attributes: false, pinned: false, unpinned: false, recall_on_open: false, recall_on_data_access: false, summarized: false }
    }
    pub fn from_file(file: &File) -> Self {

        Props { read_only: false, hidden: false, system: false, directory: false, archive: false, device: false, normal: false, temporary: false, sparse: false, reparse: false, compressed: false, offline: false, not_content_indexed: false, encrypted: false, integrity_stream: false, virtual_file: false, extended_attributes: false, pinned: false, unpinned: false, recall_on_open: false, recall_on_data_access: false, summarized: false }
    }
}



