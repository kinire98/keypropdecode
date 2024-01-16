use std::path::PathBuf;

use crate::error::*;
use crate::props::*;

#[test]
fn basic_functionality() {
    let path = PathBuf::from("Cargo.toml");
    let mut file_props = Props::default();
    file_props.change_element_type(ArcDir::Archive(ArchiveProps::default()));
    assert_eq!(Props::try_from(&path).unwrap(), file_props)
}
#[test]
#[should_panic]
fn file_does_not_exist() {
    let non_existent_path = PathBuf::from("doesn't_exists.txt");
    Props::try_from(&non_existent_path).unwrap();
}
#[test]
fn correct_error() {
    let non_existent_path = PathBuf::from("doesn't_exists.txt");
    assert_eq!(
        Props::try_from(&non_existent_path),
        Err(Error {
            kind: ErrorKind::FileNotFound
        })
    )
}
