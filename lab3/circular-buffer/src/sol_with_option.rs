use crate::Error;
use crate::Error::{EmptyBuffer, FullBuffer};

pub struct CircularBufferOption<T> {
    buffer: Vec<Option<T>>,
    read_index: usize,
    write_index: usize,
    n_elements: usize
}

impl<T> CircularBufferOption<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            read_index: 0,  // index of the first element to be read
            write_index: 0, // index of the next element to be written
            n_elements: 0, // number of elements in the buffer
            buffer: (0..capacity).into_iter().map(|_| None).collect() // initialize the buffer
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.n_elements == self.buffer.len() { return Err(FullBuffer) }
        self.buffer[self.write_index] = Option::from(_element);
        self.n_elements += 1;
        self.write_index = (self.write_index+1) % self.buffer.len();
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, crate::Error> {
        if self.n_elements == 0 { return Err(EmptyBuffer) }
        let ret_val = self.buffer[self.read_index].take();
        self.n_elements -= 1;
        self.read_index = (self.read_index+1) % self.buffer.len();
        Ok(ret_val.unwrap())
    }

    pub fn clear(&mut self) {
        (*self).n_elements = 0;
        (*self).read_index = 0;
        (*self).write_index = 0;
        (*self).buffer = (*self).buffer.iter().map(|_| None).collect();
        // another solution:
        // *self = Self::new(self.buffer.len());
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.n_elements < self.buffer.len() { return self.write(_element).unwrap(); }
        self.buffer[self.write_index] = Option::from(_element);
        self.write_index = (self.write_index+1) % self.buffer.len();
        self.read_index = (self.read_index+1) % self.buffer.len();
    }
}
