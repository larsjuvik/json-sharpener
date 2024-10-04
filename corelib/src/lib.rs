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

        // Make sure the root value is an object
        let root_object = match properties.as_object() {
            Some(v) => v,
            None => return Err("Root of JSON has to be an object".to_string()),
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
            Value::Null => Ok("object".to_string()),
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
            return Ok("object[]".to_string());
        }

        // Parse first item within array
        let first_elem = match values.iter().nth(0) {
            Some(v) => v,
            None => return Err("Could not parse first element in array".to_string()),
        };
        let first_elem_type = CSharpClass::get_type_from_value(first_elem)?;

        // Check if there are differing types within array
        for v in values {
            let v_type = CSharpClass::get_type_from_value(v)?;
            if v_type != first_elem_type {
                return Err("All types in array must be equal".to_string());
            }
        }

        Ok(format!("{}[]", first_elem_type))
    }
}

#[cfg(test)]
mod tests;
