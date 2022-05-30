use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, spawn};


pub fn frequency_iter(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    let mut handles: Vec<JoinHandle<HashMap<char, usize>>> = Vec::new();

    let iter = input
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .into_iter();

    let arc = Arc::new(Mutex::new(iter));

    (0..worker_count).into_iter().for_each(|_i| {
        let arc_c = arc.clone();
        handles.push(spawn(move || {
            let mut hm = HashMap::new();
            loop {
                let n = arc_c.lock().unwrap().next();
                match n {
                    Some(s) => {
                        s.to_lowercase().chars().for_each(|c| {
                            if c.is_alphabetic() {
                                *hm.entry(c).or_insert(0) += 1;
                            }
                        });
                    },
                    None => break
                }
            }
            hm
        }));
    });

    let mut result: HashMap<char, usize> = HashMap::new();

    for handle in handles {
        let new = handle.join().unwrap();
        for (key, value) in new {
            *result.entry(key).or_insert(0) += value;
        }
    }

    result
}


pub fn frequency_chunks(input: &[&str], worker_count: usize) -> HashMap<char, usize> {


    let mut handles: Vec<JoinHandle<HashMap<char, usize>>> = Vec::new();

    let iter = input.chunks((input.len() / worker_count) + (if input.len() % worker_count > 0 { 1 } else { 0 }));

    iter.into_iter().for_each(|chunk| {
        let str: Vec<String> = chunk.into_iter().map(|s| s.to_string()).collect();
        handles.push(spawn(move || {
            let mut hm = HashMap::new();
            str.iter().for_each(|s| {
                s.to_lowercase().chars().for_each(|c| {
                    if c.is_alphabetic() {
                        *hm.entry(c).or_insert(0) += 1;
                    }
                });
            });
            hm
        }));
    });

    let mut result: HashMap<char, usize> = HashMap::new();

    for handle in handles {
        let new = handle.join().unwrap();
        for (key, value) in new {
            *result.entry(key).or_insert(0) += value;
        }
    }

    result
}
