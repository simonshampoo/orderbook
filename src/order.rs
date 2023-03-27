use serde::{Deserialize, Serialize};
use std::cmp::Ordering;


#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    order_id: u8,

    symbol: String,
    timestamp: u32,
    local_timestamp: u32,
    side: String, 
    price: u8,
    amount: u16,
    next_order: Option<Box<Order>>,
    prev_order: Option<Box<Order>>,

    #[serde(skip)]
    exchange: String, 
    is_snapshot: bool,
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Order) -> Option<Ordering> {
        if self.price > other.price {
            Some(Ordering::Greater)
        } else if self.price < other.price {
            Some(Ordering::Less)
        } else {
            if self.timestamp > other.timestamp {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Order) -> bool {
        self.price == other.price
    }
}
