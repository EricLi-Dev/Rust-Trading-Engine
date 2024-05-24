use std::collections::HashMap;
use rust_decimal::prelude::*;

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.bid_or_ask {
            BidOrAsk::Bid => self.ask_limits(),
            BidOrAsk::Ask => self.bid_limits(),
        };

        for limit_order in self.ask_limits() {
            limit_order.fill_order(market_order);

            if market_order.is_filled() {
                break;
            }
        }
    }
    
    // Sorted lowest sell prices
    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
       let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
       limits.sort_by(|a, b| a.price.cmp(&b.price));
       limits        
    }
    
    // Sorted highest buy prices
    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
       let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
       limits.sort_by(|a, b| a.price.cmp(&b.price));
       limits        
    }

    // Add order to the price level in order book
    pub fn add_limit_order(&mut self, price: Decimal, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },

            BidOrAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }
}

// Bucket of orders per price level in the order book
#[derive(Debug)]
pub struct Limit {
    price: Decimal,       // price level of the limit
    orders: Vec<Order>, // vector of Orders at the limit
}

impl Limit {
    pub fn new(price: Decimal) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }
    
    // Gets total volume of Limit sitting at Price level
    fn total_volume(&self) -> f64 {
        self.orders
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

// Order as a custom struct
#[derive(Debug)]
pub struct Order {
    size: f64,            // quantity size of the order
    bid_or_ask: BidOrAsk, // Bid or Ask order
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { size, bid_or_ask }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    
    #[test]
    // Fill market order at cheapest price
    fn orderbook_fill_market_order_ask() {
        // Make Orderbook
        let mut orderbook = OrderBook::new();

        // Make different price levels
        orderbook.add_limit_order(dec!(500), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(200), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(300), Order::new(BidOrAsk::Ask, 10.0));
        
        // Place Market Order
        let mut market_order = Order::new(BidOrAsk::Bid, 10.0);
        orderbook.fill_market_order(&mut market_order);
        
        let ask_limits = orderbook.ask_limits();
        let matched_limit = ask_limits.get(0).unwrap();

        // Check top of ask limits is cheapest
        assert_eq!(matched_limit.price, dec!(100));
        // Check market order was filled
        assert_eq!(market_order.is_filled(), true);

        let matched_order = matched_limit.orders.get(0).unwrap();
        // Check the cheapest order at top of ASK book was filled
        assert_eq!(matched_order.is_filled(), true);
    }
    #[test]
    fn limit_total_volume() {
        // Make new price level
        let price = dec!(10000);
        let mut limit = Limit::new(price);

        // Add buy Limit Orders of 100 at price level
        let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        assert_eq!(limit.total_volume(), 200.0);
    }

    #[test]
    fn limit_order_multi_fill() {
        // Make new price level
        let price = dec!(10000);
        let mut limit = Limit::new(price);

        // Add buy Limit Orders of 100 at price level
        let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        // Fill sell Market Order of 99 at price level
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 199.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true); 
        assert_eq!(limit.orders.get(1).unwrap().size, 1.0);
    }

    #[test]
    fn limit_order_single_fill() {
        // Make new price level
        let price = dec!(10000);
        let mut limit = Limit::new(price);

        // Add buy Limit Order of 100 at price level
        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        // Fill sell Market Order of 99 at price level
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0); // remaining size of Order at Limit is
                                                            // 1
    }
}
