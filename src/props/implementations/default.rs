use crate::props::ArchiveProps;

impl Default for ArchiveProps {
    fn default() -> Self {
        ArchiveProps {
            read_only: false,
            normal: true,
            temporary: false,
            sparse: false,
            offline: false,
        }
    }
}
