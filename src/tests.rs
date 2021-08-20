use super::*;

#[test]
fn check_calling_or_default() {
    // calling this test should not have a CSV path as its first
    // argument so it should default to assuming that the file
    // exists in the current directory
    let result = get_path_env();
    let default = PathBuf::from_str("./dim.csv").unwrap();
    assert_eq!(default, result);
}
