use crate::Props;
#[test]
fn test_basic() {
    assert_eq!(Props::from_number(0), Props::default());
}
#[test]
fn read_only() {
    let mut read_only = Props::default();
    read_only.read_only(true);
    assert_eq!(Props::from_number(0b1), read_only);
}
#[test]
fn hidden() {
    let mut hidden = Props::default();
    hidden.hidden(true);
    assert_eq!(Props::from_number(1 << 1), hidden);
}
#[test]
fn system() {
    let mut system = Props::default();
    system._system(true);
    assert_eq!(Props::from_number(1 << 2), system);
}
#[test]
fn directory() {
    let mut directory = Props::default();
    directory.directory(true).unwrap();
    assert_eq!(Props::from_number(1 << 4), directory);
}
#[test]
fn archive() {
    let mut archive = Props::default();
    archive.archive(true).unwrap();
    assert_eq!(Props::from_number(1 << 5), archive);
}
#[test]
fn device() {
    let mut device = Props::default();
    device._device(true);
    assert_eq!(Props::from_number(1 << 6), device);
}
#[test]
fn normal() {
    let mut normal = Props::default();
    normal.normal(true);
    assert_eq!(Props::from_number(1 << 7), normal);
}
#[test]
fn temporary() {
    let mut temporary = Props::default();
    temporary.temporary(true);
    assert_eq!(Props::from_number(1 << 8), temporary);
}
#[test]
fn sparse() {
    let mut sparse = Props::default();
    sparse.sparse(true);
    assert_eq!(Props::from_number(1 << 9), sparse);
}
#[test]
fn reparse() {
    let mut reparse = Props::default();
    reparse.reparse(true);
    assert_eq!(Props::from_number(1 << 10), reparse);
}
#[test]
fn compressed() {
    let mut compressed = Props::default();
    compressed.compressed(true);
    assert_eq!(Props::from_number(1 << 11), compressed);
}
#[test]
fn offline() {
    let mut offline = Props::default();
    offline.offline(true);
    assert_eq!(Props::from_number(1 << 12), offline);
}
#[test]
fn not_content_indexed() {
    let mut not_content_indexed = Props::default();
    not_content_indexed.not_content_indexed(true);
    assert_eq!(Props::from_number(1 << 13), not_content_indexed);
}
#[test]
fn encrypted() {
    let mut encrypted = Props::default();
    encrypted.encrypted(true);
    assert_eq!(Props::from_number(1 << 14), encrypted);
}
#[test]
fn integrity_stream() {
    let mut integrity_stream = Props::default();
    integrity_stream.integrity_stream(true);
    assert_eq!(Props::from_number(1 << 15), integrity_stream);
}
#[test]
fn virtual_file() {
    let mut virtual_file = Props::default();
    virtual_file._virtual_file(true);
    assert_eq!(Props::from_number(1 << 16), virtual_file);
}
#[test]
fn no_scrub_data() {
    let mut no_scrub_data = Props::default();
    no_scrub_data.no_scrub_data(true);
    assert_eq!(Props::from_number(1 << 17), no_scrub_data);
}
#[test]
fn extended_attributes() {
    let mut extended_attributes = Props::default();
    extended_attributes._extended_attributes(true);
    assert_eq!(Props::from_number(1 << 18), extended_attributes);
}
#[test]
fn pinned() {
    let mut pinned = Props::default();
    pinned.pinned(true);
    assert_eq!(Props::from_number(1 << 19), pinned);
}
#[test]
fn unpinned() {
    let mut unpinned = Props::default();
    unpinned.unpinned(true);
    assert_eq!(Props::from_number(1 << 20), unpinned);
}
#[test]
fn recall_on_open() {
    let mut recall_on_open = Props::default();
    recall_on_open.recall_on_open(true);
    assert_eq!(Props::from_number(1 << 21), recall_on_open);
}
#[test]
fn recall_on_data_access() {
    let mut recall_on_data_access = Props::default();
    recall_on_data_access.recall_on_data_access(true);
    assert_eq!(Props::from_number(1 << 22), recall_on_data_access);
}
