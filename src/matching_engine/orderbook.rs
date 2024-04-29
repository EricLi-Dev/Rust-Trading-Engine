use std::collections::HashMap;

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }
    
    // Add order to the price level in order book
    pub fn add_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        limit.add_order(order);
                    },
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }

            BidOrAsk::Ask => { 
                match self.asks.get_mut(&price) {
                    Some(limit) => {
                        limit.add_order(order);
                    }, 
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            }
        }
    }
}

// Price as a custom struct
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64; // cast float to int
        let fractional = ((price % 1.0) * scalar as f64) as u64; // fractional part of price

        Price {
            integral,
            fractional,
            scalar,
        }
    }
}

// Bucket of orders per price level in the order book
#[derive(Debug)]
pub struct Limit {
    price: Price,       // price level of the limit
    orders: Vec<Order>, // vector of Orders at the limit
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order){
        self.orders.push(order);
    }
}

// Order as a custom struct
#[derive(Debug)]
pub struct Order {
    size: f64,              // quantity size of the order
    bid_or_ask: BidOrAsk,   // Bid or Ask order
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { size, bid_or_ask}
    }
}
