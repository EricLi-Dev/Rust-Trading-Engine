mod matching_engine;
use matching_engine::orderbook::{BidOrAsk, Order, OrderBook};
use matching_engine::engine::{MatchingEngine, TradingPair};

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    
    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);

    let sell_order_from_joe = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(20.0, sell_order_from_joe);
    println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let btc_pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(btc_pair.clone());

    let buy_order_from_joe = Order::new(BidOrAsk::Bid, 6.5);
    engine.place_limit_order(btc_pair, 10.000, buy_order_from_joe).unwrap();

    let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    // engine.place_limit_order(eth_pair, 10.000, buy_order_from_joe).unwrap();
}
