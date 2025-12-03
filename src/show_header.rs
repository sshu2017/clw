use crate::utils::{detect_delimiter, input_reader};
use colored::Colorize;
use csv::ReaderBuilder;
use std::error::Error;

pub fn show_header(path: Option<&str>) -> Result<(), Box<dyn Error>> {
    let mut reader = input_reader(path);

    let delimiter = detect_delimiter(&mut *reader)?;
    let mut csv = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .from_reader(reader);

    let headers = csv.headers().expect("Cannot read headers");

    println!(
        "{:>8}: {:<8}",
        "Index".green().bold(),
        "ColName".green().bold()
    );
    for (i, h) in headers.iter().enumerate() {
        println!("{:>8}: {:<8}", i, h);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use csv::ReaderBuilder;
    use std::io::Cursor;

    #[test]
    fn test_csv_headers_parsing_comma() {
        let data = "name,age,city\nAlice,30,NYC\nBob,25,LA\n";
        let reader = Cursor::new(data);
        let mut csv = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(reader);

        let headers = csv.headers().unwrap();
        assert_eq!(headers.len(), 3);
        assert_eq!(headers.get(0), Some("name"));
        assert_eq!(headers.get(1), Some("age"));
        assert_eq!(headers.get(2), Some("city"));
    }

    #[test]
    fn test_csv_headers_parsing_pipe() {
        let data = "id|product|price\n1|Widget|9.99\n2|Gadget|19.99\n";
        let reader = Cursor::new(data);
        let mut csv = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b'|')
            .from_reader(reader);

        let headers = csv.headers().unwrap();
        assert_eq!(headers.len(), 3);
        assert_eq!(headers.get(0), Some("id"));
        assert_eq!(headers.get(1), Some("product"));
        assert_eq!(headers.get(2), Some("price"));
    }

    #[test]
    fn test_csv_headers_with_spaces() {
        let data = "first name,last name,email address\nJohn,Doe,john@example.com\n";
        let reader = Cursor::new(data);
        let mut csv = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(reader);

        let headers = csv.headers().unwrap();
        assert_eq!(headers.len(), 3);
        assert_eq!(headers.get(0), Some("first name"));
        assert_eq!(headers.get(1), Some("last name"));
        assert_eq!(headers.get(2), Some("email address"));
    }
}
