use crate::Props;

#[test]
fn default() { 
    let value: u32 = Props::default().into();
    assert_eq!(value, 0b0);
}
#[test]
fn read_only() {
    let mut props = Props::default();
    props.read_only(true);
    let value: u32 = props.into();
    assert_eq!(value, 0b1);
}
#[test]
fn hidden() {
    let mut props = Props::default();
    props.hidden(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 1);
}
#[test]
fn system() {
    let mut props = Props::default();
    props._system(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 2);
}
#[test]
fn directory() {
    let mut props = Props::default();
    props.directory(true).unwrap();
    let value: u32 = props.into();
    assert_eq!(value, 1 << 4);
}
#[test]
fn archive() {
    let mut props = Props::default();
    props.archive(true).unwrap();
    let value: u32 = props.into();
    assert_eq!(value, 1 << 5);
}
#[test]
fn device() {
    let mut props = Props::default();
    props._device(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 6);
}
#[test]
fn normal() {
    let mut props = Props::default();
    props.normal(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 7);
}
#[test]
fn temporary() {
    let mut props = Props::default();
    props.temporary(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 8);
}
#[test]
fn sparse() {
    let mut props = Props::default();
    props.sparse(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 9);
}
#[test]
fn reparse() {
    let mut props = Props::default();
    props.reparse(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 10);
}
#[test]
fn compressed() {
    let mut props = Props::default();
    props.compressed(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 11);
}
#[test]
fn offline() {
    let mut props = Props::default();
    props.offline(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 12);
}
#[test]
fn not_content_indexed() {
    let mut props = Props::default();
    props.not_content_indexed(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 13);
}
#[test]
fn encrypted() {
    let mut props = Props::default();
    props.encrypted(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 14);
}
#[test]
fn integrity_stream() {
    let mut props = Props::default();
    props.integrity_stream(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 15);
}
#[test]
fn virtual_file() {
    let mut props = Props::default();
    props._virtual_file(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 16);
}
#[test]
fn no_scrub_data() {
    let mut props = Props::default();
    props.no_scrub_data(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 17);
}
#[test]
fn extended_attributes() {
    let mut props = Props::default();
    props._extended_attributes(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 18);
}
#[test]
fn pinned() {
    let mut props = Props::default();
    props.pinned(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 19);
}
#[test]
fn unpinned() {
    let mut props = Props::default();
    props.unpinned(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 20);
}
#[test]
fn recall_on_open() {
    let mut props = Props::default();
    props.recall_on_open(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 21);
}
#[test]
fn recall_on_data_access() {
    let mut props = Props::default();
    props.recall_on_data_access(true);
    let value: u32 = props.into();
    assert_eq!(value, 1 << 22);
}
