mod rand_dist;
mod page_allocation;

use page_allocation::{PageAllocation, Request};

use rayon::prelude::*;

use std::{io::Write, sync::{Arc, Mutex}};

fn main() {
    let number_of_pages: usize = 64;
    let request_size: usize = 1 << 16;
    let iterations = 10000;
    let ds: Vec<u64> = (4..=8).map(|i| 1 << i).collect();
    let ps: Vec<f64> = vec![0.01, 0.02, 0.05, 0.1, 0.2, 0.5];
    let file = Arc::new(std::fs::File::create("results.csv").expect("Unable to create file"));

    writeln!(file.clone(), "D,p,avg_cost,avg_copies").expect("Unable to write header");

    ds.par_iter().for_each(|d| {
        println!("Processing d = {}", d);
        ps.par_iter().for_each(|p| {
            println!("\tProcessing p = {}", p);
            // Create a new RandDist for each thread
            let mut rng = rand_dist::RandDist::new_uniform(number_of_pages);

            let (full_cost, full_copies) = (0..iterations)
                .fold((0_u64, 0_u64), |acc, _x| {
                    let samples = rng.sample_n(request_size);
                    let requests: Vec<Request> = samples
                        .iter()
                        .map(|page| Request::new_random_request(*p, *page as u32))
                        .collect();

                    let mut page_allocation = PageAllocation::new(*d);
                    let (cost, copies) = page_allocation.process_requests(&requests);

                    (acc.0 + cost, acc.1 + copies)
                });

            writeln!(
                file.clone(),
                "{},{},{},{}",
                d, p, full_cost as f64 / iterations as f64, full_copies as f64 / iterations as f64
            ).expect("Unable to write results");
        });
    });
}
