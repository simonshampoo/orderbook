// headers:
// exchange,symbol,timestamp,local_timestamp,is_snapshot,side,price,amount
// ["deribit", "BTC-PERP", "1585699217584000", "1585699217598331", "false", "bid", "6325", "550"]
// https://docs.tardis.dev/downloadable-csv-files#incremental_book_l2
//
use std::collections::LinkedList;
use rbtree::RBTree;
use crate::order::Order;
use crate::limit::Limit;

pub struct Orderbook<'a> {
    buy_tree: RBTree<&'a Limit, LinkedList<Order>>,
    sell_tree: RBTree<&'a Limit, LinkedList<Order>>,
    lowest_sell: &'a Limit,
    highest_buy: &'a Limit,
}

impl<'a> Orderbook<'a> {
    pub fn new(lowest_sell: &'a Limit, highest_buy: &'a Limit) -> Self {
        Self {
            buy_tree: RBTree::<& Limit, LinkedList<Order>>::new(),
            sell_tree: RBTree::<& Limit, LinkedList<Order>>::new(),
            lowest_sell,
            highest_buy,
        }
    }

    pub fn add_order_to_limit(order: Order) {
        todo!("add {:?} to limit", order);
    }

    // wont be used?
    pub fn cancel_order_at_limit(order: &Order) {
        todo!("cancel {:?} at limit", order);
    }

    // matching engine shit
    pub fn execute_order_at_limit() {
        todo!("uhh so we're just parsing the data rn")
    }
}
