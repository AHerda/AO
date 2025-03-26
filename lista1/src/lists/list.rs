use std::cell::RefCell;
use std::rc::Rc;

pub struct LinkedList<T: PartialEq> {
    head: Option<Rc<RefCell<Node<T>>>>,
    list_type: ListType,
}

struct Node<T: PartialEq> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

enum ListType {
    Normal,
    Mtf,
    Transpose,
}

impl<T: PartialEq> LinkedList<T> {
    pub fn new(list_type: ListType) -> Self {
        Self {
            head: None,
            list_type,
        }
    }

    fn new_node(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { value, next: None }))
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
        let mut current = self.head.as_ref().unwrap().clone();

        if current.borrow().value == i {
            return index;
        }

        while let Some(next) = &current.clone().borrow_mut().next {
            index += 1;
            if next.borrow().value == i {
                return index;
            }

            current = next.clone();
        }

        current.borrow_mut().next = Some(Self::new_node(i));
        index
    }

    fn mtf_access(&mut self, i: T) -> usize {
        let mut index = 1;
        let mut current = self.head.as_ref().unwrap().clone();

        if current.borrow().value == i {
            return index;
        }

        while let Some(next) = &current.clone().borrow_mut().next {
            index += 1;
            if next.borrow().value == i {
                current.borrow_mut().next = next.borrow().next.clone();
                next.borrow_mut().next = self.head.clone();
                self.head = Some(next.clone());
                return index;
            }

            current = next.clone();
        }

        current.borrow_mut().next = Some(Self::new_node(i));
        index
    }

    fn transpose_access(&mut self, i: T) -> usize {
        let mut index = 2;
        let mut prev = self.head.as_ref().unwrap().clone();
        let mut current;

        if prev.borrow().value == i {
            return 1;
        }

        match &prev.borrow().next {
            Some(node) => current = node.clone(),
            None => {
                prev.borrow_mut().next = Some(Self::new_node(i));
                return 1;
            }
        };

        if current.borrow().value == i {
            return index;
        }

        while let Some(next) = &current.clone().borrow().next {
            index += 1;

            if next.borrow().value == i {
                prev.borrow_mut().next = Some(next.clone());
                current.borrow_mut().next = next.borrow().next.clone();
                next.borrow_mut().next = Some(current.clone());
                return index;
            }

            prev = current.clone();
            current = next.clone();
        }

        current.borrow_mut().next = Some(Self::new_node(i));
        index
    }
}
