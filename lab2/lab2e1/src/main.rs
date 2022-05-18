
use clap::Parser;
use lab2e1::find;

#[derive(Parser)]
struct Args {
    /// Value to find
    #[clap(name = "Key")]
    key: i32
}

fn main() {
    let args = Args::parse();

    let mut buffer = String::new();
    let mut vec: Vec<i32> = vec![];

    while std::io::stdin().read_line(&mut buffer).unwrap() > 0 {
        let v = buffer.trim().parse::<i32>();
        if v.is_ok() { vec.push(v.unwrap()); }
        else { panic!("{}", v.unwrap_err()); }
        buffer.clear();
    }
    let res = find(&vec, args.key);
    if res.is_some() {
        println!("{}", res.unwrap() );
    } else {
        println!("Not found");
    }
}