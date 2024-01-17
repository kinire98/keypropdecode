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
    element_type: ArcDir,
    hidden: bool,                // 2 -> bit 2
    system: bool,                // 4 -> bit 3
    device: bool,                // 64 -> bit 7
    reparse: bool,               // 1024 -> bit 11
    compressed: bool,            // 2048 -> bit 12
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
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Default)]
pub enum ArcDir {
    #[default]
    Directory, // 16 -> bit 5
    Archive(ArchiveProps), // 32 -> bit 6
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Default)]
pub struct ArchiveProps {
    read_only: bool, // 1 -> bit 1
    normal: bool,    // 128 -> bit 8
    temporary: bool, // 256 -> bit 9
    sparse: bool,    // 512 -> bit 10
    offline: bool,   // 4096 -> bit 13
}
impl Props {
    /// Returns true if the element is read_only. Not available on folders.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.change_element_type(ArcDir::Archive(ArchiveProps::default()));
    /// props.read_only(true).unwrap();
    ///
    /// assert_eq!(props.is_read_only(), Ok(true));
    /// ```
    pub fn is_read_only(&self) -> Result<bool> {
        match self.element_type {
            ArcDir::Archive(arc_props) => Ok(arc_props.read_only),
            ArcDir::Directory => Err(Error {
                kind: ErrorKind::NotAFile(
                    "The read_only property is exclusive for files".to_string(),
                ),
            }),
        }
    }
    /// Allows to change the read_only state. Not available on folder.
    pub fn read_only(&mut self, read_only: bool) -> Result<()> {
        match self.element_type {
            ArcDir::Archive(mut arc_props) => {
                arc_props.read_only = read_only;
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
                Ok(())
            }
            ArcDir::Directory => Err(Error {
                kind: ErrorKind::NotAFile(
                    "The read_only property is exclusive for files".to_string(),
                ),
            }),
        }
    }
    /// Returns true if the element is hidden.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.hidden(true);
    ///
    /// assert_eq!(props.is_hidden(), true);
    /// ```
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }
    /// Allows to change the hidden state.
    pub fn hidden(&mut self, hidden: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if hidden {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.hidden = hidden;
    }
    /// Returns true if the element is used for system purposes.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::from(0x4);
    ///
    /// assert_eq!(props.is_system(), true);
    /// ```
    pub fn is_system(&self) -> bool {
        self.system
    }
    fn _system(&mut self, system: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if system {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.system = system;
    }
    /// Returns true if the element is a file.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.change_element_type(ArcDir::Archive(ArchiveProps::default()));
    ///
    /// assert_eq!(props.is_archive(), true);
    /// ```
    pub fn is_archive(&self) -> bool {
        match self.element_type {
            ArcDir::Archive(_) => true,
            ArcDir::Directory => false,
        }
    }
    /// Returns true if the element is a folder.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.change_element_type(ArcDir::Directory);
    ///
    /// assert_eq!(props.is_directory(), true);
    /// ```
    pub fn is_directory(&self) -> bool {
        match self.element_type {
            ArcDir::Directory => true,
            ArcDir::Archive(_) => false,
        }
    }
    /// Allows to change the state archive state.
    /// You give the enum with the properties neccesary, no error checking neccesary.
    pub fn change_element_type(&mut self, element_type: ArcDir) {
        self.element_type = element_type;
    }
    /// Returns true if the element represents a physical device in the file system. Reserved for system use
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::from(0x40);
    ///
    /// assert_eq!(props.is_device(), true);
    /// ```
    pub fn is_device(&self) -> bool {
        self.device
    }
    fn _device(&mut self, device: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if device {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.device = device;
    }
    /// Returns true if the element represents a normal file (a file that doesn't have any properties except being a file(check docs for more info)).
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.change_element_type(ArcDir::Archive(ArchiveProps::default()));
    /// props.normal(true);
    ///
    /// assert_eq!(props.is_normal(), Ok(true));
    /// ```
    pub fn is_normal(&self) -> Result<bool> {
        match self.element_type {
            ArcDir::Archive(arc_props) => Ok(arc_props.normal),
            ArcDir::Directory => Err(Error {
                kind: ErrorKind::NotAFile("The normal property is exclusive for files".to_string()),
            }),
        }
    }
    /// Allows to change the normal state.
    pub fn normal(&mut self, normal: bool) -> Result<()> {
        let mut comp_arc_props = ArchiveProps::default();
        comp_arc_props.normal = true;
        let comp_arc_props_2 = ArchiveProps::default();
        match self.element_type {
            ArcDir::Archive(mut arc_props) => {
                if arc_props == comp_arc_props || arc_props == comp_arc_props_2 {
                    arc_props.normal = normal;
                    self.element_type = ArcDir::Archive(arc_props);
                } else {
                    return Err(Error { kind: ErrorKind::Other("The normal file property must be only applied in a file with only itself set to true".to_string()) });
                }
                Ok(())
            }
            ArcDir::Directory => Err(Error {
                kind: ErrorKind::NotAFile("The normal property is exclusive for files".to_string()),
            }),
        }
    }
    /// Returns true if the element represents a temporary file.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.change_element_type(ArcDir::Archive(ArchiveProps::default()));
    /// props.temporary(true).unwrap();
    ///
    /// assert_eq!(props.is_temporary(), Ok(true));
    /// ```
    pub fn is_temporary(&self) -> Result<bool> {
        match self.element_type {
            ArcDir::Archive(arc_props) => Ok(arc_props.temporary),
            ArcDir::Directory => Err(Error {
                kind: ErrorKind::NotAFile(
                    "The temporary property is exclusive for files".to_string(),
                ),
            }),
        }
    }
    /// Allows to change the temporary state
    pub fn temporary(&mut self, temporary: bool) -> Result<()> {
        match self.element_type {
            ArcDir::Archive(mut arc_props) => {
                arc_props.temporary = temporary;
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
                Ok(())
            }
            ArcDir::Directory => Err(Error {
                kind: ErrorKind::NotAFile(
                    "The temporary property is exclusive for files".to_string(),
                ),
            }),
        }
    }
    /// Returns true if the element represents a sparse file (made small for space saving purposes).
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.change_element_type(ArcDir::Archive(ArchiveProps::default()));
    /// props.sparse(true).unwrap();
    ///
    /// assert_eq!(props.is_sparse(), Ok(true));
    /// ```
    pub fn is_sparse(&self) -> Result<bool> {
        match self.element_type {
            ArcDir::Archive(arc_props) => Ok(arc_props.sparse),
            ArcDir::Directory => Err(Error {
                kind: ErrorKind::NotAFile("The sparse property is exclusive for files".to_string()),
            }),
        }
    }
    /// Allows to change the sparse state.
    pub fn sparse(&mut self, sparse: bool) -> Result<()> {
        match self.element_type {
            ArcDir::Archive(mut arc_props) => {
                arc_props.sparse = sparse;
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
                Ok(())
            }
            ArcDir::Directory => Err(Error {
                kind: ErrorKind::NotAFile("The sparse property is exclusive for files".to_string()),
            }),
        }
    }
    /// Returns true if the element represents a reparse point in the file system (e.g. a symbolic link).
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.reparse(true);
    ///
    /// assert_eq!(props.is_reparse(), true);
    /// ```
    pub fn is_reparse(&self) -> bool {
        self.reparse
    }
    /// Allows to change the reparse state.
    pub fn reparse(&mut self, reparse: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if reparse {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.reparse = reparse;
    }
    /// Returns true if the element represents a compressed file
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.compressed(true);
    ///
    /// assert_eq!(props.is_compressed(), true);
    /// ```
    pub fn is_compressed(&self) -> bool {
        self.compressed
    }
    /// Allows to change the compressed state
    pub fn compressed(&mut self, compressed: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if compressed {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.compressed = compressed;
    }
    /// Returns true if the element is not available inmediately. Aplications should not change this value in an arbitrary way.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.change_element_type(ArcDir::Archive(ArchiveProps::default()));
    /// props.offline(true).unwrap();
    ///
    /// assert_eq!(props.is_offline(), Ok(true));
    /// ```
    pub fn is_offline(&self) -> Result<bool> {
        match self.element_type {
            ArcDir::Archive(arc_props) => Ok(arc_props.offline),
            ArcDir::Directory => Err(Error {
                kind: ErrorKind::NotAFile("The offline property is exclusive to files".to_string()),
            }),
        }
    }
    /// Allows to change the offline state
    pub fn offline(&mut self, offline: bool) -> Result<()> {
        match self.element_type {
            ArcDir::Archive(mut arc_props) => {
                arc_props.offline = offline;
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
                Ok(())
            }
            ArcDir::Directory => Err(Error {
                kind: ErrorKind::NotAFile("The offline property is exclusive to files".to_string()),
            }),
        }
    }
    /// Returns true if the element isn't indexed by the content indexing service
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.not_content_indexed(true);
    ///
    /// assert_eq!(props.is_not_content_indexed(), true);
    /// ```
    pub fn is_not_content_indexed(&self) -> bool {
        self.not_content_indexed
    }
    /// Allows to change the not_content_indexed state
    pub fn not_content_indexed(&mut self, not_content_indexed: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if not_content_indexed {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.not_content_indexed = not_content_indexed;
    }
    /// Returns true if the element is encrypted
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.encrypted(true);
    ///
    /// assert_eq!(props.is_encrypted(), true);
    /// ```
    pub fn is_encrypted(&self) -> bool {
        self.encrypted
    }
    /// Allows to change the encrypted state
    pub fn encrypted(&mut self, encrypted: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if encrypted {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.encrypted = encrypted;
    }
    /// Returns true if the directory or user data stream is configured with integrity
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.integrity_stream(true);
    ///
    /// assert_eq!(props.is_integrity_stream(), true);
    /// ```
    pub fn is_integrity_stream(&self) -> bool {
        self.integrity_stream
    }
    /// Allows to change the integrity_stream state
    pub fn integrity_stream(&mut self, integrity_stream: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if integrity_stream {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.integrity_stream = integrity_stream;
    }
    /// Returns true if the element is a virtual file. This value is reserver for system use
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::from(0x10000);
    ///
    /// assert_eq!(props.is_virtual_file(), true);
    /// ```
    pub fn is_virtual_file(&self) -> bool {
        self.virtual_file
    }
    fn _virtual_file(&mut self, virtual_file: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if virtual_file {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.virtual_file = virtual_file;
    }
    /// Returns true if the user data stream not to be read by the background data integrity scanner (AKA scrubber)
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.no_scrub_data(true);
    ///
    /// assert_eq!(props.is_no_scrub_data(), true);
    /// ```
    pub fn is_no_scrub_data(&self) -> bool {
        self.no_scrub_data
    }
    /// Allows to change the virtual_file state
    pub fn no_scrub_data(&mut self, no_scrub_data: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if no_scrub_data {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.no_scrub_data = no_scrub_data;
    }
    /// Returns true if the element has got extended attributes. System internal use only.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::from(0x40000);
    ///
    /// assert_eq!(props.is_extended_attributes(), true);
    /// ```
    pub fn is_extended_attributes(&self) -> bool {
        self.extended_attributes
    }
    fn _extended_attributes(&mut self, extended_attributes: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if extended_attributes {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.extended_attributes = extended_attributes;
    }
    /// Returns true if the element is indicated user intent that the file or directory should be kept fully present locally even when not being actively accessed.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.pinned(true);
    ///
    /// assert_eq!(props.is_pinned(), true);
    /// ```
    pub fn is_pinned(&self) -> bool {
        self.pinned
    }
    /// Allows to change the pinned state
    pub fn pinned(&mut self, pinned: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if pinned {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.pinned = pinned;
    }
    /// Returns true if the element is indicated user intent that the file or directory shouldn't be kept fully present locally except when being actively accessed.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
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
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.recall_on_open(true);
    ///
    /// assert_eq!(props.is_recall_on_open(), true);
    /// ```
    pub fn is_recall_on_open(&self) -> bool {
        self.recall_on_open
    }
    /// Allows to change the recall_on_open state
    pub fn recall_on_open(&mut self, recall_on_open: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if recall_on_open {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.recall_on_open = recall_on_open;
    }
    /// Returns true if the element isn't fully present locally.
    /// ```
    /// use keypropdecode::props::*;
    /// let mut props = Props::default();
    /// props.recall_on_data_access(true);
    ///
    /// assert_eq!(props.is_recall_on_data_access(), true);
    /// ```
    pub fn is_recall_on_data_access(&self) -> bool {
        self.recall_on_data_access
    }
    /// Allows to change the recall_on_data_access state
    pub fn recall_on_data_access(&mut self, recall_on_data_access: bool) {
        if let ArcDir::Archive(mut arc_props) = self.element_type {
            if recall_on_data_access {
                arc_props.normal = false;
                self.element_type = ArcDir::Archive(arc_props);
            }
        }
        self.recall_on_data_access = recall_on_data_access;
    }
}
