// headers:
// exchange,symbol,timestamp,local_timestamp,is_snapshot,side,price,amount
// ["deribit", "BTC-PERP", "1585699217584000", "1585699217598331", "false", "bid", "6325", "550"]
// https://docs.tardis.dev/downloadable-csv-files#incremental_book_l2
//
use rb_tree::RBTree;
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

// red black tree
#[derive(Debug, Deserialize, Serialize)]
pub struct Limit {
    limit_price: u8,
    limit_size: u8,
    total_vol: u8,
    // left_tick: Option<Box<Limit>>, // lower price limit
    // right_tick: Option<Box<Limit>>, // higher price limit
    head: Option<Box<Order>>,
    tail: Option<Box<Order>>,
}

// hold up, this might change depending on the side
impl PartialOrd for Limit {
    fn partial_cmp(&self, other: &Limit) -> Option<Ordering> {
        if self.limit_price > other.limit_price {
            Some(Ordering::Greater)
        } else if self.limit_price < other.limit_price {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl PartialEq for Limit {
    fn eq(&self, other: &Limit) -> bool {
        self.limit_price == other.limit_price
    }
}

impl Limit {
    pub fn new() -> Self {
        Self {
            limit_price: 0,
            limit_size: 0,
            total_vol: 0,
            head: None,
            tail: None,
        }
    }
}

pub struct Orderbook<'a> {
    buy_tree: RBTree<&'a Limit>,
    sell_tree: RBTree<&'a Limit>,
    lowest_sell: &'a Limit,
    highest_buy: &'a Limit,
}

impl<'a> Orderbook<'a> {
    pub fn new(lowest_sell: &'a Limit, highest_buy: &'a Limit) -> Self {
        Self {
            buy_tree: RBTree::<& Limit>::new(),
            sell_tree: RBTree::<& Limit>::new(),
            lowest_sell,
            highest_buy,
        }
    }

    pub fn add_order_to_limit(order: Order) {
        todo!("add order to limit");
    }

    // wont be used?
    pub fn cancel_order_at_limit(order: &Order) {
        todo!("cancel order at limit");
    }

    // matching engine shit
    pub fn execute_order_at_limit() {
        todo!("uhh so we're just parsing the data rn")
    }
}
