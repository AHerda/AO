mod cache;
mod rand_dist;

use std::{fs::File, io::Write};

use cache::{Cache, CacheType};
use rand_dist::RandDist;
use rayon::prelude::*;

fn experiment() {
    let num_of_tests = 1_000_000usize;
    let ns: Vec<usize> = (20..=100).step_by(10).collect();
    let cache_types = vec![
        CacheType::Fifo,
        CacheType::Fwf,
        CacheType::Lru,
        CacheType::Lfu,
        CacheType::Rand,
        CacheType::Rma,
    ];

    ns.par_iter().for_each(|&n| {
        println!("Running experiment for n = {}", n);
        let ks: Vec<usize> = ((n / 10)..=(n / 5)).collect();
        let mut distributions = vec![
            RandDist::new_uniform(n),
            RandDist::new_harmonic(n),
            RandDist::new_double_harmonic(n),
            RandDist::new_geometric(n),
        ];
        for distribution in &mut distributions {
            println!("\tRunning experiment for distribution = {}", distribution);
            for cache_type in &cache_types {
                println!("\t\tRunning experiment for cache_type = {}", cache_type);
                let filename = format!(
                    "data/cache-{}_n-{}_dist-{}.csv",
                    cache_type, n, distribution
                );
                let mut file = File::create(&filename).expect("Unable to create file");
                writeln!(file, "k, avg_cost").expect("Unable to write to file");

                for k in &ks {
                    let mut cache = Cache::new(k.clone(), Some(cache_type.clone()));
                    let hits: usize = (0..num_of_tests)
                        .map(|_| cache.get_page(distribution.sample()))
                        .sum();
                    let avg_cost = hits as f64 / num_of_tests as f64;

                    writeln!(file, "{}, {}", k, avg_cost).expect("Unable to write to file");
                }
            }
        }
    });
    println!("Experiment completed.");
}

fn main() {
    let now = std::time::Instant::now();
    experiment();
    println!("Time elapsed: {} s", now.elapsed().as_secs_f64());
}
