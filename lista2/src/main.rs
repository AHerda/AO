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
        CacheType::Rma2,
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

            let filename = format!("data/avg_cost_n-{}_dist-{}.csv", n, distribution);
            let mut file = File::create(&filename).expect("Unable to create file");
            let mut headers = vec!["k".to_owned()];

            for cache_type in &cache_types {
                headers.push(format!("{}", cache_type));
            }
            writeln!(file, "{}", headers.join(",")).expect("Unable to write to file");

            for k in &ks {
                println!("\t\tRunning experiment for k = {}", k);

                let mut row = vec![format!("{}", k)];

                for cache_type in &cache_types {
                    let mut cache = Cache::new(k.clone(), Some(cache_type.clone()));
                    let hits: usize = (0..num_of_tests)
                        .map(|_| cache.get_page(distribution.sample()))
                        .sum();
                    let avg_cost = hits as f64 / num_of_tests as f64;

                    row.push(format!("{}", avg_cost));
                }
                writeln!(file, "{}", row.join(",")).expect("Unable to write to file");
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
