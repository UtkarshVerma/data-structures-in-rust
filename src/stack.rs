struct Node<T> {
    value: T,

    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    pub length: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.length += 1;

        let node = Box::new(Node::new(value));
        self.head = Some(node);
        let cur_head = (&*self).head;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.length -= 1;

        self.head.map(|head| {
            self.head = head.next;
            head.value
        })
    }

    pub fn peek(&self) -> Option<T> {
        self.head.map(|node| node.value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_stack() {
        let list: Stack<isize> = Stack::new();

        list.push(5);
        list.push(7);
        list.push(9);

        assert_eq!(list.pop(), Some(9));
        assert_eq!(list.length, 2);

        list.push(11);
        assert_eq!(list.pop(), Some(11));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.peek(), Some(5));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // just wanted to make sure that I could not blow up myself when i remove
        // everything
        list.push(69);
        assert_eq!(list.peek(), Some(69));
        assert_eq!(list.length, 1);

        //yayaya
    }
}
