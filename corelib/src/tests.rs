use crate::CSharpClass;

/// Uses input JSON data and expected output to verify that it is correctly parsed
fn bulk_parse_and_verify(json_data: Vec<&str>, expected_output: &str) {
    let mut outputs: Vec<String> = Vec::new();

    for json in json_data {
        let input = json.to_string();
        let class_name = "TestClass".to_string();

        let parsed_data = CSharpClass::from_json(&input, class_name).unwrap();
        let output = parsed_data.get_csharp_output().unwrap();
        outputs.push(output);
    }

    for o in outputs {
        assert_eq!(o, expected_output);
    }
}

#[test]
fn test_correct_bool_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "boolValue": false }"#);
    json_data.push(r#"{ "boolValue": true }"#);
    let expected_output = r#"class TestClass
{
    public bool BoolValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}

#[test]
fn test_correct_string_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "stringValue": "1" }"#);
    json_data.push(r#"{ "stringValue": "" }"#);
    json_data.push(r#"{ "stringValue": "abc" }"#);
    json_data.push(r#"{ "stringValue": "test test" }"#);
    let expected_output = r#"class TestClass
{
    public string StringValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}

#[test]
fn test_correct_integer_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "integerValue": 1 }"#);
    json_data.push(r#"{ "integerValue": -1 }"#);
    json_data.push(r#"{ "integerValue": 0 }"#);
    json_data.push(r#"{ "integerValue": 2147483647 }"#); // max int value
    json_data.push(r#"{ "integerValue": -2147483648 }"#); // min int value
    let expected_output = r#"class TestClass
{
    public int IntegerValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}

#[test]
fn test_correct_long_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "longValue": 2147483648 }"#); // max int value+1
    json_data.push(r#"{ "longValue": -2147483649 }"#); // min int value+1
    let expected_output = r#"class TestClass
{
    public long LongValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}

#[test]
fn test_correct_double_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "doubleValue": 0.0 }"#);
    json_data.push(r#"{ "doubleValue": -1.0 }"#);
    json_data.push(r#"{ "doubleValue": 1.0 }"#);
    let expected_output = r#"class TestClass
{
    public double DoubleValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}
