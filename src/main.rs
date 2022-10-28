mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    let mut orderbook = Orderbook::new();
    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(20.0, sell_order);

    // println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    engine.place_limit_order(pair, 10.000, buy_order).unwrap();
}
