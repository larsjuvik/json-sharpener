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
        let parsed_value: Value = serde_json::from_str(raw_json).expect("Could not parse file");

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

fn get_string_value_from_obj_map(string_value: &Map<String, Value>) -> String {
    let mut lines = String::new();

    for (key, value) in string_value {
        let line = format!(
            "public {} {} = {}\n",
            get_type_from_value(&value),
            key,
            get_value_from_type(&value)
        );
        lines.push_str(line.as_str());
    }

    lines
}
fn get_type_from_value(value: &Value) -> String {
    match value {
        Value::Null => String::from("?"),
        Value::Bool(b) => String::from("bool"),
        Value::Number(n) => String::from("number"),
        Value::String(s) => String::from("string"),
        Value::Array(a) => String::from("[]"),
        Value::Object(o) => String::from("object"),
    }
}
fn get_value_from_type(value: &Value) -> String {
    match value {
        Value::Null => String::from("NONE"),
        Value::Bool(b) => b.to_string().to_lowercase(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::Array(a) => String::from("[...]"),
        Value::Object(o) => String::from("{...}"),
    }
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
