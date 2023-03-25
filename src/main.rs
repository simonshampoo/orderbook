use std::error::Error;
use orderbook::market_reader::MarketReader;

fn main() -> Result<(), Box<dyn Error>> {
    let mr = MarketReader::new("../data.csv".to_string());

    let res = mr.read_market_data().unwrap(); 
    
    println!("{:?}", res);
    Ok(())
}
