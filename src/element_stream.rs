pub struct ElementStream<T> {
    elements: Vec<T>,
    index: usize,
}

impl<T: Clone> ElementStream<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Self { elements, index: 0 }
    }

    pub fn peek(&self) -> Option<T> {
        if self.index + 1 > self.elements.len() {
            return None;
        }

        self.elements.get(self.index).cloned()
    }

    pub fn previous(&self, by: usize) -> Option<T> {
        let previous_index = self.index.checked_sub(by)?;
        self.elements.get(previous_index).cloned()
    }

    pub fn consume(&mut self) -> Option<T> {
        let element = self.peek();

        if self.index + 1 > self.elements.len() {
            return None;
        }

        self.index += 1;
        element
    }

    pub fn skip(&mut self) {
        if self.index + 1 > self.elements.len() {
            return;
        }

        self.index += 1;
    }
}
