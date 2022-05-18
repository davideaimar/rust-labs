use clap::Parser;
use capitalizer::capitalize;

#[derive(Parser)]
struct Args {
    /// String to capitalize
    #[clap(name = "string")]
    s: String
}

fn main() {
    let args = Args::parse();
    let s = capitalize(&args.s);
    println!("{}", s);
}
