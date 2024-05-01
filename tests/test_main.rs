// tests/test_main.rs
use file_reader::read_file;

#[test]
fn test_main() {
    let path = "test.txt";
    let result = read_file(path);
    assert_eq!(result, vec![String::from("hello")]);
}