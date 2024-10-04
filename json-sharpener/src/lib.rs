use serde_json::{Map, Number, Value};

pub struct CSharpClass {
    class_name: String,
    properties: Value,
}
impl CSharpClass {
    /// Attempts to parse JSON and create a [ClassContents] struct
    pub fn from_json(json: &String, class_name: String) -> Result<Self, String> {
        let parsed_value = CSharpClass::get_parsed_properties(json);

        match parsed_value {
            Ok(v) => Ok(Self {
                class_name,
                properties: v,
            }),
            Err(e) => Err(e),
        }
    }

    /// Attempts to parse raw json to a [Value]
    fn get_parsed_properties(raw_json: &String) -> Result<Value, String> {
        match serde_json::from_str(raw_json) {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("Could not parse file. Reason: {}", e.to_string())),
        }
    }

    /// Produces a C# class output ready to use
    pub fn get_csharp_output(&self) -> Result<String, String> {
        let properties = &self.properties;

        // Check if the root value is just an array
        if properties.is_array() {
            return CSharpClass::get_array_type(properties.as_array().unwrap());
        }

        // Check if the root value is just a type
        if !properties.is_array() && !properties.is_object() {
            return CSharpClass::get_type_from_value(properties);
        }

        // Make sure the root value is an object
        let root_object = match properties.as_object() {
            Some(v) => v,
            None => return Err("Could not parse JSON to object".to_string()),
        };

        let mut output = String::new();
        let class_decleration: String = format!("class {}\n{{\n", self.class_name);
        output.push_str(class_decleration.as_str());

        let properties = CSharpClass::get_csharp_lines(root_object)?;
        for prop in properties {
            output.push_str(format!("    {}", prop).as_str());
        }
        output.push_str("}");

        Ok(output)
    }

    /// Creates capitalized [String]
    fn capitalized(val: &String) -> Result<String, String> {
        let first_char_uppercase = match val.chars().nth(0) {
            Some(v) => v.to_uppercase(),
            None => return Err(format!("Could not find first char of \"{}\"", val)),
        };
        let remaining_chars = val.chars().skip(1).collect::<String>();

        Ok(format!("{}{}", first_char_uppercase, remaining_chars))
    }

    /// Gets a list of csharp lines without indentation from map of string-values
    fn get_csharp_lines(string_values: &Map<String, Value>) -> Result<Vec<String>, String> {
        let mut lines = Vec::new();

        for (variable_name, value) in string_values {
            let variable_type = CSharpClass::get_type_from_value(&value)?;
            let variable_name_capitalized = CSharpClass::capitalized(variable_name)?;
            let line = format!(
                "public {} {} {{ get; set; }}\n",
                variable_type, variable_name_capitalized
            );
            lines.push(line);
        }

        Ok(lines)
    }

    /// Gets type from [Value]
    fn get_type_from_value(value: &Value) -> Result<String, String> {
        match value {
            Value::Null => Ok("object?".to_string()),
            Value::Bool(_b) => Ok("bool".to_string()),
            Value::Number(n) => CSharpClass::get_type_from_number_value(n),
            Value::String(_s) => Ok("string".to_string()),
            Value::Array(a) => CSharpClass::get_array_type(a),
            Value::Object(_o) => Ok("object".to_string()),
        }
    }

    /// Gets type from [Number]
    fn get_type_from_number_value(value: &Number) -> Result<String, String> {
        if value.is_i64() {
            let val = value
                .as_i64()
                .expect(format!("Expected {} to be integer", value.to_string()).as_str());

            if val < (i32::MIN as i64) || val > (i32::MAX as i64) {
                return Ok("long".to_string());
            } else {
                return Ok("int".to_string());
            }
        } else if value.is_f64() {
            return Ok("double".to_string());
        }

        Err(format!(
            "Could not deduct number type for C#. Value: {}",
            value.to_string()
        ))
    }

    /// Gets type from array with [Value]
    fn get_array_type(values: &Vec<Value>) -> Result<String, String> {
        if values.iter().count() == 0 {
            // Can't infer type
            return Ok("object?[]".to_string());
        }

        // Get first element of array
        let first_elem: &Value = match values.iter().nth(0) {
            Some(v) => v,
            None => return Err("Could not parse first element in array".to_string()),
        };
        let first_elem_type = CSharpClass::get_type_from_value(first_elem)?;

        // Check if all values can be parsed
        let all_types = values.iter().map(|v| CSharpClass::get_type_from_value(v));
        let all_types_can_be_parsed = all_types.clone().map(|v| v.is_ok()).all(|v| v);
        if !all_types_can_be_parsed {
            return Err("Not all values in array can be parsed".to_string());
        }

        // At this point, all types are parsed successfully
        // Now check if all values are equal
        let mut all_equal = true; // assume try until deviation found
        let all_types_unwrapped = all_types.map(|v| v.unwrap());
        for t in all_types_unwrapped.clone() {
            if first_elem_type != t {
                all_equal = false;
            }
        }

        // If all values equal, we know the type
        if all_equal {
            return Ok(format!("{}[]", first_elem_type));
        }

        // At this point, we can parse all types, but they are not equal
        // If all values are not numeric, we get an error (we can't mix non-numeric values with numeric values)
        let is_numeric = |f: String| f == "long" || f == "int" || f == "double";
        let all_types_numeric = all_types_unwrapped.clone().all(|v| is_numeric(v));
        if !all_types_numeric {
            return Err("All types are not numeric, and type can't be deduced".to_string());
        }

        // At this point, all types in array are numeric
        // 1. If any long found, and rest is long / int, return long
        // 2. If a mix of double and long/int is found, return Err
        let long_found = all_types_unwrapped.clone().any(|v| v == "long");
        let int_found = all_types_unwrapped.clone().any(|v: String| v == "int");
        let double_found = all_types_unwrapped.clone().any(|v: String| v == "double");

        if double_found {
            if long_found || int_found {
                // Can't mix double and long/int
                return Err("Can't mix double and long/int in array".to_string());
            }

            return Ok("double[]".to_string());
        } else if long_found {
            return Ok("long[]".to_string());
        } else if int_found {
            return Ok("int[]".to_string());
        } else {
            return Err("Could not deduce numeric type".to_string());
        }
    }
}

#[cfg(test)]
mod tests;
