use crate::Props;

#[test]
fn default() {
    let props = Props::default();
    assert_eq!(props.to_string(), "------".to_string())
}
#[test]
fn directory() {
    let mut props = Props::default();
    props.directory(true).unwrap();
    assert_eq!(props.to_string(), "d-----".to_string())
}
#[test]
fn archive() {
    let mut props = Props::default();
    props.archive(true).unwrap();
    assert_eq!(props.to_string(), "-a----".to_string())
}
#[test]
fn read_only() {
    let mut props = Props::default();
    props.read_only(true);
    assert_eq!(props.to_string(), "--r---".to_string())
}
#[test]
fn hidden() {
    let mut props = Props::default();
    props.hidden(true);
    assert_eq!(props.to_string(), "---h--".to_string())
}
#[test]
fn system() {
    let mut props = Props::default();
    props.system(true);
    assert_eq!(props.to_string(), "----s-".to_string())
}
#[test]
fn reparse() {
    let mut props = Props::default();
    props.reparse(true);
    assert_eq!(props.to_string(), "-----l".to_string())
}