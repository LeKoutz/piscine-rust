use std::{cell::{RefCell}, rc::Rc};

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: RefCell<usize>,
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: Default::default(),
            value: Default::default(),
            max
        }
    }
    pub fn set_value(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_owned());
            return;
        }
        self.value.replace(count);
        let percentage = (count as f64 / self.max as f64) * 100.0;
        if percentage >= 70.0 {
            self.messages.borrow_mut().push(format!("Warning: You have used up over {}% of your quota!", percentage as usize));
        }
    }
    pub fn peek(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        let percentage = (count as f64 / self.max as f64) * 100.0;
        self.messages.borrow_mut().push(format!("Info: This value would use {}% of your quota", percentage as usize));
    }
}