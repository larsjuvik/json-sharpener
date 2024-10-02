use super::ClassContents;

const class_name: &str = "TestClass";

#[test]
fn test_correct_bool_property() {
    let json = r#"{ "boolValue": true }"#;
    let expected_output = r#"class TestClass
{
    public bool BoolValue { get; set; }
}"#;

    let parsed_data = ClassContents::new(&json.to_string(), class_name.to_string());
    let output = parsed_data.get_csharp_output();

    assert_eq!(output, expected_output);
}
