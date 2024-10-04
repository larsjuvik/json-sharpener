use serde_json::{Map, Number, Value};

pub struct ClassContents {
    class_name: String,
    properties: Value,
}
impl ClassContents {
    pub fn new(raw_json: &String, class_name: String) -> Result<Self, String> {
        let parsed_value = ClassContents::get_parsed_properties(raw_json);

        match parsed_value {
            Ok(v) => Ok(Self {
                class_name,
                properties: v,
            }),
            Err(e) => Err(e),
        }
    }
    fn get_parsed_properties(raw_json: &String) -> Result<Value, String> {
        match serde_json::from_str(raw_json) {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("Could not parse file. Reason: {}", e.to_string())),
        }
    }
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

        let properties = ClassContents::get_string_value_from_obj_map(root_object)?;
        for prop in properties {
            output.push_str(format!("    {}", prop).as_str());
        }
        output.push_str("}");

        Ok(output)
    }

    fn capitalized(val: &String) -> Result<String, String> {
        let first_char_uppercase = match val.chars().nth(0) {
            Some(v) => v.to_uppercase(),
            None => return Err(format!("Could not find first char of \"{}\"", val)),
        };
        let remaining_chars = val.chars().skip(1).collect::<String>();

        Ok(format!("{}{}", first_char_uppercase, remaining_chars))
    }

    fn get_string_value_from_obj_map(
        string_value: &Map<String, Value>,
    ) -> Result<Vec<String>, String> {
        let mut lines = Vec::new();

        for (variable_name, value) in string_value {
            let variable_type = ClassContents::get_type_from_value(&value)?;
            let variable_name_capitalized = ClassContents::capitalized(variable_name)?;
            let line = format!(
                "public {} {} {{ get; set; }}\n",
                variable_type, variable_name_capitalized
            );
            lines.push(line);
        }

        Ok(lines)
    }
    fn get_type_from_value(value: &Value) -> Result<String, String> {
        match value {
            Value::Null => Ok("object".to_string()),
            Value::Bool(_b) => Ok("bool".to_string()),
            Value::Number(n) => ClassContents::get_type_from_number_value(n),
            Value::String(_s) => Ok("string".to_string()),
            Value::Array(a) => ClassContents::get_array_type(a),
            Value::Object(_o) => Ok("object".to_string()),
        }
    }

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
        let first_elem_type = ClassContents::get_type_from_value(first_elem)?;

        // Check if there are differing types within array
        for v in values {
            let v_type = ClassContents::get_type_from_value(v)?;
            if v_type != first_elem_type {
                return Err("All types in array must be equal".to_string());
            }
        }

        Ok(format!("{}[]", first_elem_type))
    }
}

#[cfg(test)]
mod tests;
