//! Visualization Module
//!
//! This module provides utilities for visualizing market data.
//! Currently provides text-based visualization, with plans for graphical output.

use crate::types::{OrderBook, Trade};
use rust_decimal::Decimal;

/// Print order book in text format
pub fn print_orderbook(orderbook: &OrderBook, levels: usize) {
    println!("=== Order Book ===");
    println!("Timestamp: {}", orderbook.timestamp);
    println!("\nAsks (Sell Orders):");

    for ask in orderbook.asks.iter().take(levels) {
        let bar =
            "█".repeat((ask.quantity.to_string().parse::<f64>().unwrap_or(0.0) * 2.0) as usize);
        println!("  ${:<12} | {} {}", ask.price, bar, ask.quantity);
    }

    println!("{}", "─".repeat(50));

    for bid in orderbook.bids.iter().take(levels) {
        let bar =
            "█".repeat((bid.quantity.to_string().parse::<f64>().unwrap_or(0.0) * 2.0) as usize);
        println!("  ${:<12} | {} {}", bid.price, bar, bid.quantity);
    }

    println!("\nBids (Buy Orders):");
}

/// Print trade tape
pub fn print_trades(trades: &[Trade], limit: usize) {
    println!("=== Trade Tape ===");
    println!(
        "{:<12} {:<12} {:<12} {:<10}",
        "Time", "Price", "Quantity", "Side"
    );
    println!("{}", "─".repeat(50));

    for trade in trades.iter().take(limit) {
        let side_symbol = if trade.side == "buy" {
            "🟢 BUY"
        } else {
            "🔴 SELL"
        };
        println!(
            "{:<12} ${:<11} {:<12} {}",
            trade.timestamp, trade.price, trade.quantity, side_symbol
        );
    }
}

/// Generate ASCII chart of order book depth
pub fn ascii_depth_chart(orderbook: &OrderBook, _height: usize) -> String {
    let mut output = String::new();

    if orderbook.bids.is_empty() && orderbook.asks.is_empty() {
        return "No data".to_string();
    }

    // Find max volume for scaling
    let max_bid_qty = orderbook
        .bids
        .iter()
        .map(|l| l.quantity)
        .max()
        .unwrap_or(Decimal::ZERO);
    let max_ask_qty = orderbook
        .asks
        .iter()
        .map(|l| l.quantity)
        .max()
        .unwrap_or(Decimal::ZERO);
    let max_qty = max_bid_qty.max(max_ask_qty);

    output.push_str("Order Book Depth:\n");
    output.push_str(&format!("Max Volume: {}\n\n", max_qty));

    // Simple bar chart
    for level in orderbook.asks.iter().rev().take(5) {
        let ratio = if max_qty > Decimal::ZERO {
            (level.quantity / max_qty)
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0)
        } else {
            0.0
        };
        let bars = ((ratio * 20.0) as usize).max(1);
        output.push_str(&format!("${:<10} ASK │{}\n", level.price, "▓".repeat(bars)));
    }

    output.push_str(&format!("{}\n", "─".repeat(40)));

    for level in orderbook.bids.iter().take(5) {
        let ratio = if max_qty > Decimal::ZERO {
            (level.quantity / max_qty)
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0)
        } else {
            0.0
        };
        let bars = ((ratio * 20.0) as usize).max(1);
        output.push_str(&format!("${:<10} BID │{}\n", level.price, "▓".repeat(bars)));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Level;
    use rust_decimal_macros::dec;

    #[test]
    fn test_ascii_depth_chart() {
        let orderbook = OrderBook {
            bids: vec![
                Level {
                    price: dec!(50000.0),
                    quantity: dec!(1.0),
                },
                Level {
                    price: dec!(49999.0),
                    quantity: dec!(2.0),
                },
            ],
            asks: vec![
                Level {
                    price: dec!(50001.0),
                    quantity: dec!(1.5),
                },
                Level {
                    price: dec!(50002.0),
                    quantity: dec!(0.5),
                },
            ],
            timestamp: 1000,
        };

        let chart = ascii_depth_chart(&orderbook, 10);
        assert!(chart.contains("Order Book Depth"));
        assert!(chart.contains("ASK"));
        assert!(chart.contains("BID"));
    }

    #[test]
    fn test_ascii_depth_chart_empty() {
        let empty_ob = OrderBook {
            bids: vec![],
            asks: vec![],
            timestamp: 0,
        };

        let chart = ascii_depth_chart(&empty_ob, 10);
        assert_eq!(chart, "No data");
    }
}
