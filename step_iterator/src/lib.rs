pub struct StepIterator<T> {
    beg: T,
    end: T,
    step: T
}

use std::ops::{Add, AddAssign};

impl <T>StepIterator<T> {
	pub fn new(beg: T, end: T, step: T) -> Self {
        Self { beg, end, step }
	}
}

impl <T: PartialOrd + Add<Output = T> + Copy + AddAssign>std::iter::Iterator for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.beg > self.end {
            return None;
        }
        let current = self.beg;
        self.beg += self.step;
        Some(current)
    }
}
