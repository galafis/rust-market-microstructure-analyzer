use anyhow::Result;
use market_microstructure_analyzer::*;
use rust_decimal_macros::dec;

fn main() -> Result<()> {
    println!("=== Market Microstructure Analyzer - Order Book Analysis ===\n");

    // Create sample order book
    let orderbook = OrderBook {
        bids: vec![
            Level {
                price: dec!(50000.00),
                quantity: dec!(1.5),
            },
            Level {
                price: dec!(49999.50),
                quantity: dec!(2.3),
            },
            Level {
                price: dec!(49999.00),
                quantity: dec!(0.8),
            },
        ],
        asks: vec![
            Level {
                price: dec!(50001.00),
                quantity: dec!(1.2),
            },
            Level {
                price: dec!(50001.50),
                quantity: dec!(1.8),
            },
            Level {
                price: dec!(50002.00),
                quantity: dec!(2.5),
            },
        ],
        timestamp: 1696435200,
    };

    println!("📊 Order Book Analysis:");
    println!("\n  Bids (Buy Orders):");
    for (i, bid) in orderbook.bids.iter().enumerate() {
        println!("    Level {}: {} @ ${}", i + 1, bid.quantity, bid.price);
    }

    println!("\n  Asks (Sell Orders):");
    for (i, ask) in orderbook.asks.iter().enumerate() {
        println!("    Level {}: {} @ ${}", i + 1, ask.quantity, ask.price);
    }

    // Calculate spread
    let best_bid = &orderbook.bids[0].price;
    let best_ask = &orderbook.asks[0].price;
    let spread = best_ask - best_bid;
    let spread_pct = (spread / best_bid) * dec!(100);

    println!("\n  📈 Metrics:");
    println!("    Best Bid: ${}", best_bid);
    println!("    Best Ask: ${}", best_ask);
    println!("    Spread: ${} ({}%)", spread, spread_pct);

    println!("\n=== Analysis Complete ===");
    Ok(())
}
