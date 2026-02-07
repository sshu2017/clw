use crate::utils::{csv_writer, detect_delimiter, input_reader};
use csv::ReaderBuilder;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::error::Error;
use std::io;

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

    // Create CSV writer for proper quoting
    let stdout = io::stdout();
    let mut writer = csv_writer(stdout.lock(), delimiter);

    // Print headers if requested
    if include_header {
        writer.write_record(headers.iter().collect::<Vec<_>>())?;
    }

    // Use reservoir sampling with seeded RNG
    reservoir_sample(&mut csv, n, seed, &mut writer)?;

    Ok(())
}

// Reservoir sampling: guarantees exactly k samples with equal probability
// Uses seeded RNG for reproducible results
fn reservoir_sample<W: io::Write>(
    csv: &mut csv::Reader<Box<dyn std::io::BufRead>>,
    k: usize,
    seed: u64,
    writer: &mut csv::Writer<W>,
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

    // Write sampled rows
    for record in reservoir {
        writer.write_record(record.iter().collect::<Vec<_>>())?;
    }

    if i < k && i > 0 {
        eprintln!(
            "Warning: Requested {} rows but CSV only has {} rows. Returned all rows.",
            k, i
        );
    }

    Ok(())
}
