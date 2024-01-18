//! # keypropdecode
//! A library for decoding windows file system element properties.  
//! Since Windows stores these properties as a number and each individual property is stored in a determined bit of that number, decoding it can bloat the code.  
//! This library attemps to solve this.
//! You can use this crate with different purposes:
//! 1. You can provide an `u32`, with the From trait, and get back a Props instance with the correspondent properties.
//! 2. You can provide a `PathBuf` or a reference to it, with the TryFrom trait, and you won't have
//!    to extract the correspondent properties. You can also provide a valid `&str`.  
//! 3. With the properties correctly set you can get the `u32` correpondent to those properties you
//!    set. The library will ensure you don't set invalid states
//! 4. The `Display` implementation of the struct return a `String` identical as the one that prints with `GetChild-Item` in PowerShell, which are the most commonly used.  
//!
//! For reference with all the file system element properties go to the [Microsoft File Attribute Constants Documentation](https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants).  
//! The implementation of the library uses enums to make invalid states unrepresentable.  
//! It is strongly recommended that **if you don't know what a property does, don't change it**.  
//! ## Example
//! ```Rust
//! use keypropdecode::Props;
//! let mut props = Props::default();
//! props.change_element_type(ArcDir::Archive(ArchiveProps::default()));
//! assert_eq!(Props::try_from(r"hidden_file_example.txt").unwrap(), props);
//! ```

/// The Error type for the crate
pub mod error;
/// The main module of the library
pub mod props;
/// The re-export of the main Struct of the library
pub use props::Props;
