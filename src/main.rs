use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("../data.csv");

    let mut csv_reader = match file {
        Ok(f) => ReaderBuilder::new().has_headers(true).from_reader(f),
        Err(e) => panic!("Failed to open file: {}", e),
    };

    for result in csv_reader.records() {
        let record = result?;

        println!("{:?}", record);
    }
    Ok(())
}
