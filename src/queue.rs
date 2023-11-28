pub struct Queue<T> {
    contents: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue { contents: vec![] }
    }

    fn enqueue(mut self, value: T) {
        self.contents.push(value);
    }

    fn dequeue(mut self) -> Option<T> {
        self.head.map(|head| {
            self.head = head.prev;
            head.value
        })
    }

    fn peek(self) -> Option<T> {
        self.head.map(|head| head.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
