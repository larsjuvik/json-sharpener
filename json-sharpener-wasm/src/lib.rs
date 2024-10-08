use json_sharpener::CSharpClass;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn convert_json_to_csharp(json: &str) -> String {
    let parsed = CSharpClass::from_json(&json.to_string(), "MyClass".to_string());
    match parsed {
        Ok(v) => match v.get_csharp_output() {
            Ok(vv) => vv,
            Err(_e) => String::new(),
        },
        Err(_e) => String::new(),
    }
}

#[wasm_bindgen]
/// Returns error string if error occured, otherwise an empty string
pub fn convert_json_to_csharp_error(json: &str) -> String {
    let parsed = CSharpClass::from_json(&json.to_string(), "MyClass".to_string());
    match parsed {
        Ok(v) => match v.get_csharp_output() {
            Ok(vv) => String::new(),
            Err(_e) => _e,
        },
        Err(_e) => _e,
    }
}
