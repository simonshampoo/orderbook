use std::fs::File;
use csv::ReaderBuilder;
use std::error::Error;

pub struct MarketReader {
    path: String,
}

impl MarketReader {
    pub fn new(path: String) -> Self {
        Self { path }
    }
    pub fn read_market_data(&self) -> Result<(), Box<dyn Error>>{
        let file = File::open(&self.path);

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
}
