// headers:
// exchange,symbol,timestamp,local_timestamp,is_snapshot,side,price,amount
// ["deribit", "BTC-PERP", "1585699217584000", "1585699217598331", "false", "bid", "6325", "550"]
// https://docs.tardis.dev/downloadable-csv-files#incremental_book_l2
struct Order<'a> {
    symbol: &'a str,
    msg_arrival_time: u32,
    side: bool,
    price: u8,
    amount: u16,
    next_order: Option<Box<Order<'a>>>,
    prev_order: Option<Box<Order<'a>>>,
}

// red black tree
struct Limit<'a> {
    limit_price: u8, 
    limit_size: u8, 
    total_vol: u8,
    left_tick: Option<Box<Limit>>, // lower price limit
    right_tick: Option<Box<Limit>>, // higher price limit
    head: Option<Box<Order<'a>>>,
    tail: Option<&'a mut Order<'a>>,
}

impl<'a> Limit<'a> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, order: Order) {
        let mut new_order = Box::new(order);
        new_order.next_order
    }
}

struct Orderbook<'a> {
    buy_tree: &'a Limit,
    sell_tree: &'a Limit,
    lowest_sell: &'a Limit, 
    highest_buy: &'a Limit,
} 


