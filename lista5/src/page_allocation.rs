use std::collections::HashSet;

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
    counts: [u64; 64],
    treshhold: u64,
    copies: HashSet<u32>,
}

impl PageAllocation {
    pub fn new(treshhold: u64) -> Self {
        let mut copies = HashSet::new();
        copies.insert(0);
        Self {
            counts: [0; 64],
            treshhold,
            copies,
        }
    }

    pub fn process_request(&mut self, request: &Request) -> u64 {
        match request {
            Request::Read(page) => {
                if self.copies.contains(&page) {
                    0
                } else {
                    1
                }
            }
            Request::Write(page) => {
                let mut cost: u64;
                if self.copies.contains(&page) {
                    cost = self.copies.len() as u64 - 1
                } else {
                    cost = self.copies.len() as u64
                }

                self.counts[*page as usize] += 1;
                if self.counts[*page as usize] >= self.treshhold {
                    if self.copies.insert(*page) {
                        cost += self.treshhold;
                    }
                }

                cost
            }
        }
    }

    pub fn process_requests(&mut self, requests: &[Request]) -> (u64, u64) {
        (
            requests
                .iter()
                .fold(0, |acc, x| acc + self.process_request(x)),
            self.copies.len() as u64,
        )
    }


}
