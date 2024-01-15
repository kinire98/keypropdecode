mod implementations;
#[cfg(test)]
mod tests;

use crate::error::*;

/// This struct is the one that gives you the desired behaviour.  
/// It implements the following traits:
/// 1. Display for Props
/// 2. From<u32> for Props
/// 3. From<Props> for u32
/// 4. TryFrom<Pathbuf> for Props
/// 5. TryFrom<&Pathbuf> for Props
/// 6. TryFrom<&str> for Props  
/// (The TryFrom ones won't appear in the documentation because they use Windows specific behaviour and it is put behind a conditional flag because docs.rs only uses a Linux container)
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Default)]
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
    /// Returns true if the element is read_only. Not available on folders.
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.archive(true).unwrap();
    /// props.read_only(true).unwrap();
    ///
    /// assert_eq!(props.is_read_only(), true);
    /// ```
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }
    /// Allows to change the read_only state. Not available on folder.
    pub fn read_only(&mut self, read_only: bool) -> Result<()> {
        match (read_only, self.archive) {
            (true, false) => Err(Error {
                kind: ErrorKind::ConflictingFlags(
                    "The read only flag must be applied to a file".to_string(),
                ),
            }),
            (true, true) => {
                self.read_only = true;
                Ok(())
            }
            (false, _) => {
                self.read_only = false;
                Ok(())
            }
        }
    }
    /// Returns true if the element is hidden.
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.hidden(true);
    ///
    /// assert_eq!(props.is_hidden(), true);
    /// ```
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }
    /// Allows to change the hidden state.
    pub fn hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }
    /// Returns true if the element is used for system purposes.
    /// ```
    /// let mut props = keypropdecode::Props::from(0x4);
    ///
    /// assert_eq!(props.is_system(), true);
    /// ```
    pub fn is_system(&self) -> bool {
        self.system
    }
    fn _system(&mut self, system: bool) {
        self.system = system;
    }
    /// Returns true if the element is a file.
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.archive(true).unwrap();
    ///
    /// assert_eq!(props.is_archive(), true);
    /// ```
    pub fn is_archive(&self) -> bool {
        self.archive
    }
    /// Allows to change the state archive state.
    /// It performs some error checking in order to prevent that a struct can be a directory and an archive at the same time.
    /// ```
    /// use keypropdecode::error::*;
    /// let mut props = keypropdecode::Props::default();
    /// props.archive(true).unwrap();
    /// assert_eq!(props.directory(true), Err( Error { kind: ErrorKind::ConflictingFlags("An element cannot be an archive and a directory at the same time".to_string()) }) );
    /// ```
    pub fn archive(&mut self, archive: bool) -> Result<()> {
        match (archive, self.directory) {
            (true, true) => Err(Error {
                kind: ErrorKind::ConflictingFlags(
                    "An element cannot be an archive and a directory at the same time".to_string(),
                ),
            }),
            (true, false) => {
                self.archive = true;
                Ok(())
            }
            (false, _) => {
                self.archive = false;
                Ok(())
            }
        }
    }
    /// Returns true if the element is a folder.
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.directory(true).unwrap();
    ///
    /// assert_eq!(props.is_directory(), true);
    /// ```
    pub fn is_directory(&self) -> bool {
        self.directory
    }
    /// Allows to change the directory state.
    /// It performs some error checking in order to prevent that a struct can be a directory and an archive at the same time.
    /// ```
    /// use keypropdecode::error::*;
    /// let mut props = keypropdecode::Props::default();
    /// props.directory(true);
    /// assert_eq!(props.archive(true), Err( Error { kind: ErrorKind::ConflictingFlags("An element cannot be an archive and a directory at the same time".to_string()) }) );
    /// ```
    pub fn directory(&mut self, directory: bool) -> Result<()> {
        match (directory, self.archive) {
            (true, true) => Err(Error {
                kind: ErrorKind::ConflictingFlags(
                    "An element cannot be an archive and a directory at the same time".to_string(),
                ),
            }),
            (true, false) => {
                self.directory = true;
                Ok(())
            }
            (false, _) => {
                self.directory = false;
                Ok(())
            }
        }
    }
    /// Returns true if the element represents a physical device in the file system. Reserved for system use
    /// ```
    /// let mut props = keypropdecode::Props::from(0x40);
    ///
    /// assert_eq!(props.is_device(), true);
    /// ```
    pub fn is_device(&self) -> bool {
        self.device
    }
    fn _device(&mut self, device: bool) {
        self.device = device;
    }
    /// Returns true if the element represents a normal file (a file that doesn't have any properties except being a file(check docs for more info)).
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.archive(true).unwrap();
    /// props.normal(true).unwrap();
    ///
    /// assert_eq!(props.is_normal(), true);
    /// ```
    pub fn is_normal(&self) -> bool {
        self.normal
    }
    /// Allows to change the normal state.
    pub fn normal(&mut self, normal: bool) -> Result<()> {
        let mut comparison = Self::default();
        comparison.archive(true).unwrap();
        match (&mut comparison == self, normal) {
            (false, true) => {
                Err(Error { kind: ErrorKind::ConflictingFlags("The normal flag can only be applied to an archive with no properties, such as read only or hidden. If you are sure the element is not a directory and hasn't any properties, check if the normal flag is already applied. In that case, this call to the method shouldn't be neccesary".to_string()) })
            },
            (true, true) => {
                self.normal = true;
                Ok(())
            },
            (_, false) => {
                self.normal = false;
                Ok(())
            }
        }
    }
    /// Returns true if the element represents a temporary file.
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.archive(true).unwrap();
    /// props.temporary(true).unwrap();
    ///
    /// assert_eq!(props.is_temporary(), true);
    /// ```
    pub fn is_temporary(&self) -> bool {
        self.temporary
    }
    /// Allows to change the temporary state
    pub fn temporary(&mut self, temporary: bool) -> Result<()> {
        match (temporary, self.archive) {
            (true, false) => Err(Error {
                kind: ErrorKind::ConflictingFlags(
                    "The temporary flag must be applied to a file".to_string(),
                ),
            }),
            (true, true) => {
                self.temporary = true;
                Ok(())
            }
            (false, _) => {
                self.temporary = false;
                Ok(())
            }
        }
    }
    /// Returns true if the element represents a sparse file (made small for space saving purposes).
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.archive(true).unwrap();
    /// props.sparse(true).unwrap();
    ///
    /// assert_eq!(props.is_sparse(), true);
    /// ```
    pub fn is_sparse(&self) -> bool {
        self.sparse
    }
    /// Allows to change the sparse state.
    pub fn sparse(&mut self, sparse: bool) -> Result<()> {
        match (sparse, self.archive) {
            (true, false) => Err(Error {
                kind: ErrorKind::ConflictingFlags(
                    "The sparse flag must be applied to a file".to_string(),
                ),
            }),
            (true, true) => {
                self.sparse = true;
                Ok(())
            }
            (false, _) => {
                self.sparse = false;
                Ok(())
            }
        }
    }
    /// Returns true if the element represents a reparse point in the file system (e.g. a symbolic link).
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.reparse(true);
    ///
    /// assert_eq!(props.is_reparse(), true);
    /// ```
    pub fn is_reparse(&self) -> bool {
        self.reparse
    }
    /// Allows to change the reparse state.
    pub fn reparse(&mut self, reparse: bool) {
        self.reparse = reparse;
    }
    /// Returns true if the element represents a compressed file
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.compressed(true);
    ///
    /// assert_eq!(props.is_compressed(), true);
    /// ```
    pub fn is_compressed(&self) -> bool {
        self.compressed
    }
    /// Allows to change the compressed state
    pub fn compressed(&mut self, compressed: bool) {
        self.compressed = compressed;
    }
    /// Returns true if the element is not available inmediately. Aplications should not change this value in an arbitrary way.
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.archive(true).unwrap();
    /// props.offline(true).unwrap();
    ///
    /// assert_eq!(props.is_offline(), true);
    /// ```
    pub fn is_offline(&self) -> bool {
        self.offline
    }
    /// Allows to change the offline state
    pub fn offline(&mut self, offline: bool) -> Result<()> {
        match (offline, self.archive) {
            (true, false) => Err(Error {
                kind: ErrorKind::ConflictingFlags(
                    "The offline flag can only be applied to files".to_string(),
                ),
            }),
            (true, true) => {
                self.offline = true;
                Ok(())
            }
            (false, _) => {
                self.offline = false;
                Ok(())
            }
        }
    }
    /// Returns true if the element isn't indexed by the content indexing service
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.not_content_indexed(true);
    ///
    /// assert_eq!(props.is_not_content_indexed(), true);
    /// ```
    pub fn is_not_content_indexed(&self) -> bool {
        self.not_content_indexed
    }
    /// Allows to change the not_content_indexed state
    pub fn not_content_indexed(&mut self, not_content_indexed: bool) {
        self.not_content_indexed = not_content_indexed;
    }
    /// Returns true if the element is encrypted
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.encrypted(true);
    ///
    /// assert_eq!(props.is_encrypted(), true);
    /// ```
    pub fn is_encrypted(&self) -> bool {
        self.encrypted
    }
    /// Allows to change the encrypted state
    pub fn encrypted(&mut self, encrypted: bool) {
        self.encrypted = encrypted;
    }
    /// Returns true if the directory or user data stream is configured with integrity
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.integrity_stream(true);
    ///
    /// assert_eq!(props.is_integrity_stream(), true);
    /// ```
    pub fn is_integrity_stream(&self) -> bool {
        self.integrity_stream
    }
    /// Allows to change the integrity_stream state
    pub fn integrity_stream(&mut self, integrity_stream: bool) {
        self.integrity_stream = integrity_stream;
    }
    /// Returns true if the element is a virtual file. This value is reserver for system use
    /// ```
    /// let mut props = keypropdecode::Props::from(0x10000);
    ///
    /// assert_eq!(props.is_virtual_file(), true);
    /// ```
    pub fn is_virtual_file(&self) -> bool {
        self.virtual_file
    }
    fn _virtual_file(&mut self, virtual_file: bool) {
        self.virtual_file = virtual_file;
    }
    /// Returns true if the user data stream not to be read by the background data integrity scanner (AKA scrubber)
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.no_scrub_data(true);
    ///
    /// assert_eq!(props.is_no_scrub_data(), true);
    /// ```
    pub fn is_no_scrub_data(&self) -> bool {
        self.no_scrub_data
    }
    /// Allows to change the virtual_file state
    pub fn no_scrub_data(&mut self, no_scrub_data: bool) {
        self.no_scrub_data = no_scrub_data;
    }
    /// Returns true if the element has got extended attributes. System internal use only.
    /// ```
    /// let mut props = keypropdecode::Props::from(0x40000);
    ///
    /// assert_eq!(props.is_extended_attributes(), true);
    /// ```
    pub fn is_extended_attributes(&self) -> bool {
        self.extended_attributes
    }
    fn _extended_attributes(&mut self, extended_attributes: bool) {
        self.extended_attributes = extended_attributes;
    }
    /// Returns true if the element is indicated user intent that the file or directory should be kept fully present locally even when not being actively accessed.
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.pinned(true);
    ///
    /// assert_eq!(props.is_pinned(), true);
    /// ```
    pub fn is_pinned(&self) -> bool {
        self.pinned
    }
    /// Allows to change the pinned state
    pub fn pinned(&mut self, pinned: bool) {
        self.pinned = pinned;
    }
    /// Returns true if the element is indicated user intent that the file or directory shouldn't be kept fully present locally except when being actively accessed.
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.unpinned(true);
    ///
    /// assert_eq!(props.is_unpinned(), true);
    /// ```
    pub fn is_unpinned(&self) -> bool {
        self.unpinned
    }
    /// Allows to change the unpinned state
    pub fn unpinned(&mut self, unpinned: bool) {
        self.unpinned = unpinned;
    }
    /// Returns true if the element hasn't got any physical representation on the local system; the item is vitual. Opening the item will be more expensive than normal, e.g., a file in a remote storage
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.recall_on_open(true);
    ///
    /// assert_eq!(props.is_recall_on_open(), true);
    /// ```
    pub fn is_recall_on_open(&self) -> bool {
        self.recall_on_open
    }
    /// Allows to change the recall_on_open state
    pub fn recall_on_open(&mut self, recall_on_open: bool) {
        self.recall_on_open = recall_on_open;
    }
    /// Returns true if the element isn't fully present locally.
    /// ```
    /// let mut props = keypropdecode::Props::default();
    /// props.recall_on_data_access(true);
    ///
    /// assert_eq!(props.is_recall_on_data_access(), true);
    /// ```
    pub fn is_recall_on_data_access(&self) -> bool {
        self.recall_on_data_access
    }
    /// Allows to change the recall_on_data_access state
    pub fn recall_on_data_access(&mut self, recall_on_data_access: bool) {
        self.recall_on_data_access = recall_on_data_access;
    }
}
