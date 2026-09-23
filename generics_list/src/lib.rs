#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl <T> List<T> {
    pub fn new() -> Self {
        Self {
            head: Default::default()
        }
    }

    pub fn push(&mut self, value: T) {
        let mut new = Node{ value, next: Default::default()};
        match self.head.take() {
            None => self.head = Some(new),
            Some(old) => {
                new.next = Some(Box::new(old));
                self.head = Some(new);
            }
        }
    }

    pub fn pop(&mut self) {
        match self.head.take() {
            None => self.head = None,
            Some(head) => self.head = head.next.map(|boxed| *boxed)
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            len += 1;
            current = node.next.as_deref();
        }
        len
    }
}
