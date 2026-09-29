#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl <'a>Numbers<'a> {
    pub fn new(numbers: &'a[u32]) -> Self {
        Self {
            numbers 
        }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.iter().last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut result = self.numbers.to_owned();
        result.sort();
        result.reverse();
        result.truncate(3);
        result
    }
}
