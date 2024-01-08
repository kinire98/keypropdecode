use crate::Props;

use std::{fs::Metadata, path::PathBuf};
use std::os::windows::prelude::*;

use crate::error::*;

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
        let mut props = value;
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