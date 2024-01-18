//! # keypropdecode
//! This crate gives you a handy struct to manage Windows file system elements properties.  
//! The different constants are defined in [Microsoft File Attributes Constants Documentation](https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants).

/// The Error type for the crate
pub mod error;
/// The main module of the library
pub mod props;
/// The re-export of the main Struct of the library
pub use props::Props;
