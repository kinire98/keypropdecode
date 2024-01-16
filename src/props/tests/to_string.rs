use crate::props::*;

#[test]
fn default_directory() {
    let props = Props::default();
    assert_eq!(props.to_string(), "d-----".to_string())
}
#[test]
fn directory() {
    let mut props = Props::default();
    props.change_element_type(ArcDir::Directory);
    assert_eq!(props.to_string(), "d-----".to_string())
}
#[test]
fn archive() {
    let mut props = Props::default();
    props.change_element_type(ArcDir::Archive(ArchiveProps::default()));
    assert_eq!(props.to_string(), "-a----".to_string())
}
#[test]
fn read_only() {
    let mut props = Props::default();
    props.change_element_type(ArcDir::Archive(ArchiveProps::default()));
    props.read_only(true).unwrap();
    println!("{:?}", props.is_read_only());
    assert_eq!(props.to_string(), "-ar---".to_string())
}
#[test]
fn hidden() {
    let mut props = Props::default();
    props.hidden(true);
    assert_eq!(props.to_string(), "d--h--".to_string())
}
#[test]
fn system() {
    let props = Props::from(0x4);
    assert_eq!(props.to_string(), "d---s-".to_string())
}
#[test]
fn reparse() {
    let mut props = Props::default();
    props.reparse(true);
    assert_eq!(props.to_string(), "d----l".to_string())
}
