use super::ClassContents;

const CLASS_NAME: &str = "TestClass";

#[test]
fn test_correct_bool_property() {
    let json = r#"{ "boolValue": true }"#;
    let expected_output = r#"class TestClass
{
    public bool BoolValue { get; set; }
}"#;

    let parsed_data = ClassContents::new(&json.to_string(), CLASS_NAME.to_string());
    let output = parsed_data.get_csharp_output();

    assert_eq!(output, expected_output);
}

#[test]
fn test_correct_string_property() {
    let json = r#"{ "stringValue": "test" }"#;
    let expected_output = r#"class TestClass
{
    public string StringValue { get; set; }
}"#;

    let parsed_data = ClassContents::new(&json.to_string(), CLASS_NAME.to_string());
    let output = parsed_data.get_csharp_output();

    assert_eq!(output, expected_output);
}
