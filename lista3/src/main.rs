mod binpacking;
mod rand_dist;

use binpacking::*;
use rand_dist::*;

use std::{fs::File, io::Write};

use rand;
use rayon::prelude::*;

fn experiment(pack_type: &PackType, dist_type: &RandDistType) -> f64 {
    let mut weights = Vec::new();
    let mut dist = RandDist::new(*dist_type, 10);
    while weights.len() < 100 {
        let weight: f64 = rand::random::<f64>();
        for _ in 0..dist.sample() {
            weights.push(weight);
        }
    }

    let mut bp = BinPacking::new(*pack_type);
    bp.pack_multiple(&weights);
    bp.number_of_bins() as f64 / BinPacking::optimal_packing(&weights) as f64
}

fn experiments() {
    let num_of_tests = 1_000_usize;
    let pack_types = vec![
        PackType::NextFit,
        PackType::RandomFit,
        PackType::FirstFit,
        PackType::BestFit,
        PackType::WorstFit,
    ];
    let distributions_type = vec![
        RandDistType::Uniform,
        RandDistType::Harmonic,
        RandDistType::DoubleHarmonic,
        RandDistType::Geometric,
    ];

    let filename = "data/data.csv".to_string();
    let mut file = File::create(&filename).expect("Unable to create file");
    writeln!(
        file,
        "distributions,{}",
        pack_types
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(",")
    )
    .expect("Unable to wrote to file");

    for dist_type in &distributions_type {
        let mut row = vec![format!("{}", dist_type)];
        row.append(
            &mut pack_types
                .par_iter()
                .map(|pack_type| {
                    let avg = (0..num_of_tests)
                        .into_par_iter()
                        .map(|_| experiment(pack_type, dist_type))
                        .sum::<f64>()
                        / num_of_tests as f64;
                    format!("{}", avg)
                })
                .collect::<Vec<_>>(),
        );
        writeln!(file, "{}", row.join(",")).expect("Unable to write to file");
    }
}

fn main() {
    let now = std::time::Instant::now();
    experiments();
    println!("Time elapsed: {} s", now.elapsed().as_secs_f64());
}
