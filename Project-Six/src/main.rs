use polars::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "your_file.csv.gz" with the path to your gzip-compressed CSV file.
    let file = File::open("your_file.csv.gz")?;
    let reader = BufReader::new(file);

    // Read the CSV file into a DataFrame
    let df = CsvReader::new(reader)
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    // Do something with the DataFrame
    println!("{:?}", df);

    Ok(())
}
