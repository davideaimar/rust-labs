
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // println!("{:?}, {}", array, key);
    return if array.len() == 0 || (array.len() == 1 && array[0] != key) {
        None
    } else {
        let middle = array.len() / 2;
        if array[middle] == key {
            return Some(middle);
        }
        if array[middle] < key {
            let f = find(array.get(middle..array.len()).unwrap(), key);
            if f.is_some() {
                Some(middle + f.unwrap())
            } else {
                None
            }
        } else {
            let f = find(array.get(0..middle).unwrap(), key);
            if f.is_some() {
                Some(f.unwrap())
            } else {
                None
            }
        }
    }
}
/*
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut low: i32 = 0;
    let mut high: i32 = array.len() as i32 - 1;
    while low<=high {
        let mid = ((low + high) / 2) as usize;
        // println!("low: {}, high: {}, mid: {}", low, high, mid);
        if array[mid] == key {
            return Some(mid);
        } else if low==high {
            return None
        } else if array[mid] < key {
            low = mid as i32 + 1;
        } else {
            high = mid as i32 - 1;
        }
    }
    None
}*/