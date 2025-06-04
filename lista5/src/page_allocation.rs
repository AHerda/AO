use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CounterState {
    Increasing,
    Decreasing,
    Holding,
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
}

impl PageAllocation {
    pub fn new(treshhold: u64) -> Self {
        let mut copies = HashSet::new();
        copies.insert(0);
        let mut counts = [(0, CounterState::Increasing); 64];
        counts[0].1 = CounterState::Holding;
        Self {
            counts,
            treshhold,
            copies,
        }
    }

    pub fn process_request(&mut self, request: &Request) -> u64 {
        match request {
            Request::Read(page) => self.process_read(page),
            Request::Write(page) => self.process_write(page),
        } + self.process_rest(page)
    }

    pub fn process_requests(&mut self, requests: &[Request]) -> (u64, u64) {
        (
            requests
                .iter()
                .fold(0, |acc, x| acc + self.process_request(x)),
            self.copies.len() as u64,
        )
    }

    fn process_read(&mut self, page: &u32) -> u64 {
        let mut cost = 0_u64;

        if !self.copies.contains(page) {
            cost += 1;
        }

        self.counts[*page as usize].0 += 1;
        if self.counts[*page as usize].0 >= self.treshhold {
            cost += self.add_copy(page);
        }

        cost
    }

    fn process_write(&mut self, page: &u32) -> u64 {
        let mut cost = 0_u64;
        let c = self.copies.len() as u64;
        cost += if self.copies.contains(page) {c - 1} else { c };

        self.counts[*page as usize].0 += 1;
        self.counts.iter_mut().enumerate().for_each(|(p, (count, state))| {
            if p != *page as usize {
                if *count > 0 {
                    *count -= 1;
                }
                if *count == 0 {
                    if self.copies.len() > 1 {
                        self.copies.remove(page);
                    } else {
                        *state = CounterState::ToDelete;
                    }
                }
            }
        });

        if self.counts[*page as usize].0 >= self.treshhold {
            cost += self.add_copy(page);
        }

        cost
    }

    fn process_rest(&mut self, page: &u32) -> u64 {


    fn add_copy(&mut self, page: &u32) -> u64 {
        if self.copies.insert(*page) {
            let mut idx: Option<usize> = None;
            for (i, (_, state)) in self.counts.iter_mut().enumerate() {
                if *state == CounterState::ToDelete && self.copies.remove(page) {
                    *state = CounterState::Waiting;
                    idx = Some(i);
                }
            }
            if let Some(i) = idx {
                assert_eq!(self.counts[i].1, CounterState::Waiting);
            }

            self.treshhold
        } else {
            0
        }
    }
}
