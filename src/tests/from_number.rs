
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
    assert_eq!(Props::from_number(0b10), hidden);
}
#[test]
fn system() {
    let mut system = Props::default();
    system.system(true);
    assert_eq!(Props::from_number(0b100), system);
}
#[test]
#[ignore = "Not yet implemented functionality"]
fn directory() {
    let mut directory = Props::default();
    directory.directory(true);
    assert_eq!(Props::from_number(0b10000), directory);
}
#[test]
#[ignore = "Not yet implemented functionality"]
fn archive() {
    let mut archive = Props::default();
    archive.archive(true);
    assert_eq!(Props::from_number(0b100000), archive);
}
#[test]
fn device() {
    let mut device = Props::default();
    device.device(true);
    assert_eq!(Props::from_number(0b1000000), device);
}
#[test]
fn normal() {
    let mut normal = Props::default();
    normal.normal(true);
    assert_eq!(Props::from_number(0b10000000), normal);
}
#[test]
fn temporary() {
    let mut temporary = Props::default();
    temporary.temporary(true);
    assert_eq!(Props::from_number(0b100000000), temporary);
}
#[test]
fn sparse() {
    let mut sparse = Props::default();
    sparse.sparse(true);
    assert_eq!(Props::from_number(0b1000000000), sparse);
}
#[test]
fn reparse() {
    let mut reparse = Props::default();
    reparse.reparse(true);
    assert_eq!(Props::from_number(0b10000000000), reparse);
}
#[test]
fn compressed() {
    let mut compressed = Props::default();
    compressed.compressed(true);
    assert_eq!(Props::from_number(0b100000000000), compressed);
}
#[test]
fn offline() {
    let mut offline = Props::default();
    offline.offline(true);
    assert_eq!(Props::from_number(0b1000000000000), offline);
}
#[test]
fn not_content_indexed() {
    let mut not_content_indexed= Props::default();
    not_content_indexed.not_content_indexed(true);
    assert_eq!(Props::from_number(0b10000000000000), not_content_indexed);
}
#[test]
fn encrypted() {
    let mut encrypted = Props::default();
    encrypted.encrypted(true);
    assert_eq!(Props::from_number(0b100000000000000), encrypted);
}
#[test]
fn integrity_stream() {
    let mut integrity_stream = Props::default();
    integrity_stream.integrity_stream(true);
    assert_eq!(Props::from_number(0b1000000000000000), integrity_stream);
}
#[test]
fn virtual_file() {
    let mut virtual_file = Props::default();
    virtual_file.virtual_file(true);
    assert_eq!(Props::from_number(0b10000000000000000), virtual_file);
}
#[test]
fn no_scrub_data() {
    let mut no_scrub_data = Props::default();
    no_scrub_data.no_scrub_data(true);
    assert_eq!(Props::from_number(0b100000000000000000), no_scrub_data);
}
#[test]
fn extended_attributes() {
    let mut extended_attributes = Props::default();
    extended_attributes.extended_attributes(true);
    assert_eq!(Props::from_number(0b1000000000000000000), extended_attributes);
}
#[test]
fn pinned() {
    let mut pinned = Props::default();
    pinned.pinned(true);
    assert_eq!(Props::from_number(0b10000000000000000000), pinned);
}
#[test]
fn unpinned() {
    let mut unpinned = Props::default();
    unpinned.unpinned(true);
    assert_eq!(Props::from_number(0b100000000000000000000), unpinned);
}
#[test]
fn recall_on_open() {
    let mut recall_on_open = Props::default();
    recall_on_open.recall_on_open(true);
    assert_eq!(Props::from_number(0b1000000000000000000000), recall_on_open);
}
#[test]
fn recall_on_data_access() {
    let mut recall_on_data_access = Props::default();
    recall_on_data_access.recall_on_data_access(true);
    assert_eq!(Props::from_number(0b10000000000000000000000), recall_on_data_access);
}