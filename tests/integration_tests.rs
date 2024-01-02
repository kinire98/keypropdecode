use keypropdecode::Props;
#[test]
fn read_only_and_hidden_archive_from_number() {
    let mut props = Props::default();
    props.archive(true).unwrap();
    props.read_only(true);
    props.hidden(true);
    assert_eq!(props, Props::from_number(0b100011))
}
#[test]
fn hidden_directory_string() {
    let mut props = Props::default();
    props.directory(true).unwrap();
    props.hidden(true);
    assert_eq!(props.to_string(), "d--h--".to_string())
}
#[test]
fn recall_on_data_access_and_recall_on_open() {
    let mut props = Props::default();
    props.recall_on_data_access(true);
    props.recall_on_open(true);
    assert_eq!(props, Props::from_number((1 << 22) + (1 << 21)));
}
#[test]
fn get_most_common_attributes() {
    let props = Props::from_number(0b10000110111);
    assert_eq!(props.is_read_only(), true);
    assert_eq!(props.is_hidden(), true);
    assert_eq!(props.is_system(), true);
    assert_eq!(props.is_directory(), true);
    assert_eq!(props.is_archive(), true);
    assert_eq!(props.is_reparse(), true);
}
