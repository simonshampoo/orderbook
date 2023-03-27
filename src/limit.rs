use std::collections::LinkedList;
use rbtree::RBTree;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use crate::order::Order;

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

impl Eq for Limit {}

impl Ord for Limit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
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
