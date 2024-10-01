use clap::Parser;
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
    name: String,
    fields: Vec<String>,
}
impl ClassContents {
    fn new(raw_json: &String) -> Self {
        Self {
            name: String::new(),
            fields: Vec::new(),
        }
    }
    fn get_csharp_class(&self) -> String {
        unimplemented!()
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
