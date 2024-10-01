use clap::Parser;

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

fn main() {
    let args: Args = Args::parse();

    println!("Sharpening '{}'", args.file);
}
