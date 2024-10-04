use corelib::CSharpClass;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn convert_json_to_csharp(json: &str) -> String {
    let parsed = CSharpClass::from_json(&json.to_string(), "MyClass".to_string());
    match parsed {
        Ok(v) => match v.get_csharp_output() {
            Ok(vv) => vv,
            Err(e) => String::new(),
        },
        Err(e) => String::new(),
    }
}
