use clap::Parser;
use calendar_slot::calendar::Calendar;

#[derive(Parser)]
struct Args {
    /// Path of calendar 1
    #[clap()]
    cal1: String,

    /// Path of calendar 2
    #[clap()]
    cal2: String,

    /// Duration of desired timeslot
    #[clap()]
    duration: u32
}

fn main() {
    let args = Args::parse();
    let calendar1 = Calendar::from_file(args.cal1.as_str()).unwrap();
    let calendar2 = Calendar::from_file(args.cal2.as_str()).unwrap();
    let free_slots = calendar1.free_slots_with(&calendar2, args.duration);
    for slot in free_slots {
        println!("{}", slot);
    }
}
