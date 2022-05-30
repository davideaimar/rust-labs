mod lib;

use std::sync::{Arc};
use std::thread;
use std::thread::{sleep};
use std::time::Duration;
use rand::{Rng, thread_rng};
use crate::lib::SensorBarrier;

fn read_value() -> usize {
    let mut rng = rand::thread_rng();
    sleep(Duration::from_secs(rng.gen_range(1..4)));
    rng.gen_range(0..10)
}

fn set_speed(sum: usize) {
    if (sum) > 50 {
        println!("Slowing down...");
    } else {
        println!("Speeding up...");
    }
    sleep(Duration::from_secs(thread_rng().gen_range(1..4)));
    println!("Speed set.");
}

fn main() {

    let barrier = Arc::new(SensorBarrier::new(10));

    (1..=10).into_iter().for_each(|i| {
        let barrier_c = barrier.clone();
        thread::Builder::new()
            .name(format!("Producer {}", i))
            .spawn(move || {
                loop {
                    let value = read_value();
                    println!("  {:2}  | {:2}  ", i, value);
                    barrier_c.produce(value);
                }
            }).unwrap();
    });

    let barrier_c = barrier.clone();

    thread::Builder::new()
        .name("Consumer".to_string())
        .spawn(move || {
            loop {
                barrier_c.consume(|data| {
                    let mut sum = 0;
                    for v in (*data).iter() {
                        sum += *v;
                    }
                    println!("-------------");
                    println!("        {:2}", sum);
                    set_speed(sum);
                    println!("-------------");
                    println!(" prod | value");
                    println!("-------------");
                });
            }
        }).unwrap().join().unwrap();


}
