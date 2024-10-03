use clap::Parser;
use corelib::ClassContents;
use std::fs;
use std::path::Path;
use std::process::exit;

/// Arguments for this program
#[derive(Parser)]
#[command(
    about = "A tool for converting JSON to C# classes",
    long_about = "A tool for converting JSON to C# classes"
)]
struct Args {
    /// Path of the file to parse
    #[arg(short, long)]
    file: String,

    /// Classname of the output class
    #[arg(short, long, default_value_t = String::from("SharpenedClass"))]
    class_name: String,
}

fn main() {
    let args: Args = Args::parse();

    let file_exists = Path::new(&args.file).exists();
    if !file_exists {
        println!("Could not find file.\n> {}", args.file);
        exit(1);
    }

    let file_contents = fs::read_to_string(&args.file).expect("Could not read file");
    println!("> {}", args.file);
    println!("Contents:\n{}", file_contents);

    let parsed_contents = ClassContents::new(&file_contents, args.class_name);
    match parsed_contents {
        Ok(v) => println!("Parsed:\n{}", v.get_csharp_output()),
        Err(e) => println!("Error:\n{}", e),
    }
}
