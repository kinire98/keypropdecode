use crate::Props;

#[test]
fn default() {
    assert_eq!(Props::default().as_number(), 0b0);
}
#[test]
fn read_only() {
    let mut props = Props::default();
    props.read_only(true);
    assert_eq!(props.as_number(), 0b1);
}
#[test]
fn hidden() {
    let mut props = Props::default();
    props.hidden(true);
    assert_eq!(props.as_number(), 1 << 1);
}
#[test]
fn system() {
    let mut props = Props::default();
    props._system(true);
    assert_eq!(props.as_number(), 1 << 2);
}
#[test]
fn directory() {
    let mut props = Props::default();
    props.directory(true).unwrap();
    assert_eq!(props.as_number(), 1 << 4);
}
#[test]
fn archive() {
    let mut props = Props::default();
    props.archive(true).unwrap();
    assert_eq!(props.as_number(), 1 << 5);
}
#[test]
fn device() {
    let mut props = Props::default();
    props._device(true);
    assert_eq!(props.as_number(), 1 << 6);
}
#[test]
fn normal() {
    let mut props = Props::default();
    props.normal(true);
    assert_eq!(props.as_number(), 1 << 7);
}
#[test]
fn temporary() {
    let mut props = Props::default();
    props.temporary(true);
    assert_eq!(props.as_number(), 1 << 8);
}
#[test]
fn sparse() {
    let mut props = Props::default();
    props.sparse(true);
    assert_eq!(props.as_number(), 1 << 9);
}
#[test]
fn reparse() {
    let mut props = Props::default();
    props.reparse(true);
    assert_eq!(props.as_number(), 1 << 10);
}
#[test]
fn compressed() {
    let mut props = Props::default();
    props.compressed(true);
    assert_eq!(props.as_number(), 1 << 11);
}
#[test]
fn offline() {
    let mut props = Props::default();
    props.offline(true);
    assert_eq!(props.as_number(), 1 << 12);
}
#[test]
fn not_content_indexed() {
    let mut props = Props::default();
    props.not_content_indexed(true);
    assert_eq!(props.as_number(), 1 << 13);
}
#[test]
fn encrypted() {
    let mut props = Props::default();
    props.encrypted(true);
    assert_eq!(props.as_number(), 1 << 14);
}
#[test]
fn integrity_stream() {
    let mut props = Props::default();
    props.integrity_stream(true);
    assert_eq!(props.as_number(), 1 << 15);
}
#[test]
fn virtual_file() {
    let mut props = Props::default();
    props._virtual_file(true);
    assert_eq!(props.as_number(), 1 << 16);
}
#[test]
fn no_scrub_data() {
    let mut props = Props::default();
    props.no_scrub_data(true);
    assert_eq!(props.as_number(), 1 << 17);
}
#[test]
fn extended_attributes() {
    let mut props = Props::default();
    props._extended_attributes(true);
    assert_eq!(props.as_number(), 1 << 18);
}
#[test]
fn pinned() {
    let mut props = Props::default();
    props.pinned(true);
    assert_eq!(props.as_number(), 1 << 19);
}
#[test]
fn unpinned() {
    let mut props = Props::default();
    props.unpinned(true);
    assert_eq!(props.as_number(), 1 << 20);
}
#[test]
fn recall_on_open() {
    let mut props = Props::default();
    props.recall_on_open(true);
    assert_eq!(props.as_number(), 1 << 21);
}
#[test]
fn recall_on_data_access() {
    let mut props = Props::default();
    props.recall_on_data_access(true);
    assert_eq!(props.as_number(), 1 << 22);
}
