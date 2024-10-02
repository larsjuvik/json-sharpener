use super::*;

#[test]
fn simple_test() {
    let json = String::from("{ \"boolValue\": true }");
    let result = ClassContents::new(&json, "TestClass".to_string());

    let expected = "class TestClass\n{\n    public bool BoolValue { get; set; }\n}";
    assert_eq!(result.get_csharp_output(), String::from(expected));
}
