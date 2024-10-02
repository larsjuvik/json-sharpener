use serde_json::{Map, Number, Value};

pub struct ClassContents {
    class_name: String,
    properties: Value,
}
impl ClassContents {
    pub fn new(raw_json: &String, class_name: String) -> Self {
        let parsed_value: Value = ClassContents::get_parsed_properties(raw_json);

        Self {
            class_name,
            properties: parsed_value,
        }
    }
    fn get_parsed_properties(raw_json: &String) -> Value {
        match serde_json::from_str(raw_json) {
            Ok(v) => v,
            Err(e) => panic!("Could not parse file. Reason: {}", e.to_string()),
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
            Value::Null => String::from("dynamic"),
            Value::Bool(_b) => String::from("bool"),
            Value::Number(n) => ClassContents::get_type_from_number_value(n),
            Value::String(_s) => String::from("string"),
            Value::Array(a) => format!("{}[]", ClassContents::get_array_type(a)),
            Value::Object(_o) => String::from("object"),
        }
    }
    fn get_type_from_number_value(value: &Number) -> String {
        if value.is_f64() {
            return String::from("double");
        } else if value.is_i64() {
            return String::from("int");
        }

        panic!(
            "Could not deduct number type for C#. Value: {}",
            value.to_string()
        )
    }
    fn get_array_type(values: &Vec<Value>) -> String {
        if values.iter().count() == 0 {
            return String::from("dynamic");
        }
        // Check if all values in array are the similar s.t. a type can be given
        let first_elem: &Value = values.iter().nth(0).expect("no first element");
        let first_elem_type = ClassContents::get_type_from_value(first_elem);
        if values
            .iter()
            .all(|v| ClassContents::get_type_from_value(v) == first_elem_type)
        {
            return first_elem_type;
        }

        // Otherwise dynamic
        String::from("dynamic")
    }
}

#[cfg(test)]
mod tests;
