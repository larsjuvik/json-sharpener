use clap::Parser;
use serde_json::{Map, Value};
use std::fs;
use std::path::Path;

/// Arguments for this program
#[derive(Parser)]
#[command(
    about = "A tool for converting JSON to C# classes",
    long_about = "A tool for converting JSON to C# classes"
)]
struct Args {
    /// Path of the file to parse
    #[arg(short = 'f', long = "file")]
    file: String,
}

/// Contains the contents this parser can convert
struct ClassContents {
    contents: Value,
}
impl ClassContents {
    fn new(raw_json: &String) -> Self {
        let parsed_value: Value = match serde_json::from_str(raw_json) {
            Ok(v) => v,
            Err(e) => panic!("Could not parse file. Reason: {}", e.to_string()),
        };

        Self {
            contents: parsed_value,
        }
    }
    fn get_csharp_class(&self) -> String {
        let contents = &self.contents;

        // Loop through all contents - we need all fields
        get_string_value_from_obj_map(&contents.as_object().expect("not an object"))
    }
}
fn capitalized(val: &String) -> String {
    let first_char = val.chars().nth(0).expect("no characters").to_uppercase();
    format!(
        "{}{}",
        first_char.collect::<String>(),
        val.chars().skip(1).collect::<String>()
    )
}
fn get_string_value_from_obj_map(string_value: &Map<String, Value>) -> String {
    let mut lines = String::new();

    for (variable_name, value) in string_value {
        let line = format!(
            "public {} {} {{ get; set; }}\n",
            get_type_from_value(&value),
            capitalized(variable_name)
        );
        lines.push_str(line.as_str());
    }

    lines
}
fn get_type_from_value(value: &Value) -> String {
    match value {
        Value::Null => String::from("dynamic"),
        Value::Bool(_b) => String::from("bool"),
        Value::Number(_n) => String::from("number"),
        Value::String(_s) => String::from("string"),
        Value::Array(a) => format!("{}[]", get_array_type(a)),
        Value::Object(_o) => String::from("object"),
    }
}
fn get_array_type(values: &Vec<Value>) -> String {
    if values.iter().count() == 0 {
        return String::from("dynamic");
    }
    // Check if all values in array are the similar s.t. a type can be given
    let first_elem: &Value = values.iter().nth(0).expect("no first element");
    let first_elem_type = get_type_from_value(first_elem);
    if values
        .iter()
        .all(|v| get_type_from_value(v) == first_elem_type)
    {
        return first_elem_type;
    }

    // Otherwise dynamic
    String::from("dynamic")
}

fn main() {
    let args: Args = Args::parse();

    let file_exists = Path::new(&args.file).exists();
    if !file_exists {
        println!("Could not find file.\n> {}", args.file);
    }

    let file_contents = fs::read_to_string(&args.file).expect("Could not read file");
    println!("> {}", args.file);
    println!("Contents:\n{}", file_contents);

    let parsed_contents = ClassContents::new(&file_contents);
    println!("Parsed:\n{}", parsed_contents.get_csharp_class());
}
