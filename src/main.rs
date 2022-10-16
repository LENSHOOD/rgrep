use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct GrepArgs {
    // pattern for string to be matched
    #[arg(short, long)]
    pattern: String,

    // file to be grep
    #[arg(short, long)]
    file: String
}

fn main() {
    let args = GrepArgs::parse();
    println!("rgrep args: pattern: {}, file: {}", args.pattern, args.file);
}
