use crate::utils::{csv_writer, detect_delimiter, input_reader};
use csv::ReaderBuilder;
use std::error::Error;
use std::io::{self, BufRead, Cursor, Read};

#[allow(unused_mut)]
pub fn stack(files: &[String]) -> Result<(), Box<dyn Error>> {
    let stdin_count = files.iter().filter(|s| *s == "-").count();
    if stdin_count > 1 {
        return Err(
            "Cannot use '-' for more than one input; at most one piped stream is supported".into(),
        );
    }

    // Pure stdin mode — single stream, nothing to cross-validate
    if files.is_empty() {
        let mut reader = input_reader(None);
        let delimiter = detect_delimiter(&mut *reader)?;
        let mut csv = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(delimiter as u8)
            .from_reader(reader);
        let headers = csv.headers()?.clone();

        let stdout = io::stdout();
        let mut writer = csv_writer(stdout.lock(), delimiter);
        writer.write_record(headers.iter().collect::<Vec<_>>())?;
        for result in csv.records() {
            let record = result?;
            writer.write_record(record.iter().collect::<Vec<_>>())?;
        }
        writer.flush()?;
        return Ok(());
    }

    // Buffer stdin content upfront if '-' is used, so we can replay it multiple
    // times during validation and output.
    let stdin_content: Option<Vec<u8>> = if stdin_count == 1 {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        Some(buf)
    } else {
        None
    };

    // Helper: get a fresh reader for the i-th input
    let reader_at = |i: usize| -> Box<dyn BufRead> {
        if files[i] == "-" {
            // Each call gets its own cursor over the shared buffer
            Box::new(Cursor::new(stdin_content.as_ref().unwrap().clone()))
        } else {
            input_reader(Some(&files[i]))
        }
    };

    // ── Pass 1: validate all headers match before writing anything ──

    let delimiter = {
        let mut r = reader_at(0);
        detect_delimiter(&mut *r)?
    };

    let headers = {
        let mut r = reader_at(0);
        let mut csv = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(delimiter as u8)
            .from_reader(r);
        csv.headers()?.clone()
    };

    for i in 1..files.len() {
        let file_delimiter = {
            let mut r = reader_at(i);
            detect_delimiter(&mut *r)?
        };

        if file_delimiter != delimiter {
            return Err(format!(
                "Files have different delimiters: '{}' vs '{}'",
                delimiter, file_delimiter
            )
            .into());
        }

        let file_headers = {
            let mut r = reader_at(i);
            let mut csv = ReaderBuilder::new()
                .has_headers(true)
                .delimiter(delimiter as u8)
                .from_reader(r);
            csv.headers()?.clone()
        };

        if file_headers.len() != headers.len() {
            return Err(format!(
                "Files have different number of columns: {} vs {}",
                headers.len(),
                file_headers.len()
            )
            .into());
        }

        for (j, (h1, h2)) in headers.iter().zip(file_headers.iter()).enumerate() {
            if h1 != h2 {
                return Err(format!(
                    "Headers don't match at position {}: '{}' vs '{}'",
                    j, h1, h2
                )
                .into());
            }
        }
    }

    // ── Pass 2: all valid, write output ──

    let stdout = io::stdout();
    let mut writer = csv_writer(stdout.lock(), delimiter);
    writer.write_record(headers.iter().collect::<Vec<_>>())?;

    for i in 0..files.len() {
        let mut r = reader_at(i);
        let mut csv = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(delimiter as u8)
            .from_reader(r);
        // Consume header (already validated above)
        csv.headers()?;
        for result in csv.records() {
            let record = result?;
            writer.write_record(record.iter().collect::<Vec<_>>())?;
        }
    }

    writer.flush()?;
    Ok(())
}
