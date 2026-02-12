//! Metrics Calculation Module
//!
//! This module provides advanced metrics calculations for market microstructure analysis.

use crate::types::{OrderBook, Trade};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;

/// Volume Profile data structure
#[derive(Debug, Clone)]
pub struct VolumeProfile {
    /// Price levels and their volumes
    pub levels: HashMap<Decimal, Decimal>,
    /// Point of Control (price with highest volume)
    pub poc: Option<Decimal>,
    /// Value Area High
    pub vah: Option<Decimal>,
    /// Value Area Low
    pub val: Option<Decimal>,
}

/// Calculate volume profile from trades
///
/// # Arguments
/// * `trades` - List of executed trades
/// * `tick_size` - Price tick size for grouping
///
/// # Returns
/// VolumeProfile structure with POC, VAH, VAL
#[must_use]
pub fn calculate_volume_profile(trades: &[Trade], tick_size: Decimal) -> VolumeProfile {
    let mut levels: HashMap<Decimal, Decimal> = HashMap::new();

    // Group trades by price level
    for trade in trades {
        let price_level = (trade.price / tick_size).round() * tick_size;
        *levels.entry(price_level).or_insert(dec!(0)) += trade.quantity;
    }

    if levels.is_empty() {
        return VolumeProfile {
            levels,
            poc: None,
            vah: None,
            val: None,
        };
    }

    // Find Point of Control (highest volume)
    let poc = levels
        .iter()
        .max_by_key(|(_, &vol)| vol)
        .map(|(&price, _)| price);

    // Calculate Value Area (70% of volume)
    let total_volume: Decimal = levels.values().sum();
    let value_area_volume = total_volume * dec!(0.70);

    // Sort levels by volume descending to build value area from highest-volume levels
    let mut sorted_by_volume: Vec<_> = levels.iter().collect();
    sorted_by_volume.sort_by(|a, b| b.1.cmp(a.1));

    // Find VAH and VAL by accumulating highest-volume price levels until 70% is reached
    let (vah, val) = if let Some(poc_price) = poc {
        let mut accumulated = dec!(0);
        let mut low = poc_price;
        let mut high = poc_price;

        for (&price, &volume) in &sorted_by_volume {
            if accumulated >= value_area_volume {
                break;
            }
            accumulated += volume;
            if price > high {
                high = price;
            }
            if price < low {
                low = price;
            }
        }
        (Some(high), Some(low))
    } else {
        (None, None)
    };

    VolumeProfile {
        levels,
        poc,
        vah,
        val,
    }
}

/// Calculate Delta Volume (buying pressure - selling pressure)
///
/// # Arguments
/// * `trades` - List of executed trades
///
/// # Returns
/// Net delta (positive = buying pressure, negative = selling pressure)
#[must_use]
pub fn calculate_delta(trades: &[Trade]) -> Decimal {
    trades
        .iter()
        .map(|trade| {
            if trade.side == "buy" {
                trade.quantity
            } else {
                -trade.quantity
            }
        })
        .sum()
}

/// Calculate Cumulative Volume Delta over time
///
/// # Arguments
/// * `trades` - List of executed trades (should be sorted by timestamp)
///
/// # Returns
/// Vector of (timestamp, cumulative_delta) pairs
#[must_use]
pub fn calculate_cvd(trades: &[Trade]) -> Vec<(i64, Decimal)> {
    let mut cvd = dec!(0);
    let mut result = Vec::new();

    for trade in trades {
        if trade.side == "buy" {
            cvd += trade.quantity;
        } else {
            cvd -= trade.quantity;
        }
        result.push((trade.timestamp, cvd));
    }

    result
}

/// Calculate weighted mid price
///
/// Weights the mid price by the volumes at best bid and ask
#[must_use]
pub fn weighted_mid_price(orderbook: &OrderBook) -> Option<Decimal> {
    if orderbook.bids.is_empty() || orderbook.asks.is_empty() {
        return None;
    }

    let best_bid = &orderbook.bids[0];
    let best_ask = &orderbook.asks[0];

    let total_qty = best_bid.quantity + best_ask.quantity;
    if total_qty == dec!(0) {
        return None;
    }

    let weighted =
        (best_bid.price * best_ask.quantity + best_ask.price * best_bid.quantity) / total_qty;

    Some(weighted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Level;

    fn sample_trades() -> Vec<Trade> {
        vec![
            Trade {
                price: dec!(50000.0),
                quantity: dec!(1.0),
                side: "buy".to_string(),
                timestamp: 1000,
            },
            Trade {
                price: dec!(50000.0),
                quantity: dec!(0.5),
                side: "sell".to_string(),
                timestamp: 1001,
            },
            Trade {
                price: dec!(50001.0),
                quantity: dec!(2.0),
                side: "buy".to_string(),
                timestamp: 1002,
            },
            Trade {
                price: dec!(50000.0),
                quantity: dec!(1.5),
                side: "buy".to_string(),
                timestamp: 1003,
            },
        ]
    }

    #[test]
    fn test_calculate_delta() {
        let trades = sample_trades();
        let delta = calculate_delta(&trades);

        // buy: 1.0 + 2.0 + 1.5 = 4.5
        // sell: 0.5
        // delta: 4.5 - 0.5 = 4.0
        assert_eq!(delta, dec!(4.0));
    }

    #[test]
    fn test_calculate_cvd() {
        let trades = sample_trades();
        let cvd = calculate_cvd(&trades);

        assert_eq!(cvd.len(), 4);
        assert_eq!(cvd[0], (1000, dec!(1.0)));
        assert_eq!(cvd[1], (1001, dec!(0.5)));
        assert_eq!(cvd[2], (1002, dec!(2.5)));
        assert_eq!(cvd[3], (1003, dec!(4.0)));
    }

    #[test]
    fn test_volume_profile() {
        let trades = sample_trades();
        let profile = calculate_volume_profile(&trades, dec!(1.0));

        assert!(profile.poc.is_some());
        // POC should be at 50000.0 with highest volume (1.0 + 1.5 + 0.5 = 3.0)
        assert_eq!(profile.poc.unwrap(), dec!(50000.0));
    }

    #[test]
    fn test_weighted_mid_price() {
        let orderbook = OrderBook {
            bids: vec![Level {
                price: dec!(100.0),
                quantity: dec!(10.0),
            }],
            asks: vec![Level {
                price: dec!(101.0),
                quantity: dec!(5.0),
            }],
            timestamp: 1000,
        };

        let wmp = weighted_mid_price(&orderbook).unwrap();
        // (100 * 5 + 101 * 10) / 15 = (500 + 1010) / 15 = 1510 / 15 = 100.666...
        assert!(wmp > dec!(100.6) && wmp < dec!(100.7));
    }
}
