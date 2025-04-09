use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

#[derive(Copy, Clone, Debug)]
pub enum CacheType {
    Fifo,
    Fwf,
    Lru,
    Lfu,
    Rand,
    Rma,
    Rma2,
}

impl std::fmt::Display for CacheType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheType::Fifo => write!(f, "FIFO"),
            CacheType::Fwf => write!(f, "FWF"),
            CacheType::Lru => write!(f, "LRU"),
            CacheType::Lfu => write!(f, "LFU"),
            CacheType::Rand => write!(f, "RAND"),
            CacheType::Rma => write!(f, "RMA"),
            CacheType::Rma2 => write!(f, "RMA2"),
        }
    }
}

pub struct Cache<T>
where
    T: Eq + Hash + Clone,
{
    size: usize,
    pub data: VecDeque<T>,
    cache_type: CacheType,
    lfu_counter: Option<HashMap<T, usize>>,
    rma_marker: Option<Vec<bool>>,
}

impl<T> Cache<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(size: usize, cache_type: Option<CacheType>) -> Self {
        Cache {
            size,
            data: VecDeque::with_capacity(size),
            lfu_counter: match cache_type {
                Some(CacheType::Lfu) => Some(HashMap::new()),
                _ => None,
            },
            rma_marker: match cache_type {
                Some(CacheType::Rma) | Some(CacheType::Rma2) => Some(Vec::with_capacity(size)),
                _ => None,
            },
            cache_type: cache_type.unwrap_or(CacheType::Fifo),
        }
    }

    pub fn get_page(&mut self, page: T) -> usize {
        if self.data.contains(&page) {
            self.update_page(&page);
            0
        } else {
            self.add_page(page);
            1
        }
    }

    fn update_page(&mut self, page: &T) {
        match self.cache_type {
            CacheType::Lru => self.lru_update(page),
            CacheType::Lfu => self.lfu_update(page),
            CacheType::Rma | CacheType::Rma2 => self.rma_update(page),
            _ => (),
        }
    }

    fn add_page(&mut self, page: T) {
        match self.cache_type {
            CacheType::Fifo => self.fifo_add(page),
            CacheType::Fwf => self.fwf_add(page),
            CacheType::Lru => self.lru_add(page),
            CacheType::Lfu => self.lfu_add(page),
            CacheType::Rand => self.rand_add(page),
            CacheType::Rma => self.rma_add(page, true),
            CacheType::Rma2 => self.rma_add(page, false),
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

    fn lru_update(&mut self, page: &T) {
        let pos = self
            .data
            .iter()
            .position(|x| x == page)
            .expect("Err 1\nPage not found");
        let page = self
            .data
            .remove(pos)
            .expect(format!("Err 2\nError removing page at index {}", pos).as_str());
        self.data.push_front(page);
    }

    fn lfu_add(&mut self, page: T) {
        if self.data.len() == self.size {
            let (pos, _) = self
                .data
                .iter()
                .enumerate()
                .min_by_key(|(_, x)| self.lfu_counter.as_ref().unwrap()[x])
                .unwrap();
            self.data.remove(pos);
        }
        self.lfu_update(&page);
        self.data.push_back(page);
    }

    fn lfu_update(&mut self, page: &T) {
        self.lfu_counter
            .as_mut()
            .unwrap()
            .entry(page.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(0);
    }

    fn rand_add(&mut self, page: T) {
        if self.data.len() == self.size {
            self.data.remove(rand::random_range(..self.size));
        }
        self.data.push_back(page);
    }

    fn rma_add(&mut self, page: T, marked: bool) {
        if self.data.len() == self.size {
            let mut unmarked_count = self
                .rma_marker
                .as_ref()
                .unwrap()
                .into_iter()
                .filter(|&marker| !marker)
                .count();

            if unmarked_count == 0 {
                self.rma_marker
                    .as_mut()
                    .unwrap()
                    .iter_mut()
                    .for_each(|marker| *marker = false);
                unmarked_count = self.size;
            }

            let unmarked_pos = rand::random_range(..unmarked_count);
            let (pos, _) = self
                .rma_marker
                .as_ref()
                .unwrap()
                .into_iter()
                .enumerate()
                .filter(|(_, &marker)| !marker)
                .nth(unmarked_pos)
                .expect(
                    format!(
                        "Err 4\nUnmarked index {} too big for {} unmarked found",
                        unmarked_pos, unmarked_count
                    )
                    .as_str(),
                );

            self.data.remove(pos);
            self.rma_marker.as_mut().unwrap().remove(pos);
        }

        self.data.push_back(page);
        self.rma_marker.as_mut().unwrap().push(marked);
    }

    fn rma_update(&mut self, page: &T) {
        let pos = self
            .data
            .iter()
            .position(|x| x == page)
            .expect("Err 3\n Page Not found in cache");
        self.rma_marker.as_mut().unwrap()[pos] = true;
    }
}
