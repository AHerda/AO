mod page_migration;
mod rand_dist;

use rayon::prelude::*;

use std::collections::HashMap;
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
    let num_of_tests = 1_000_usize;
    let distributions_type = vec![
        RandDistType::Uniform,
        RandDistType::Harmonic,
        RandDistType::DoubleHarmonic,
    ];
    let structures = vec![
        GraphStructure::Hypercube,
        GraphStructure::Torus
    ];
    let algorithms = vec![
        MigrationType::MoveToMin,
        MigrationType::CoinFlip,
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

        let mut files_alg: HashMap<_,_> = algorithms.iter().map(|algo| {
            let filename = format!("data/dist-{}_alg-{}.csv", dist_type, algo);
            let mut file = File::create(&filename).expect("Unable to create file");
            writeln!(
                file,
                "D,{}",
                structures
                    .iter()
                    .map(|struture| format!("{}", struture))
                    .collect::<Vec<_>>()
                    .join(",")
            ).expect("Unable to wrote to file");
            (*algo, file)
        }).collect();
        let mut files_struct: HashMap<_,_> = structures.iter().map(|structure| {
            let filename = format!("data/dist-{}_struct-{}.csv", dist_type, structure);
            let mut file = File::create(&filename).expect("Unable to create file");
            writeln!(
                file,
                "D,{}",
                algorithms
                    .iter()
                    .map(|algorithm| format!("{}", algorithm))
                    .collect::<Vec<_>>()
                    .join(",")
            ).expect("Unable to wrote to file");
            (*structure, file)
        }).collect();

        for d in &ds {
            println!("\t- Running experiments for d: {}", d);
            let mut rows_alg: HashMap<_, _> = algorithms.iter().map(|x| (*x, vec![format!("{}", d)])).collect();
            let mut rows_struct: HashMap<_, _> = structures.iter().map(|x| (*x, vec![format!("{}", d)])).collect();
            for (structure, algo) in &structures_and_algorithms {
                println!("\t\t- Running experiments for structure: {} and algo: {}", structure, algo);
                let avg_moving_cost: f64 = (0..num_of_tests).into_par_iter().fold(|| 0.0, |acc, _| {
                    acc + (experiment(*d, *dist_type, *structure, *algo) as f64 / num_of_tests as f64)
                }).sum();

                if let Some(row) = rows_alg.get_mut(algo) { row.push(format!("{:.3}", avg_moving_cost)) }
                if let Some(row) = rows_struct.get_mut(structure) { row.push(format!("{:.3}", avg_moving_cost)) }
            }
            algorithms.iter().for_each(|x| {
                if let Some(file) = files_alg.get_mut(x) {
                    writeln!(file, "{}", rows_alg[x].join(",")).expect("Unable to wrote to file");
                }
            });
            structures.iter().for_each(|x| {
                if let Some(file) = files_struct.get_mut(x) {
                    writeln!(file, "{}", rows_struct[x].join(",")).expect("Unable to wrote to file");
                }
            });
        }


    });
}

fn main() {
    let now = std::time::Instant::now();
    experiments();
    println!("Time elapsed: {} s", now.elapsed().as_secs_f64());
}
