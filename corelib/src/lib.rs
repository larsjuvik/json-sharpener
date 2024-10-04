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
    pub fn get_csharp_output(&self) -> String {
        let properties = &self.properties;

        // Make sure the root value is an object
        let root_object = match properties.as_object() {
            Some(v) => v,
            None => panic!("Root of JSON has to be an object."),
        };

        let mut output = String::new();
        let class_decleration: String = format!("class {}\n{{\n", self.class_name);
        output.push_str(class_decleration.as_str());

        let properties = ClassContents::get_string_value_from_obj_map(root_object);
        for prop in properties {
            output.push_str(format!("    {}", prop).as_str());
        }
        output.push_str("}");

        output
    }
    fn capitalized(val: &String) -> String {
        let first_char = val.chars().nth(0).expect("no characters").to_uppercase();
        format!(
            "{}{}",
            first_char.collect::<String>(),
            val.chars().skip(1).collect::<String>()
        )
    }
    fn get_string_value_from_obj_map(string_value: &Map<String, Value>) -> Vec<String> {
        let mut lines = Vec::new();

        for (variable_name, value) in string_value {
            let line = format!(
                "public {} {} {{ get; set; }}\n",
                ClassContents::get_type_from_value(&value),
                ClassContents::capitalized(variable_name)
            );
            lines.push(line);
        }

        lines
    }
    fn get_type_from_value(value: &Value) -> String {
        match value {
            Value::Null => String::from("object"),
            Value::Bool(_b) => String::from("bool"),
            Value::Number(n) => ClassContents::get_type_from_number_value(n)
                .expect("Could not convert Number to type")
                .to_string(),
            Value::String(_s) => String::from("string"),
            Value::Array(a) => format!("{}[]", ClassContents::get_array_type(a)),
            Value::Object(_o) => String::from("object"),
        }
    }
    fn get_type_from_number_value(value: &Number) -> Result<&str, String> {
        if value.is_i64() {
            let val = value
                .as_i64()
                .expect(format!("Expected {} to be integer", value.to_string()).as_str());

            if val < (i32::MIN as i64) || val > (i32::MAX as i64) {
                return Ok("long");
            } else {
                return Ok("int");
            }
        } else if value.is_f64() {
            return Ok("double");
        }

        Err(format!(
            "Could not deduct number type for C#. Value: {}",
            value.to_string()
        ))
    }
    fn get_array_type(values: &Vec<Value>) -> Result<&str, String> {
        if values.iter().count() == 0 {
            // Can't infer type
            return Ok("object");
        }

        // Check if all values in array are the similar s.t. a type can be given
        let first_elem = values.iter().nth(0).unwrap();
        let first_elem_type = ClassContents::get_type_from_value(first_elem);
        if values
            .iter()
            .all(|v| ClassContents::get_type_from_value(v) == first_elem_type)
        {
            return Ok(first_elem_type.as_str());
        }

        Ok("object")
    }
}

#[cfg(test)]
mod tests;
