use crate::Props;

use std::{fs::Metadata, path::PathBuf};
use std::os::windows::prelude::*;

use crate::error::*;



const READ_ONLY: u32 = 1;
const HIDDEN: u32 = 1 << 1;
const SYSTEM: u32  = 1 << 2;
const DIRECTORY: u32 = 1 << 4;
const ARCHIVE: u32 = 1 << 5;
const DEVICE: u32 = 1 << 6;
const NORMAL: u32 = 1 << 7;
const TEMPORARY: u32 = 1 << 8;
const SPARSE: u32 = 1 << 9;
const REPARSE: u32 = 1 << 10;
const COMPRESSED: u32 = 1 << 11;
const OFFLINE: u32 = 1 << 12; 
const NOT_CONTENT_INDEXED: u32 = 1 << 13;
const ENCRYPTED: u32 = 1 << 14;
const INTEGRITY_STREAM: u32 = 1 << 15;
const VIRTUAL_FILE: u32 = 1 << 16;
const NO_SCRUB_DATA: u32 = 1 << 17; 
const EXTENDED_ATTRIBUTES: u32 = 1 << 18;
const PINNED: u32 = 1 << 19;
const UNPINNED: u32 = 1 << 20;
const RECALL_ON_OPEN: u32 = 1 << 21;
const RECALL_ON_DATA_ACCESS: u32 = 1 << 22;
impl From<u32> for Props {
    fn from(value: u32) -> Self {
        /*
           All of this left/right shift operations because
           Windows gives files properties in integer form
           Here is needed to check if a specific bit is a 1
           I make a copy of the properties, if the number
           after a right shift and then a left one is equal to the
           clone that means there was a 0 there. Otherwise, there was a 1
        */
        let mut props = Self::default();
        if value & READ_ONLY  == READ_ONLY {
            props.read_only = true;
        }
        if value & HIDDEN == HIDDEN {
            props.hidden = true;
        }
        if value & SYSTEM == SYSTEM {
            props.system = true;
        }
        if value & DIRECTORY == DIRECTORY {
            props.directory = true;
        }
        if value & ARCHIVE == ARCHIVE {
            props.archive = true;
        }
        if value & DEVICE == DEVICE {
            props.device = true;
        }
        if value & NORMAL == NORMAL {
            props.normal = true;
        }
        if value & TEMPORARY == TEMPORARY {
            props.temporary = true;
        }
        if value & SPARSE == SPARSE {
            props.sparse = true;
        }
        if value & REPARSE == REPARSE {
            props.reparse = true;
        }
        if value & COMPRESSED == COMPRESSED {
            props.compressed = true;
        }
        if value & OFFLINE == OFFLINE {
            props.offline = true;
        }
        if value & NOT_CONTENT_INDEXED == NOT_CONTENT_INDEXED {
            props.not_content_indexed = true;
        }
        if value & ENCRYPTED == ENCRYPTED {
            props.encrypted = true;
        }
        if  value & INTEGRITY_STREAM == INTEGRITY_STREAM {
            props.integrity_stream = true;
        }
        if value & VIRTUAL_FILE == VIRTUAL_FILE {
            props.virtual_file = true;
        }
        if value & NO_SCRUB_DATA == NO_SCRUB_DATA {
            props.no_scrub_data = true;
        }
        if value & EXTENDED_ATTRIBUTES == EXTENDED_ATTRIBUTES {
            props.extended_attributes = true;
        }
        if value & PINNED == PINNED {
            props.pinned = true;
        }
        if value & UNPINNED == UNPINNED {
            props.unpinned = true;
        }
        if value & RECALL_ON_OPEN == RECALL_ON_OPEN {
            props.recall_on_open = true;
        }
        if value & RECALL_ON_DATA_ACCESS == RECALL_ON_DATA_ACCESS {
            props.recall_on_data_access = true;
        }
        props
    }
}
impl TryFrom<PathBuf> for Props {
    type Error = crate::error::Error;
    fn try_from(value: PathBuf) -> std::prelude::v1::Result<Self, Self::Error> { 
        let metadata: Metadata = match std::fs::metadata(value.clone()) {
            Ok(obtained_metadata) => obtained_metadata,
            Err(_) => {
                return Err(Error {
                    kind: ErrorKind::FileNotFound,
                })
            }
        };
        Ok(Props::from(metadata.file_attributes()))
    }
}
impl TryFrom<&PathBuf> for Props {
    type Error = crate::error::Error;
    fn try_from(value: &PathBuf) -> std::prelude::v1::Result<Self, Self::Error> { 
        let metadata: Metadata = match std::fs::metadata(value.clone()) {
            Ok(obtained_metadata) => obtained_metadata,
            Err(_) => {
                return Err(Error {
                    kind: ErrorKind::FileNotFound,
                })
            }
        };
        Ok(Props::from(metadata.file_attributes()))
    }
}