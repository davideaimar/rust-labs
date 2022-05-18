use clap::Parser;
use luhn::is_valid;

#[derive(Parser)]
struct Args {
    /// Credit card number to check
    #[clap(validator = is_valid_format)]
    number: String
}

fn main() {
    let args = Args::parse();
    if is_valid(args.number.as_str()) {
        println!("{} is valid", args.number);
    } else {
        println!("{} is invalid", args.number);
    }
}

fn is_valid_format(s: &str) -> Result<(), String> {
    if s.len() != 19 {
        return Err("Credit card number must be 16 digits in 4 groups separated by one space.\nExample: 1234 1234 1234 1234".to_string());
    }

    if String::from(s)
        .split(" ")
        .map(|s| s.len()==4 && s.parse::<u32>().is_ok())
        .any(|s| !s) {
        return Err(String::from("Credit card number must be 16 digits in 4 groups separated by one space.\nExample: 1234 1234 1234 1234"));
    }
    return Ok(())
}