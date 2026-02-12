//! Visualization Module
//!
//! This module provides utilities for visualizing market data.
//! Currently provides text-based visualization, with plans for graphical output.

use crate::types::{OrderBook, Trade};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::fmt::Write;

/// Convert a Decimal to a non-negative f64, clamped at 0.0
fn decimal_to_f64(d: Decimal) -> f64 {
    d.to_f64().unwrap_or(0.0).max(0.0)
}

/// Print order book in text format
pub fn print_orderbook(orderbook: &OrderBook, levels: usize) {
    println!("=== Order Book ===");
    println!("Timestamp: {}", orderbook.timestamp);
    println!("\nAsks (Sell Orders):");

    // Display asks in reverse order (highest first) for natural book layout
    for ask in orderbook.asks.iter().take(levels).collect::<Vec<_>>().iter().rev() {
        let bar_len = (decimal_to_f64(ask.quantity) * 2.0) as usize;
        let bar = "█".repeat(bar_len);
        println!("  ${:<12} | {bar} {}", ask.price, ask.quantity);
    }

    println!("{}", "─".repeat(50));

    println!("Bids (Buy Orders):");
    for bid in orderbook.bids.iter().take(levels) {
        let bar_len = (decimal_to_f64(bid.quantity) * 2.0) as usize;
        let bar = "█".repeat(bar_len);
        println!("  ${:<12} | {bar} {}", bid.price, bid.quantity);
    }
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
#[must_use]
pub fn ascii_depth_chart(orderbook: &OrderBook, depth: usize) -> String {
    let mut output = String::new();

    if orderbook.bids.is_empty() && orderbook.asks.is_empty() {
        return "No data".to_string();
    }

    let display_levels = depth.max(1);

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
    let _ = writeln!(output, "Max Volume: {max_qty}\n");

    // Simple bar chart
    for level in orderbook.asks.iter().rev().take(display_levels) {
        let ratio = if max_qty > Decimal::ZERO {
            decimal_to_f64(level.quantity / max_qty)
        } else {
            0.0
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let bars = ((ratio * 20.0) as usize).max(1);
        let _ = writeln!(output, "${:<10} ASK │{}", level.price, "▓".repeat(bars));
    }

    let _ = writeln!(output, "{}", "─".repeat(40));

    for level in orderbook.bids.iter().take(display_levels) {
        let ratio = if max_qty > Decimal::ZERO {
            decimal_to_f64(level.quantity / max_qty)
        } else {
            0.0
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let bars = ((ratio * 20.0) as usize).max(1);
        let _ = writeln!(output, "${:<10} BID │{}", level.price, "▓".repeat(bars));
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
