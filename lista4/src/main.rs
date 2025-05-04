mod page_migration;
mod rand_dist;

use rayon::prelude::*;

use std::fs::File;
use std::io::Write;

use page_migration::*;
use rand_dist::*;

fn experiment(d: usize, dist: RandDistType, structure: GraphStructure, algo: MigrationType) -> usize {
    let mut distribution = RandDist::new(dist, 64);
    let seq = distribution.sample_n(65536);
    let mut pm = PageMigration::new(
        0,
        d,
        structure,
        algo,
    );

    pm.migrate_multiple(&seq)
}

fn experiments() {
    let num_of_tests = 1_00_usize;
    let distributions_type = vec![
        RandDistType::Uniform,
        RandDistType::Harmonic,
        RandDistType::DoubleHarmonic,
    ];
    let structures_and_algorithms = vec![
        (GraphStructure::Hypercube, MigrationType::MoveToMin),
        (GraphStructure::Hypercube, MigrationType::CoinFlip),
        (GraphStructure::Torus, MigrationType::MoveToMin),
        (GraphStructure::Torus, MigrationType::CoinFlip),
    ];
    let ds: Vec<usize> = vec![16, 32, 64, 128, 256];


    distributions_type.iter().for_each(|dist_type| {
        println!("Running experiments for distribution: {}", dist_type);
        let filename = format!("data/2dist-{}.csv", dist_type);
        let mut file = File::create(&filename).expect("Unable to create file");
        writeln!(
            file,
            "D,{}",
            structures_and_algorithms
                .iter()
                .map(|(struture, algo)| format!("{} with {}", struture, algo))
                .collect::<Vec<_>>()
                .join(",")
        ).expect("Unable to wrote to file");

        for d in &ds {
            println!("\t- Running experiments for d: {}", d);
            let mut row = vec![format!("{}", d)];
            for (structure, algo) in &structures_and_algorithms {
                println!("\t\t- Running experiments for structure: {} and algo: {}", structure, algo);
                let avg_moving_cost: f64 = (0..num_of_tests).into_par_iter().fold(|| 0.0, |acc, _| {
                    acc + (experiment(*d, *dist_type, *structure, *algo) as f64 / num_of_tests as f64)
                }).sum();

                row.push(format!("{:.3}", avg_moving_cost));
            }
            writeln!(file, "{}", row.join(",")).expect("Unable to write to file");
        }
    });
}

fn main() {
    let now = std::time::Instant::now();
    experiments();
    println!("Time elapsed: {} s", now.elapsed().as_secs_f64());
}
