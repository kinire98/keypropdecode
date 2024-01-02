use std::path::PathBuf;

use crate::error::*;
use crate::Props;

#[test]
fn basic_functionality() {
    let path = PathBuf::from("Cargo.toml");
    let mut file_props = Props::default();
    file_props.archive(true).unwrap();
    assert_eq!(Props::from_file(&path).unwrap(), file_props)
}
#[test]
#[should_panic]
fn file_does_not_exist() {
    let non_existent_path = PathBuf::from("doesn't_exists.txt");
    Props::from_file(&non_existent_path).unwrap();
}
#[test]
fn correct_error() {
    let non_existent_path = PathBuf::from("doesn't_exists.txt");
    assert_eq!(
        Props::from_file(&non_existent_path),
        Err(Error {
            kind: ErrorKind::FileNotFound
        })
    )
}
