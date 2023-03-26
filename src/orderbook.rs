// headers:
// exchange,symbol,timestamp,local_timestamp,is_snapshot,side,price,amount
// ["deribit", "BTC-PERP", "1585699217584000", "1585699217598331", "false", "bid", "6325", "550"]
// https://docs.tardis.dev/downloadable-csv-files#incremental_book_l2
//
use rb_tree::RBTree;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Deserialize, Serialize)]
struct Order<'a> {
    order_id: u8,

    symbol: &'a str,
    timestamp: u32,
    local_timestamp: u32,
    side: &'a str,
    price: u8,
    amount: u16,

    next_order: Option<Box<Order<'a>>>,
    prev_order: Option<Box<Order<'a>>>,

    #[serde(skip)]
    exchange: &'a str,
    is_snapshot: bool,
}

impl<'a> PartialOrd for Order<'a> {
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

impl<'a> PartialEq for Order<'a> {
    fn eq(&self, other: &Order) -> bool {
        self.price == other.price
    }
}

// red black tree
#[derive(Debug, Deserialize, Serialize)]
struct Limit<'a, 'de: 'a> {
    limit_price: u8,
    limit_size: u8,
    total_vol: u8,
    // left_tick: Option<Box<Limit>>, // lower price limit
    // right_tick: Option<Box<Limit>>, // higher price limit
    head: Option<Box<Order<'a>>>,
    tail: Option<Box<Order<'a>>>,
}

// hold up, this might change depending on the side
impl<'a> PartialOrd for Limit<'a> {
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

impl<'a> PartialEq for Limit<'a> {
    fn eq(&self, other: &Limit) -> bool {
        self.limit_price == other.limit_price
    }
}

impl<'a> Limit<'a> {
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

struct Orderbook<'a> {
    buy_tree: RBTree<&'a Limit<'a>>,
    sell_tree: RBTree<&'a Limit<'a>>,
    lowest_sell: &'a Limit<'a>,
    highest_buy: &'a Limit<'a>,
}

impl<'a> Orderbook<'a> {
    pub fn new(lowest_sell: &'a Limit, highest_buy: &'a Limit) -> Self {
        Self {
            buy_tree: RBTree::<&'a Limit>::new(),
            sell_tree: RBTree::<&'a Limit>::new(),
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
