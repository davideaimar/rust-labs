use minesweeper::annotate;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    rows: u32,

    #[clap(short, long)]
    cols: u32,

    grid: String
}

fn main() {
    let args = Args::parse();

    if args.grid.len() != args.rows as usize * args.cols as usize {
        panic!("Invalid grid size");
    }

    let res1 = annotate(args.grid.as_bytes().chunks(args.cols as usize).map(|chunk| {
        let slice = unsafe {
            std::str::from_utf8_unchecked(chunk)
        };
        slice
    }).collect::<Vec<&str>>().as_slice());

    println!("{:?}", res1);
}