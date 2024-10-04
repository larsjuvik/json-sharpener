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

#[test]
fn test_correct_null_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "nullValue": null }"#);
    let expected_output = r#"class TestClass
{
    public object? NullValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}

#[test]
fn test_correct_object_array_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "arrayValue": [] }"#);
    let expected_output = r#"class TestClass
{
    public object?[] ArrayValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}

#[test]
fn test_correct_integer_array_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "arrayValue": [1, 2, 3] }"#);
    json_data.push(r#"{ "arrayValue": [-1, -2, -3] }"#);
    json_data.push(r#"{ "arrayValue": [1] }"#);
    let expected_output = r#"class TestClass
{
    public int[] ArrayValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}

#[test]
fn test_correct_long_array_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "arrayValue": [2147483648] }"#); // max int value + 1
    json_data.push(r#"{ "arrayValue": [-2147483649] }"#); // min int value - 1
    json_data.push(r#"{ "arrayValue": [2147483648, -2147483649] }"#);
    json_data.push(r#"{ "arrayValue": [-2147483649, 1, 3] }"#);
    json_data.push(r#"{ "arrayValue": [1, 2, -2147483649] }"#);
    let expected_output = r#"class TestClass
{
    public long[] ArrayValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}

#[test]
fn test_correct_double_array_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "arrayValue": [1.0] }"#);
    json_data.push(r#"{ "arrayValue": [299993.2] }"#);
    json_data.push(r#"{ "arrayValue": [-3648.99] }"#);
    json_data.push(r#"{ "arrayValue": [0.9999999, 0.00000001] }"#);
    json_data.push(r#"{ "arrayValue": [0.0000001] }"#);
    let expected_output = r#"class TestClass
{
    public double[] ArrayValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}

#[test]
fn test_correct_bool_array_property() {
    let mut json_data: Vec<&str> = Vec::new();
    json_data.push(r#"{ "arrayValue": [true, false] }"#);
    json_data.push(r#"{ "arrayValue": [false] }"#);
    json_data.push(r#"{ "arrayValue": [true] }"#);
    let expected_output = r#"class TestClass
{
    public bool[] ArrayValue { get; set; }
}"#;

    bulk_parse_and_verify(json_data, &expected_output);
}

#[test]
#[should_panic]
/// This should panic as we can't mix long and double in array
fn test_array_long_then_double() {
    let json = r#"{ "arrayValue": [2147483648, 1.0] }"#.to_string();
    let parsed = match CSharpClass::from_json(&json, "TestClass".to_string()) {
        Ok(v) => v,
        Err(_) => return, // if we can't parse it, return (triggering error as this function expects panic)
    };
    let _ = parsed.get_csharp_output().unwrap();
}

#[test]
#[should_panic]
/// This should panic as we can't mix double and long in array
fn test_array_double_then_long() {
    let json = r#"{ "arrayValue": [1.0, 2147483648] }"#.to_string();
    let parsed = match CSharpClass::from_json(&json, "TestClass".to_string()) {
        Ok(v) => v,
        Err(_) => return, // if we can't parse it, return (triggering error as this function expects panic)
    };
    let _ = parsed.get_csharp_output().unwrap();
}
