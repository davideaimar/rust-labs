use std::sync::{Condvar, Mutex};

pub struct CyclicBarrier {
    expected: usize,
    state: Mutex<(usize, bool)>, // (count, filling)
    cv: Condvar
}

impl CyclicBarrier {
    pub fn new(expected: usize) -> Self{
        Self{
            expected,
            state: Mutex::new((0, true)),
            cv: Condvar::new()
        }
    }
    pub fn wait(&self) {

        // Wait if barrier is freeing the old cycle threads
        let mut state = self.state.lock().unwrap();
        while ! (*state).1 {
            state = self.cv.wait(state).unwrap();
        }

        (*state).0 += 1;

        if (*state).0 == self.expected {
            (*state).1=false;
            (*state).0 -= 1;
            self.cv.notify_all();
        } else {
            while (*state).1 {
                state = self.cv.wait(state).unwrap();
            }
            (*state).0 -= 1;
            if (*state).0==0 {
                (*state).1 = true;
                self.cv.notify_all();
            }
        }

    }
}