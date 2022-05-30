use std::sync::{Condvar, Mutex};

enum State<T> {
    Growing(Vec<T>),
    Consuming(Vec<T>),
    Shrinking(usize)
}

impl<T: Clone> State<T> {
    fn is_growing(&self) -> bool {
        match self {
            State::Growing(_) => true,
            _ => false
        }
    }

    fn is_shrinking(&self) -> bool {
        match self {
            State::Shrinking(_) => true,
            _ => false
        }
    }

    fn is_consuming(&self) -> bool {
        match self {
            State::Consuming(_) => true,
            _ => false
        }
    }

    fn get_data(&self) -> &Vec<T> {
        match self {
            State::Growing(data) => data,
            State::Consuming(data) => data,
            State::Shrinking(_) => panic!("Shrinking state")
        }
    }

    fn get_count(&self) -> usize {
        match self {
            State::Growing(c) => (*c).len(),
            State::Consuming(_) => panic!("Cannot get count from consuming state"),
            State::Shrinking(c) => *c
        }
    }
}

pub struct SensorBarrier<T: Clone> {
    expected_count: usize,
    state: Mutex<State<T>>,
    cv: Condvar
}

impl<T: Clone> SensorBarrier<T> {
    pub fn new(expected: usize) -> Self{
        Self{
            expected_count: expected,
            state: Mutex::new(State::Growing(Vec::with_capacity(expected))),
            cv: Condvar::new()
        }
    }

    pub fn produce(&self, value: T) {
        // Wait for the barrier to be ready
        let mut state = self.state.lock().unwrap();
        while ! (*state).is_growing()  {
            state = self.cv.wait(state).unwrap();
        }

        match *state {
            State::Growing(ref mut data) => {
                data.push(value);
                if data.len() == self.expected_count {
                    *state = State::Consuming(data.clone());
                    self.cv.notify_all();
                }
            },
            _ => panic!("Wrong state")
        }

        // wait until all the producers are done and the consumer consumed the data
        while ! (*state).is_shrinking() {
            state = self.cv.wait(state).unwrap();
        }
        *state = State::Shrinking((*state).get_count() - 1);
        if (*state).get_count() == 0 {
            *state = State::Growing(Vec::with_capacity(self.expected_count));
            self.cv.notify_all();
        }
    }

    pub fn consume(&self, fun: fn(&[T])) {
        let mut state = self.state.lock().unwrap();
        while ! (*state).is_consuming() {
            state = self.cv.wait(state).unwrap();
        }
        fun((*state).get_data());
        *state = State::Shrinking(self.expected_count);
        self.cv.notify_all();
    }

}