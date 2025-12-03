use crate::utils::{detect_delimiter, input_reader};
use csv::ReaderBuilder;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::error::Error;

pub fn sample_rows(
    path: Option<&str>,
    n: usize,
    seed: u64,
    include_header: bool,
) -> Result<(), Box<dyn Error>> {
    let mut reader = input_reader(path);
    let delimiter = detect_delimiter(&mut *reader)?;

    let mut csv = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .from_reader(reader);

    let headers = csv.headers()?.clone();

    // Print headers if requested
    if include_header {
        println!(
            "{}",
            headers
                .iter()
                .collect::<Vec<_>>()
                .join(&delimiter.to_string())
        );
    }

    // Use reservoir sampling with seeded RNG
    reservoir_sample(&mut csv, n, seed, &delimiter)?;

    Ok(())
}

// Reservoir sampling: guarantees exactly k samples with equal probability
// Uses seeded RNG for reproducible results
fn reservoir_sample(
    csv: &mut csv::Reader<Box<dyn std::io::BufRead>>,
    k: usize,
    seed: u64,
    delimiter: &char,
) -> Result<(), Box<dyn Error>> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut reservoir: Vec<csv::StringRecord> = Vec::with_capacity(k);

    let mut i = 0;
    for result in csv.records() {
        let record = result?;

        if i < k {
            // Fill reservoir with first k items
            reservoir.push(record);
        } else {
            // Randomly replace elements with decreasing probability
            let j = rng.gen_range(0..=i);
            if j < k {
                reservoir[j] = record;
            }
        }
        i += 1;
    }

    // Print sampled rows
    for record in reservoir {
        println!(
            "{}",
            record
                .iter()
                .collect::<Vec<_>>()
                .join(&delimiter.to_string())
        );
    }

    if i < k && i > 0 {
        eprintln!(
            "Warning: Requested {} rows but CSV only has {} rows. Returned all rows.",
            k, i
        );
    }

    Ok(())
}
