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
fn hidden_system_directory_string() {
    let mut props = Props::default();
    props.directory(true).unwrap();
    props.hidden(true);
    props.system(true);
    assert_eq!(props.to_string(), "d--hs-".to_string())
}