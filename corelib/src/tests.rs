use super::ClassContents;

const CLASS_NAME: &str = "TestClass";

#[test]
fn test_correct_bool_property() {
    let json = [r#"{ "boolValue": false }"#, r#"{ "boolValue": true }"#];
    let expected_output = r#"class TestClass
{
    public bool BoolValue { get; set; }
}"#;

    let mut outputs: Vec<String> = Vec::new();
    for j in json {
        let parsed_data = ClassContents::new(&j.to_string(), CLASS_NAME.to_string());
        let output = parsed_data.get_csharp_output();
        outputs.push(output);
    }

    for o in outputs {
        assert_eq!(o, expected_output);
    }
}

#[test]
fn test_correct_string_property() {
    let json = [
        r#"{ "stringValue": "1" }"#,
        r#"{ "stringValue": "" }"#,
        r#"{ "stringValue": "abc" }"#,
        r#"{ "stringValue": "test test" }"#,
    ];
    let expected_output = r#"class TestClass
{
    public string StringValue { get; set; }
}"#;

    let mut outputs: Vec<String> = Vec::new();
    for j in json {
        let parsed_data = ClassContents::new(&j.to_string(), CLASS_NAME.to_string());
        let output = parsed_data.get_csharp_output();
        outputs.push(output);
    }

    for o in outputs {
        assert_eq!(o, expected_output);
    }
}

#[test]
fn test_correct_integer_property() {
    let json = [
        r#"{ "integerValue": 1 }"#,
        r#"{ "integerValue": -1 }"#,
        r#"{ "integerValue": 0 }"#,
        r#"{ "integerValue": 2147483647 }"#,  // max int value
        r#"{ "integerValue": -2147483648 }"#, // min int value
    ];
    let expected_output = r#"class TestClass
{
    public int IntegerValue { get; set; }
}"#;

    let mut outputs: Vec<String> = Vec::new();
    for j in json {
        let parsed_data = ClassContents::new(&j.to_string(), CLASS_NAME.to_string());
        let output = parsed_data.get_csharp_output();
        outputs.push(output);
    }

    for o in outputs {
        assert_eq!(o, expected_output);
    }
}

#[test]
fn test_correct_long_property() {
    let json = [
        r#"{ "longValue": 2147483648 }"#,  // max int value+1
        r#"{ "longValue": -2147483649 }"#, // min int value+1
    ];
    let expected_output = r#"class TestClass
{
    public long LongValue { get; set; }
}"#;

    let mut outputs: Vec<String> = Vec::new();
    for j in json {
        let parsed_data = ClassContents::new(&j.to_string(), CLASS_NAME.to_string());
        let output = parsed_data.get_csharp_output();
        outputs.push(output);
    }

    for o in outputs {
        assert_eq!(o, expected_output);
    }
}

#[test]
fn test_correct_double_property() {
    let json = [
        r#"{ "doubleValue": 0.0 }"#,
        r#"{ "doubleValue": -1.0 }"#,
        r#"{ "doubleValue": 1.0 }"#,
    ];
    let expected_output = r#"class TestClass
{
    public double DoubleValue { get; set; }
}"#;

    let mut outputs: Vec<String> = Vec::new();
    for j in json {
        let parsed_data = ClassContents::new(&j.to_string(), CLASS_NAME.to_string());
        let output = parsed_data.get_csharp_output();
        outputs.push(output);
    }

    for o in outputs {
        assert_eq!(o, expected_output);
    }
}
