//! Tape Reading Module
//!
//! This module provides functionality for analyzing trade flow (time & sales).

use crate::types::Trade;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Trade classification
#[derive(Debug, Clone, PartialEq)]
pub enum TradeType {
    /// Aggressive buy
    Buy,
    /// Aggressive sell
    Sell,
    /// Block trade (large trade)
    Block { side: String },
}

/// Analyze a single trade
#[must_use]
pub fn classify_trade(trade: &Trade, block_threshold: Decimal) -> TradeType {
    if trade.quantity >= block_threshold {
        TradeType::Block {
            side: trade.side.clone(),
        }
    } else if trade.side == "buy" {
        TradeType::Buy
    } else {
        TradeType::Sell
    }
}

/// Calculate buying vs selling pressure over a period
///
/// # Arguments
/// * `trades` - List of trades
///
/// # Returns
/// (buy_volume, sell_volume, net_volume)
#[must_use]
pub fn calculate_trade_pressure(trades: &[Trade]) -> (Decimal, Decimal, Decimal) {
    let buy_volume: Decimal = trades
        .iter()
        .filter(|t| t.side == "buy")
        .map(|t| t.quantity)
        .sum();

    let sell_volume: Decimal = trades
        .iter()
        .filter(|t| t.side == "sell")
        .map(|t| t.quantity)
        .sum();

    let net_volume = buy_volume - sell_volume;

    (buy_volume, sell_volume, net_volume)
}

/// Identify block trades
///
/// # Arguments
/// * `trades` - List of trades
/// * `threshold` - Minimum size to classify as block trade
///
/// # Returns
/// Vector of block trades
#[must_use]
pub fn identify_block_trades(trades: &[Trade], threshold: Decimal) -> Vec<Trade> {
    trades
        .iter()
        .filter(|t| t.quantity >= threshold)
        .cloned()
        .collect()
}

/// Calculate trade aggression ratio
///
/// Ratio of aggressive buys to total trades
/// High ratio (>0.6) indicates strong buying aggression
///
/// # Arguments
/// * `trades` - List of trades
///
/// # Returns
/// Aggression ratio between 0.0 and 1.0
#[must_use]
pub fn calculate_aggression_ratio(trades: &[Trade]) -> Decimal {
    if trades.is_empty() {
        return dec!(0.5);
    }

    let buy_count = trades.iter().filter(|t| t.side == "buy").count();
    let total_count = trades.len();

    Decimal::from(buy_count) / Decimal::from(total_count)
}

/// Detect trade clusters (rapid succession of trades)
///
/// # Arguments
/// * `trades` - List of trades (must be sorted by timestamp)
/// * `time_window` - Maximum time gap in seconds
/// * `min_cluster_size` - Minimum trades to form a cluster
///
/// # Returns
/// Vector of cluster start indices
#[must_use]
pub fn detect_trade_clusters(
    trades: &[Trade],
    time_window: i64,
    min_cluster_size: usize,
) -> Vec<usize> {
    let mut clusters = Vec::new();
    let mut cluster_start = 0;
    let mut cluster_count = 1;

    for i in 1..trades.len() {
        let time_diff = trades[i].timestamp - trades[i - 1].timestamp;

        if time_diff <= time_window {
            cluster_count += 1;
        } else {
            if cluster_count >= min_cluster_size {
                clusters.push(cluster_start);
            }
            cluster_start = i;
            cluster_count = 1;
        }
    }

    // Check final cluster
    if cluster_count >= min_cluster_size {
        clusters.push(cluster_start);
    }

    clusters
}

/// Calculate Volume-Weighted Average Price (VWAP)
///
/// # Arguments
/// * `trades` - List of trades
///
/// # Returns
/// VWAP or None if no trades
#[must_use]
pub fn calculate_vwap(trades: &[Trade]) -> Option<Decimal> {
    if trades.is_empty() {
        return None;
    }

    let total_value: Decimal = trades.iter().map(|t| t.price * t.quantity).sum();

    let total_volume: Decimal = trades.iter().map(|t| t.quantity).sum();

    if total_volume == dec!(0) {
        return None;
    }

    Some(total_value / total_volume)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_trades() -> Vec<Trade> {
        vec![
            Trade {
                price: dec!(50000.0),
                quantity: dec!(1.0),
                side: "buy".to_string(),
                timestamp: 1000,
            },
            Trade {
                price: dec!(50001.0),
                quantity: dec!(0.5),
                side: "sell".to_string(),
                timestamp: 1001,
            },
            Trade {
                price: dec!(50002.0),
                quantity: dec!(2.0),
                side: "buy".to_string(),
                timestamp: 1002,
            },
            Trade {
                price: dec!(50003.0),
                quantity: dec!(0.3),
                side: "sell".to_string(),
                timestamp: 1003,
            },
        ]
    }

    #[test]
    fn test_classify_trade() {
        let trade1 = Trade {
            price: dec!(50000.0),
            quantity: dec!(1.0),
            side: "buy".to_string(),
            timestamp: 1000,
        };
        let trade2 = Trade {
            price: dec!(50000.0),
            quantity: dec!(10.0),
            side: "buy".to_string(),
            timestamp: 1000,
        };

        assert_eq!(classify_trade(&trade1, dec!(5.0)), TradeType::Buy);
        assert_eq!(
            classify_trade(&trade2, dec!(5.0)),
            TradeType::Block {
                side: "buy".to_string()
            }
        );
    }

    #[test]
    fn test_calculate_trade_pressure() {
        let trades = sample_trades();
        let (buy_vol, sell_vol, net_vol) = calculate_trade_pressure(&trades);

        assert_eq!(buy_vol, dec!(3.0)); // 1.0 + 2.0
        assert_eq!(sell_vol, dec!(0.8)); // 0.5 + 0.3
        assert_eq!(net_vol, dec!(2.2)); // 3.0 - 0.8
    }

    #[test]
    fn test_identify_block_trades() {
        let trades = sample_trades();
        let blocks = identify_block_trades(&trades, dec!(1.5));

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].quantity, dec!(2.0));
    }

    #[test]
    fn test_calculate_aggression_ratio() {
        let trades = sample_trades();
        let ratio = calculate_aggression_ratio(&trades);

        // 2 buys out of 4 trades = 0.5
        assert_eq!(ratio, dec!(0.5));
    }

    #[test]
    fn test_detect_trade_clusters() {
        let trades = vec![
            Trade {
                price: dec!(50000.0),
                quantity: dec!(1.0),
                side: "buy".to_string(),
                timestamp: 1000,
            },
            Trade {
                price: dec!(50000.0),
                quantity: dec!(1.0),
                side: "buy".to_string(),
                timestamp: 1001,
            },
            Trade {
                price: dec!(50000.0),
                quantity: dec!(1.0),
                side: "buy".to_string(),
                timestamp: 1002,
            },
            Trade {
                price: dec!(50000.0),
                quantity: dec!(1.0),
                side: "buy".to_string(),
                timestamp: 1010,
            }, // Gap
            Trade {
                price: dec!(50000.0),
                quantity: dec!(1.0),
                side: "buy".to_string(),
                timestamp: 1011,
            },
        ];

        let clusters = detect_trade_clusters(&trades, 2, 3);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0], 0); // First cluster starts at index 0
    }

    #[test]
    fn test_calculate_vwap() {
        let trades = sample_trades();
        let vwap = calculate_vwap(&trades).unwrap();

        // (50000*1.0 + 50001*0.5 + 50002*2.0 + 50003*0.3) / (1.0 + 0.5 + 2.0 + 0.3)
        // = (50000 + 25000.5 + 100004 + 15000.9) / 3.8
        // = 190005.4 / 3.8 = 50001.42...
        assert!(vwap > dec!(50001.0) && vwap < dec!(50002.0));
    }

    #[test]
    fn test_calculate_vwap_empty() {
        let trades: Vec<Trade> = vec![];
        assert!(calculate_vwap(&trades).is_none());
    }
}
