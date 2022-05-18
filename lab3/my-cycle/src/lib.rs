
#[derive(Clone)]
pub struct MyCycle<I: Clone+Iterator> {
    repeat: usize,
    base_iter: I,
    curr_iter: I
}

impl <I> MyCycle<I> where I : Clone+Iterator {
    pub fn new(iter: I, repeat: usize ) -> Self {
        Self {
            repeat,
            curr_iter: iter.clone(),
            base_iter: iter
        }
    }
}

impl<I> Iterator for MyCycle<I> where I : Clone+Iterator {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.curr_iter.next();
        if next.is_none() && (self.repeat == 0 || self.repeat > 1) {
            self.curr_iter = self.base_iter.clone();
            if self.repeat > 0 { self.repeat -= 1; }
            next = self.curr_iter.next();
        }
        next
    }
}
