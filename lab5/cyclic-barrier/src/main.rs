use std::sync::Arc;
use cyclic_barrier::CyclicBarrier;

fn main() {
    let a_barrier = Arc::new(CyclicBarrier::new(3));

    let mut vt = Vec::new();
    for i in 0..3 {
        let c_barrier = a_barrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                c_barrier.wait();
                println!("after barrier {} {}\n", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}
