use std::{collections::{HashMap, VecDeque}};

pub enum CacheType {
    Fifo,
    Fwf,
    Lru,
    Lfu,
    Rand,
    Rma,
}

pub struct Cache<T>
    where T: Eq
{
    size: usize,
    pub data: VecDeque<T>,
    cache_type: CacheType,
    lfu_counter: Option<HashMap<T, usize>>,
}


impl<T> Cache<T>
    where T: Eq
{
    pub fn new(size: usize, cache_type: Option<CacheType>) -> Self {
        Cache {
            size,
            data: VecDeque::with_capacity(size),
            lfu_counter: match cache_type {
                Some(CacheType::Lfu) => Some(HashMap::new()),
                _ => None,
            },
            cache_type: cache_type.unwrap_or(CacheType::Fifo),
        }
    }

    pub fn get_page(&mut self, page: T) -> usize {
        if self.data.contains(&page) {
            self.update_page(page);
            0
        } else {
            self.add_page(page);
            1
        }
    }

    fn update_page(&mut self, page: T) {
        match self.cache_type {
            CacheType::Lru => self.lru_update(page),
            _ => ()
        }
    }

    fn add_page(&mut self, page: T) {
        match self.cache_type {
            CacheType::Fifo => self.fifo_add(page),
            CacheType::Fwf => self.fwf_add(page),
            CacheType::Lru => self.lru_add(page),
            CacheType::Lfu => {
                // Implement LFU logic
            }
            CacheType::Rand => {
                // Implement Random logic
            }
            CacheType::Rma => {
                // Implement RMA logic
            }
        }
    }

    fn fifo_add(&mut self, page: T) {
        if self.data.len() == self.size {
            self.data.pop_front();
        }
        self.data.push_back(page);
    }

    fn fwf_add(&mut self, page: T) {
        if self.data.len() == self.size {
            self.data.clear();
        }
        self.data.push_back(page);
    }

    fn lru_add(&mut self, page: T) {
        if self.data.len() == self.size {
            self.data.pop_back();
        }
        self.data.push_front(page);
    }

    fn lru_update(&mut self, page: T) {
        let pos = self.data.iter().position(|x| *x == page).expect("Err 1\nPage not found");
        let page = self.data.remove(pos).expect(format!("Err 2\nError removing page at index {}", pos).as_str());
        self.data.push_front(page);
    }

    fn lfu_add(&mut self, page: T) {
        if self.data.len() == self.size {
            // Implement LFU eviction logic
        }
        self.data.push_back(page);
    }
}
