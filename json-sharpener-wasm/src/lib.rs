use json_sharpener::CSharpClass;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
/// Returns C#-representation of JSON input as a string.
/// Returns empty string if string input could not be parsed as JSON.
pub fn convert_json_to_csharp(json: &str, class_name: &str) -> String {
    let parsed = CSharpClass::from_json(&json.to_string(), class_name.to_string());
    match parsed {
        Ok(v) => match v.get_csharp_output() {
            Ok(vv) => vv,
            Err(_e) => String::new(),
        },
        Err(_e) => String::new(),
    }
}

#[wasm_bindgen]
/// Tries to convert JSON string to C# string and returns potential error messages.
/// Returns non-empty string with error description if error occured, otherwise an empty string if successful.
pub fn convert_json_to_csharp_error(json: &str) -> String {
    let parsed = CSharpClass::from_json(&json.to_string(), "".to_string());
    match parsed {
        Ok(v) => match v.get_csharp_output() {
            Ok(_vv) => String::new(),
            Err(_e) => _e,
        },
        Err(_e) => _e,
    }
}
