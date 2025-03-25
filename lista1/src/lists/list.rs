pub struct LinkedList<T: PartialEq> {
    head: Option<Box<Node<T>>>,
    list_type: ListType,
}

struct Node<T: PartialEq> {
    value: T,
    next: Option<Box<Node<T>>>,
}

enum ListType {
    Normal,
    Mtf,
    Transpose,
}

impl<T: PartialEq> LinkedList<T> {
    pub fn new(list_type: ListType) -> Self {
        Self {head: None, list_type}
    }

    fn new_node(value: T) -> Box<Node<T>> {
        Box::new(Node {value, next: None})
    }

    pub fn access(&mut self, i: T) -> usize {
        if self.head.is_none() {
            self.head = Some(Self::new_node(i));
            return 0;
        }

        match self.list_type {
            ListType::Normal => self.normal_access(i),
            ListType::Mtf => self.mtf_access(i),
            ListType::Transpose => self.transpose_access(i),
        }

    }

    fn normal_access(&mut self, i: T) -> usize {
        let mut index = 1;
        let mut current = self.head.as_mut().unwrap();

        if current.value == i {
            return index;
        }

        while let Some(node) = &current.next {
            index += 1;
            if node.value == i {
                return index;
            }

            current = current.next.as_mut().unwrap();
        }

        current.next = Some(Self::new_node(i));
        index
    }

    fn mtf_access(&mut self, i: T) -> usize {
        let mut index = 1;
        let mut prev = self.head.as_mut().unwrap();
        let mut current = &mut prev.next;

        if prev.value == i {
            return index;
        }

        while let Some(node) = current.as_mut() {
            index += 1;

            if node.value == i {
                prev.next = node.next;
                node.next = self.head;
                self.head = Some(*node);
                return index;
            }

            prev = node;
            current = &mut node.next;
        }

        prev.next = Some(Self::new_node(i));
        index
    }

    fn transpose_access(&mut self, i: T) -> usize {
        let mut index = 1;
        let mut prev_prev = head;
        let mut prev;
        let mut current;

        if prev_prev.value == i {
            return index;
        }

        match prev_prev.next {
            Some(node) => {
                if node.value == i {
                    prev_prev.next = node.next;
                    node.next = Some(prev_prev);
                    self.head = Some(*node);
                    return index;
                }

                prev = node;
                current = node.next;
            }
            None => {
                prev_prev.next = Some(Self::new_node(i));
                return index;
            }
        }

        while let Some(node) = &mut current {
            index += 1;

            if node.value == i {
                prev.next = node.next;
                node.next = Some(head);
                self.head = Some(*node);
                return index;
            }

            prev = current;
            current = node.next;
        }

        current.next = Some(Self::new_node(i));
        index
    }
}
