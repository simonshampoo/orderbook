use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use orderbook::market_reader::MarketReader;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("../data.csv");
    let mr = MarketReader::new("../data.csv".to_string());

    mr.read_market_data(); 
        
    Ok(())
}
