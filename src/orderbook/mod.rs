//! Order Book Analysis Module
//!
//! This module provides functionality for analyzing order book data,
//! including spread calculation, imbalance detection, and depth analysis.

use crate::types::{Level, OrderBook};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Calculate the bid-ask spread
///
/// # Arguments
/// * `orderbook` - The order book to analyze
///
/// # Returns
/// A tuple containing (spread, spread_percentage)
pub fn calculate_spread(orderbook: &OrderBook) -> Option<(Decimal, Decimal)> {
    if orderbook.bids.is_empty() || orderbook.asks.is_empty() {
        return None;
    }

    let best_bid = &orderbook.bids[0].price;
    let best_ask = &orderbook.asks[0].price;
    let spread = best_ask - best_bid;
    let spread_pct = (spread / best_bid) * dec!(100);

    Some((spread, spread_pct))
}

/// Calculate order book imbalance
///
/// Imbalance ratio: (bid_volume - ask_volume) / (bid_volume + ask_volume)
/// - Positive values indicate more buying pressure
/// - Negative values indicate more selling pressure
///
/// # Arguments
/// * `orderbook` - The order book to analyze
/// * `depth` - Number of levels to consider (None for all levels)
///
/// # Returns
/// The imbalance ratio between -1.0 and 1.0
pub fn calculate_imbalance(orderbook: &OrderBook, depth: Option<usize>) -> Decimal {
    let depth = depth.unwrap_or(usize::MAX);

    let total_bid_volume: Decimal = orderbook.bids.iter().take(depth).map(|l| l.quantity).sum();

    let total_ask_volume: Decimal = orderbook.asks.iter().take(depth).map(|l| l.quantity).sum();

    let total = total_bid_volume + total_ask_volume;
    if total == dec!(0) {
        return dec!(0);
    }

    (total_bid_volume - total_ask_volume) / total
}

/// Get the best bid price
pub fn best_bid(orderbook: &OrderBook) -> Option<Decimal> {
    orderbook.bids.first().map(|l| l.price)
}

/// Get the best ask price
pub fn best_ask(orderbook: &OrderBook) -> Option<Decimal> {
    orderbook.asks.first().map(|l| l.price)
}

/// Get the mid price (average of best bid and ask)
pub fn mid_price(orderbook: &OrderBook) -> Option<Decimal> {
    match (best_bid(orderbook), best_ask(orderbook)) {
        (Some(bid), Some(ask)) => Some((bid + ask) / dec!(2)),
        _ => None,
    }
}

/// Calculate total volume at a specific depth
pub fn total_volume(levels: &[Level], depth: Option<usize>) -> Decimal {
    let depth = depth.unwrap_or(usize::MAX);
    levels.iter().take(depth).map(|l| l.quantity).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Level, OrderBook};

    fn sample_orderbook() -> OrderBook {
        OrderBook {
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
        }
    }

    #[test]
    fn test_calculate_spread() {
        let ob = sample_orderbook();
        let (spread, spread_pct) = calculate_spread(&ob).unwrap();

        assert_eq!(spread, dec!(1.00));
        assert!(spread_pct > dec!(0.0019) && spread_pct < dec!(0.0021));
    }

    #[test]
    fn test_calculate_spread_empty() {
        let empty_ob = OrderBook {
            bids: vec![],
            asks: vec![],
            timestamp: 0,
        };

        assert!(calculate_spread(&empty_ob).is_none());
    }

    #[test]
    fn test_calculate_imbalance() {
        let ob = sample_orderbook();
        let imbalance = calculate_imbalance(&ob, None);

        // Total bids: 1.5 + 2.3 + 0.8 = 4.6
        // Total asks: 1.2 + 1.8 + 2.5 = 5.5
        // Imbalance: (4.6 - 5.5) / (4.6 + 5.5) = -0.9 / 10.1 ≈ -0.089
        assert!(imbalance < dec!(0));
        assert!(imbalance > dec!(-0.1) && imbalance < dec!(-0.08));
    }

    #[test]
    fn test_calculate_imbalance_with_depth() {
        let ob = sample_orderbook();
        let imbalance = calculate_imbalance(&ob, Some(1));

        // Only first level: bids 1.5, asks 1.2
        // Imbalance: (1.5 - 1.2) / (1.5 + 1.2) = 0.3 / 2.7 ≈ 0.111
        assert!(imbalance > dec!(0));
        assert!(imbalance > dec!(0.1) && imbalance < dec!(0.12));
    }

    #[test]
    fn test_best_prices() {
        let ob = sample_orderbook();

        assert_eq!(best_bid(&ob), Some(dec!(50000.00)));
        assert_eq!(best_ask(&ob), Some(dec!(50001.00)));
        assert_eq!(mid_price(&ob), Some(dec!(50000.50)));
    }

    #[test]
    fn test_total_volume() {
        let ob = sample_orderbook();

        let bid_vol = total_volume(&ob.bids, None);
        let ask_vol = total_volume(&ob.asks, None);

        assert_eq!(bid_vol, dec!(4.6));
        assert_eq!(ask_vol, dec!(5.5));
    }

    #[test]
    fn test_total_volume_with_depth() {
        let ob = sample_orderbook();

        let bid_vol = total_volume(&ob.bids, Some(2));
        assert_eq!(bid_vol, dec!(3.8)); // 1.5 + 2.3
    }
}
