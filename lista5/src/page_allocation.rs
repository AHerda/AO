use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CounterState {
    Normal,
    Waiting,
}

pub enum Request {
    Read(u32),
    Write(u32),
}

impl Request {
    pub fn new_random_request(p: f64, page: u32) -> Self {
        match rand::random_bool(p) {
            true => Request::Write(page),
            false => Request::Read(page),
        }
    }
}

pub struct PageAllocation {
    counts: [(u64, CounterState); 64],
    treshhold: u64,
    copies: HashSet<u32>,
    max_copies: u8,
}

impl PageAllocation {
    pub fn new(treshhold: u64) -> Self {
        let mut copies = HashSet::new();
        copies.insert(0);
        let mut counts = [(0, CounterState::Normal); 64];
        counts[0].1 = CounterState::Waiting;
        Self {
            counts,
            treshhold,
            copies,
            max_copies: 1,
        }
    }

    pub fn process_request(&mut self, request: &Request) -> u64 {
        match request {
            Request::Read(page) => self.process_read(page),
            Request::Write(page) => self.process_write(page),
        }
    }

    pub fn process_requests(&mut self, requests: &[Request]) -> (u64, u64) {
        (
            requests
                .iter()
                .fold(0, |acc, x| acc + self.process_request(x)),
            self.max_copies as u64,
        )
    }

    fn process_read(&mut self, page: &u32) -> u64 {
        let mut cost = 0_u64;

        if !self.copies.contains(page) {
            cost += 1;

            if self.counts[*page as usize].0 < self.treshhold {
                self.counts[*page as usize].0 += 1;
            }
        }


        if self.counts[*page as usize].0 == self.treshhold {
            cost += self.add_copy(page);
        }

        cost
    }

    fn process_write(&mut self, page: &u32) -> u64 {
        let mut cost = 0_u64;
        let c = self.copies.len() as u64;
        if self.copies.contains(page) {
            cost += c - 1;
        } else {
            cost += c;

            if c == 1
                && self.counts[*page as usize].0 < self.treshhold
                && self.counts.iter().any(|(_, state)| {*state == CounterState::Waiting}) {
                self.counts[*page as usize].0 += 1;
            }
        };


        if self.counts[*page as usize].0 == self.treshhold {
            cost += self.add_copy(page);
        }

        for p in 0..64 {
            if p != *page {
                self.process_write_by_another_page(p as usize);
            }
        }

        cost
    }

    fn process_write_by_another_page(&mut self, page: usize) {
        if self.counts[page].0 > 0 && self.copies.contains(&(page as u32)) {
            self.counts[page].0 -= 1;
        }

        if self.counts[page].0 == 0 {
            if self.copies.contains(&(page as u32)) {
                if self.copies.len() > 1 {
                    self.copies.remove(&(page as u32));
                    self.counts[page].1 = CounterState::Normal;
                } else {
                    self.counts[page].1 = CounterState::Waiting;
                }
            }
        }
    }

    fn add_copy(&mut self, page: &u32) -> u64 {
        if self.copies.insert(*page) {
            if self.copies.len() as u8 > self.max_copies {
                self.max_copies = self.copies.len() as u8;
            }

            let page_to_remove =
                self.copies.iter().find(|&&p| self.counts[p as usize].1 == CounterState::Waiting);
            if let Some(&page_to_remove) = page_to_remove {
                self.copies.remove(&page_to_remove);
                self.counts[page_to_remove as usize].1 = CounterState::Normal;
            }
            self.treshhold
        } else {
            0
        }
    }
}
