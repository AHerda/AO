use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct LinkedList<T: Eq + Hash + Clone + Debug> {
    head: Option<Box<Node<T>>>,
    list_type: ListType<T>,
    index: usize,
    len: usize,
}

#[derive(Clone, Debug)]
struct Node<T: Eq> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
pub enum ListType<T: Hash> {
    Normal,
    Mtf,
    Transpose,
    Count(HashMap<T, usize>),
}

impl<T: Eq + Hash + Clone + Debug> LinkedList<T> {
    pub fn new(list_type: ListType<T>) -> Self {
        Self {
            head: None,
            list_type,
            index: 0,
            len: 0,
        }
    }

    fn new_node(value: T) -> Box<Node<T>> {
        Box::new(Node { value, next: None })
    }

    fn get_weight_mut(&mut self, i: &T) -> Option<&mut usize> {
        match &mut self.list_type {
            ListType::Count(count) => Some(count.get_mut(&i)?),
            _ => panic!("ListType is not Count"),
        }
    }

    fn get_weight(&self, i: &T) -> Option<&usize> {
        match &self.list_type {
            ListType::Count(count) => Some(count.get(&i)?),
            _ => panic!("ListType is not Count"),
        }
    }

    fn get_nth(&mut self, n: usize) -> Option<&mut Box<Node<T>>> {
        let mut current = self.head.as_mut().unwrap();
        if n == 0 || n >= self.len {
            return Some(current);
        }
        for _ in 0..(n - 1) {
            current = current.next.as_mut().unwrap();
        }
        Some(current)
    }

    pub fn access(&mut self, i: T) -> usize {
        if self.head.is_none() {
            if let ListType::Count(count) = &mut self.list_type {
                count.insert(i.clone(), 1);
            }

            self.head = Some(Self::new_node(i));
            self.len = 1;
            return 0;
        }

        match self.list_type {
            ListType::Normal => self.normal_access(i),
            ListType::Mtf => self.mtf_access(i),
            ListType::Transpose => self.transpose_access(i),
            ListType::Count(_) => self.count_access(i),
        }
    }

    fn normal_access(&mut self, i: T) -> usize {
        let mut index = 1;
        let mut current = &mut self.head;

        if current.as_ref().unwrap().value == i {
            return index;
        }

        while let Some(next) = current.as_ref().unwrap().next.as_ref() {
            index += 1;
            if next.value == i {
                return index;
            }

            current = &mut current.as_mut().unwrap().next;
        }

        current.as_mut().unwrap().next = Some(Self::new_node(i));
        self.len += 1;
        index
    }

    fn mtf_access(&mut self, i: T) -> usize {
        let mut index = 1;
        let mut current = &mut self.head;

        if current.as_ref().unwrap().value == i {
            return index;
        }

        while let Some(next) = &mut current.as_mut().unwrap().next {
            index += 1;
            if next.value == i {
                let mut found = current.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = found.as_mut().unwrap().next.take();
                found.as_mut().unwrap().next = self.head.take();
                self.head = found;
                return index;
            }

            current = &mut current.as_mut().unwrap().next;
        }

        current.as_mut().unwrap().next = Some(Self::new_node(i));
        self.len += 1;
        index
    }

    fn transpose_access(&mut self, i: T) -> usize {
        let mut index = 1;
        let mut current = &mut self.head;

        if current.as_ref().unwrap().value == i {
            return index;
        }

        while let Some(next) = &current.as_ref().unwrap().next {
            index += 1;

            if next.value == i {
                let mut found = current.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = found.as_mut().unwrap().next.take();
                let temp = current.take();
                found.as_mut().unwrap().next = temp;
                *current = found;
                return index;
            }

            current = &mut current.as_mut().unwrap().next;
        }

        current.as_mut().unwrap().next = Some(Self::new_node(i));
        self.len += 1;
        index
    }

    fn count_access(&mut self, i: T) -> usize {
        let mut index = 1;
        let weights = match &mut (self.list_type) {
            ListType::Count(count) => count,
            _ => panic!("ListType is not Count"),
        };
        let head_weight = weights.get(&self.head.as_ref().unwrap().value).unwrap().clone();
        let mut current = &mut self.head;
        let mut current_count = *weights.get(&current.as_ref().unwrap().value).unwrap();
        let mut last_of_higher_count_index = 0;

        if current.as_ref().unwrap().value == i {
            *weights.get_mut(&i).unwrap() += 1;
            return index;
        }

        while let Some(next) = &current.as_ref().unwrap().next {
            index += 1;

            if next.value == i {
                let w_current = weights.get(&current.as_ref().unwrap().value).unwrap().clone();
                let w = weights.get_mut(&i).unwrap();
                *w += 1;

                if w_current == *w {
                    return index;
                }

                let mut found = current.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = found.as_mut().unwrap().next.take();

                if last_of_higher_count_index == 0 && *w > head_weight {
                    found.as_mut().unwrap().next = self.head.take();
                    self.head = found.take();
                    return index;
                }

                let mut last_of_higher_count = self.head.as_mut();
                for _ in 0..last_of_higher_count_index {
                    last_of_higher_count = last_of_higher_count.unwrap().next.as_mut();
                }

                let mut temp = found.take();
                temp.as_mut().unwrap().next = last_of_higher_count.as_mut().unwrap().next.take();
                last_of_higher_count.as_mut().unwrap().next = temp;

                return index;
            }

            if weights.get(&next.value).unwrap() < &current_count {
                last_of_higher_count_index = index - 2;
                current_count = *weights.get(&current.as_ref().unwrap().value).unwrap();
            }

            current = &mut current.as_mut().unwrap().next;
        }

        weights.insert(i.clone(), 1);
        current.as_mut().unwrap().next = Some(Self::new_node(i));

        self.len += 1;
        index
    }
}


impl<T: Eq + Hash + Clone + Debug> Iterator for &mut LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.head {
            Some(node) => {
                if self.index < self.len {
                    let mut current = node;
                    for _ in 0..self.index {
                        current = current.next.as_ref()?;
                    }
                    self.index += 1;
                    Some(current.value.clone())
                } else {
                    self.index = 0;
                    None
                }
            }
            None => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let mut list = LinkedList::new(ListType::Normal);
        assert_eq!(list.len, 0);
        list.access(1);
        assert_eq!(list.len, 1);
        list.access(2);
        assert_eq!(list.len, 2);
        list.access(1);
        assert_eq!(list.len, 2);
        list.access(3);
        assert_eq!(list.len, 3);
        list.access(2);
        assert_eq!(list.len, 3);
    }

    #[test]
    fn test_iterator() {
        let mut list = LinkedList::new(ListType::Normal);
        list.access(1);
        list.access(2);
        list.access(3);

        let mut list = list.into_iter();
        assert_eq!(list.next(), Some(1));
        assert_eq!(list.next(), Some(2));
        assert_eq!(list.next(), Some(3));
        assert_eq!(list.next(), None);
    }

    #[test]
    fn test_normal_access() {
        let mut list = LinkedList::new(ListType::Normal);
        assert_eq!(list.access(1), 0);
        assert_eq!(list.access(2), 1);
        assert_eq!(list.access(1), 1);
        assert_eq!(list.access(3), 2);
        assert_eq!(list.access(2), 2);

        for (list, expected) in list.into_iter().zip(vec![1, 2, 3].into_iter()) {
            assert_eq!(expected, list, "Expected {}, got {}", expected, list);
        }
    }

    #[test]
    fn test_mtf_access() {
        let mut list = LinkedList::new(ListType::Mtf);
        assert_eq!(list.access(1), 0);
        assert_eq!(list.access(2), 1);
        assert_eq!(list.access(1), 1);
        assert_eq!(list.access(3), 2);
        assert_eq!(list.access(2), 2);

        for (list, expected) in list.into_iter().zip(vec![2, 1, 3].into_iter()) {
            assert_eq!(expected, list, "Expected {}, got {}", expected, list);
        }
    }

    #[test]
    fn test_transpose_access() {
        let mut list = LinkedList::new(ListType::Transpose);
        assert_eq!(list.access(1), 0);
        assert_eq!(list.access(2), 1);
        assert_eq!(list.access(1), 1);
        assert_eq!(list.access(3), 2);
        assert_eq!(list.access(2), 2);
        assert_eq!(list.access(4), 3, "List: {:?}", list.into_iter().collect::<Vec<usize>>());

        for (from_list, expected) in list.into_iter().zip(vec![2, 1, 3, 4].into_iter()) {
            assert_eq!(expected, from_list, "Expected {}, got {}\nList: {:?}", expected, from_list, list.into_iter().collect::<Vec<usize>>());
        }
    }

    #[test]
    fn test_count_access_small() {
        let mut list = LinkedList::new(ListType::Count(HashMap::new()));
        assert_eq!(list.access(1), 0);
        assert_eq!(list.access(2), 1);
        assert_eq!(list.access(2), 2);

        assert_eq!(list.get_weight(&1).unwrap(), &1);
        assert_eq!(list.get_weight(&2).unwrap(), &2);

        for (i, (from_list, expected)) in list.into_iter().zip(vec![2, 1].into_iter()).enumerate() {
            assert_eq!(expected, from_list, "\nPass: {i},\nExpected {}, got {}\nList: {:?}, Len {}", expected, from_list, list.into_iter().collect::<Vec<usize>>(), list.into_iter().count());
        }
    }

    #[test]
    fn test_count_access() {
        let mut list = LinkedList::new(ListType::Count(HashMap::new()));
        assert_eq!(list.access(1), 0);
        assert_eq!(list.access(2), 1);
        assert_eq!(list.access(1), 1);
        assert_eq!(list.access(3), 2);
        assert_eq!(list.access(3), 3);

        assert_eq!(list.get_weight(&1).unwrap(), &2);
        assert_eq!(list.get_weight(&2).unwrap(), &1);
        assert_eq!(list.get_weight(&3).unwrap(), &2);

        assert_eq!(list.access(2), 3);
        assert_eq!(list.access(4), 3);
        assert_eq!(list.access(4), 4);
        assert_eq!(list.access(4), 4);

        for (i, (from_list, expected)) in list.into_iter().zip(vec![4, 1, 3, 2].into_iter()).enumerate() {
            assert_eq!(expected, from_list, "\nPass: {i},\nExpected {}, got {}\nList: {:?}, Len {}", expected, from_list, list.into_iter().collect::<Vec<usize>>(), list.into_iter().count());
        }
    }

    #[test]
    fn test_count_weights() {
        let mut list = LinkedList::new(ListType::Count(HashMap::new()));
        list.access(1);
        list.access(2);
        list.access(1);
        list.access(3);
        list.access(2);
        list.access(1);
        list.access(2);

        assert_eq!(list.get_weight(&1).unwrap(), &3);
        assert_eq!(list.get_weight(&2).unwrap(), &3);
        assert_eq!(list.get_weight(&3).unwrap(), &1);
    }
}
