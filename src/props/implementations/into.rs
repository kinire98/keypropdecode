use crate::Props;




impl Into<u32> for Props {
    fn into(self) -> u32 {
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