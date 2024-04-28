use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }
    
    // Add order to the price level in order book
    fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let price = Price::new(price);

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

            BidOrAsk::Ask => { }
        }
    }
}

// Price as a custom struct
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
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
struct Limit {
    price: Price,       // price level of the limit
    orders: Vec<Order>, // vector of Orders at the limit
}

impl Limit {
    fn new(price: Price) -> Limit {
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
struct Order {
    size: f64,              // quantity size of the order
    bid_or_ask: BidOrAsk,   // Bid or Ask order
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { size, bid_or_ask}
    }
}

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    //let sell_order = Order::new(BidOrAsk::Ask, 2.45);
    
    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);
    println!("{:?}", orderbook);
}
