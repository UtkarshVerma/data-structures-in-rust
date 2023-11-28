struct Link<T: Copy + Eq> {
    target: Box<Node<T>>,
}

impl<T: Copy + Eq> Link<T> {
    fn new(node: Box<Node<T>>) -> Self {
        Link { target: node }
    }
}

struct Node<T: Copy + Eq> {
    value: T,

    next: Option<Link<T>>,
}

impl<T: Copy + Eq> Node<T> {
    fn new(value: T, next: Option<Link<T>>) -> Box<Self> {
        Box::new(Node { value, next })
    }
}

pub struct SinglyLinkedList<T: Copy + Eq> {
    head: Option<Link<T>>,
    pub length: usize,
}

impl<T: Copy + Eq> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        SinglyLinkedList::new()
    }
}

impl<T: Copy + Eq> SinglyLinkedList<T> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Link<T>> {
        if index >= self.length {
            return None;
        }

        let mut link = self.head.as_mut();
        for _ in 0..index {
            link = match link {
                None => break,
                Some(link) => link.target.next.as_mut(),
            }
        }

        link
    }

    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }

        let mut link = self.head.as_ref();
        for _ in 0..index {
            link = match link {
                None => break,
                Some(link) => link.target.next.as_ref(),
            };
        }

        link.map(|link| link.target.value)
    }

    pub fn prepend(&mut self, value: T) {
        self.insert_at(value, 0);
    }

    pub fn append(&mut self, value: T) {
        self.insert_at(value, self.length);
    }

    pub fn insert_at(&mut self, value: T, index: usize) -> bool {
        let prev_node_next = match index {
            0 => &mut self.head,
            _ => match self.get_mut(index - 1) {
                None => return false,
                Some(prev_link) => &mut prev_link.target.next,
            },
        };

        let node = Node::new(value, (*prev_node_next).take());
        *prev_node_next = Some(Link::new(node));
        self.length += 1;
        true
    }

    pub fn remove(&mut self, value: T) -> Option<T> {
        let (prev_link, found) = if self
            .head
            .as_ref()
            .is_some_and(|head| head.target.value == value)
        {
            (None, true)
        } else {
            let mut prev_link: Option<&mut Link<T>> = self.head.as_mut();
            while let Some(ref prev) = prev_link {
                let next_link = prev.target.next.as_ref();
                if next_link.is_some_and(|next| next.target.value == value) {
                    break;
                }

                prev_link = prev_link.and_then(|link| link.target.next.as_mut());
            }

            let found = prev_link.is_some();
            (prev_link, found)
        };

        if found {
            let link_ptr = match prev_link {
                None => &mut self.head,
                Some(prev) => &mut prev.target.next,
            };

            let link = link_ptr.take();
            let value = link.as_ref().map(|l| l.target.value);
            *link_ptr = link.and_then(|mut link| link.target.next.take());

            self.length -= 1;
            return value;
        }

        None
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        let prev_node_next = match index {
            0 => &mut self.head,
            _ => match self.get_mut(index - 1) {
                None => return None,
                Some(prev_link) => &mut prev_link.target.next,
            },
        };

        let curr_link = prev_node_next.take();
        match curr_link {
            None => None,
            Some(curr_link) => {
                let curr_node = curr_link.target;
                *prev_node_next = curr_node.next;
                self.length -= 1;

                Some(curr_node.value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singly_linked_list() {
        let mut list = SinglyLinkedList::new();

        list.append(5);
        list.append(7);
        list.append(9);

        assert_eq!(list.get(2), Some(9));
        assert_eq!(list.remove_at(1), Some(7));
        assert_eq!(list.length, 2);

        list.append(11);
        assert_eq!(list.remove_at(1), Some(9));
        assert_eq!(list.remove(9), None);
        assert_eq!(list.remove_at(0), Some(5));
        assert_eq!(list.remove_at(0), Some(11));
        assert_eq!(list.length, 0);

        list.prepend(5);
        list.prepend(7);
        list.prepend(9);

        assert_eq!(list.get(2), Some(5));
        assert_eq!(list.remove(5), Some(5));
        assert_eq!(list.get(0), Some(9));
        assert_eq!(list.remove(9), Some(9));
        assert_eq!(list.length, 1);
        assert_eq!(list.get(0), Some(7));
    }
}
