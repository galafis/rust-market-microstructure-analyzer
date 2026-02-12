//! Pattern Detection Module
//!
//! This module provides functionality to detect common market microstructure patterns.

use crate::types::{OrderBook, Trade};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Represents a detected pattern
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Large hidden order (iceberg)
    IcebergOrder {
        price: Decimal,
        estimated_size: Decimal,
    },
    /// Potential spoofing detected
    Spoofing { price: Decimal, side: String },
    /// Strong support level
    Support { price: Decimal, strength: Decimal },
    /// Strong resistance level
    Resistance { price: Decimal, strength: Decimal },
    /// Liquidity absorption detected
    Absorption { price: Decimal, volume: Decimal },
}

/// Detect potential iceberg orders
///
/// Iceberg orders are large orders hidden by placing small visible amounts
/// Detection: Multiple fills at same price with consistent size
///
/// # Arguments
/// * `trades` - Recent trades
/// * `min_fills` - Minimum number of fills to consider
/// * `price_tolerance` - Maximum price difference to group trades
///
/// # Returns
/// Vector of detected iceberg patterns
#[must_use]
pub fn detect_iceberg_orders(
    trades: &[Trade],
    min_fills: usize,
    price_tolerance: Decimal,
) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut price_fills: std::collections::HashMap<Decimal, Vec<&Trade>> =
        std::collections::HashMap::new();

    // Group trades by similar price
    for trade in trades {
        let price_level = (trade.price / price_tolerance).round() * price_tolerance;
        price_fills.entry(price_level).or_default().push(trade);
    }

    // Check for repeated fills at same price
    for (price, fills) in price_fills {
        if fills.len() >= min_fills {
            let total_size: Decimal = fills.iter().map(|t| t.quantity).sum();
            let avg_size = total_size / Decimal::from(fills.len());

            // If many fills with consistent small sizes, likely an iceberg
            // Check if individual fills are small compared to average
            let consistent_small_fills = fills.iter().all(|t| t.quantity <= avg_size * dec!(1.5));

            if consistent_small_fills {
                patterns.push(Pattern::IcebergOrder {
                    price,
                    estimated_size: total_size,
                });
            }
        }
    }

    patterns
}

/// Detect potential spoofing
///
/// Spoofing: Large orders on one side that get cancelled, manipulating price
/// Detection: Large orders far from mid that don't get filled
///
/// # Arguments
/// * `orderbook` - Current order book
/// * `threshold` - Minimum size to consider as potential spoof
///
/// # Returns
/// Vector of potential spoofing patterns
#[must_use]
pub fn detect_spoofing(orderbook: &OrderBook, threshold: Decimal) -> Vec<Pattern> {
    let mut patterns = Vec::new();

    // Check for unusually large orders
    for (i, bid) in orderbook.bids.iter().enumerate() {
        if i > 0 && bid.quantity > threshold {
            // Large order not at best bid might be spoofing
            patterns.push(Pattern::Spoofing {
                price: bid.price,
                side: "bid".to_string(),
            });
        }
    }

    for (i, ask) in orderbook.asks.iter().enumerate() {
        if i > 0 && ask.quantity > threshold {
            // Large order not at best ask might be spoofing
            patterns.push(Pattern::Spoofing {
                price: ask.price,
                side: "ask".to_string(),
            });
        }
    }

    patterns
}

/// Detect support and resistance levels
///
/// # Arguments
/// * `orderbook` - Current order book
/// * `threshold` - Minimum volume to consider significant
///
/// # Returns
/// Vector of support/resistance patterns
#[must_use]
pub fn detect_support_resistance(orderbook: &OrderBook, threshold: Decimal) -> Vec<Pattern> {
    let mut patterns = Vec::new();

    // Support levels (large bids)
    for bid in &orderbook.bids {
        if bid.quantity >= threshold {
            patterns.push(Pattern::Support {
                price: bid.price,
                strength: bid.quantity,
            });
        }
    }

    // Resistance levels (large asks)
    for ask in &orderbook.asks {
        if ask.quantity >= threshold {
            patterns.push(Pattern::Resistance {
                price: ask.price,
                strength: ask.quantity,
            });
        }
    }

    patterns
}

/// Detect absorption (large volume traded without price movement)
///
/// # Arguments
/// * `trades` - Recent trades
/// * `volume_threshold` - Minimum volume to detect
/// * `price_range` - Maximum price movement
///
/// # Returns
/// Vector of absorption patterns
#[must_use]
pub fn detect_absorption(
    trades: &[Trade],
    volume_threshold: Decimal,
    price_range: Decimal,
) -> Vec<Pattern> {
    let mut patterns = Vec::new();

    if trades.is_empty() {
        return patterns;
    }

    let min_price = trades.iter().map(|t| t.price).min().unwrap();
    let max_price = trades.iter().map(|t| t.price).max().unwrap();
    let total_volume: Decimal = trades.iter().map(|t| t.quantity).sum();

    // If large volume traded in small price range, absorption likely
    if total_volume >= volume_threshold && (max_price - min_price) <= price_range {
        let avg_price = trades.iter().map(|t| t.price * t.quantity).sum::<Decimal>() / total_volume;

        patterns.push(Pattern::Absorption {
            price: avg_price,
            volume: total_volume,
        });
    }

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Level;

    #[test]
    fn test_detect_iceberg_orders() {
        let trades = vec![
            Trade {
                price: dec!(50000.0),
                quantity: dec!(0.1),
                side: "buy".to_string(),
                timestamp: 1000,
            },
            Trade {
                price: dec!(50000.0),
                quantity: dec!(0.1),
                side: "buy".to_string(),
                timestamp: 1001,
            },
            Trade {
                price: dec!(50000.0),
                quantity: dec!(0.1),
                side: "buy".to_string(),
                timestamp: 1002,
            },
            Trade {
                price: dec!(50000.0),
                quantity: dec!(0.1),
                side: "buy".to_string(),
                timestamp: 1003,
            },
            Trade {
                price: dec!(50000.0),
                quantity: dec!(0.1),
                side: "buy".to_string(),
                timestamp: 1004,
            },
        ];

        let patterns = detect_iceberg_orders(&trades, 3, dec!(1.0));
        assert!(!patterns.is_empty());

        if let Pattern::IcebergOrder {
            price,
            estimated_size,
        } = &patterns[0]
        {
            assert_eq!(*price, dec!(50000.0));
            assert_eq!(*estimated_size, dec!(0.5));
        } else {
            panic!("Expected IcebergOrder pattern");
        }
    }

    #[test]
    fn test_detect_spoofing() {
        let orderbook = OrderBook {
            bids: vec![
                Level {
                    price: dec!(50000.0),
                    quantity: dec!(1.0),
                },
                Level {
                    price: dec!(49999.0),
                    quantity: dec!(100.0),
                }, // Large order
            ],
            asks: vec![Level {
                price: dec!(50001.0),
                quantity: dec!(1.0),
            }],
            timestamp: 1000,
        };

        let patterns = detect_spoofing(&orderbook, dec!(50.0));
        assert_eq!(patterns.len(), 1);

        if let Pattern::Spoofing { price, side } = &patterns[0] {
            assert_eq!(*price, dec!(49999.0));
            assert_eq!(side, "bid");
        } else {
            panic!("Expected Spoofing pattern");
        }
    }

    #[test]
    fn test_detect_support_resistance() {
        let orderbook = OrderBook {
            bids: vec![
                Level {
                    price: dec!(50000.0),
                    quantity: dec!(10.0),
                },
                Level {
                    price: dec!(49999.0),
                    quantity: dec!(5.0),
                },
            ],
            asks: vec![Level {
                price: dec!(50001.0),
                quantity: dec!(15.0),
            }],
            timestamp: 1000,
        };

        let patterns = detect_support_resistance(&orderbook, dec!(8.0));
        assert_eq!(patterns.len(), 2);

        // Should detect support at 50000 and resistance at 50001
        assert!(patterns
            .iter()
            .any(|p| matches!(p, Pattern::Support { .. })));
        assert!(patterns
            .iter()
            .any(|p| matches!(p, Pattern::Resistance { .. })));
    }

    #[test]
    fn test_detect_absorption() {
        let trades = vec![
            Trade {
                price: dec!(50000.0),
                quantity: dec!(5.0),
                side: "buy".to_string(),
                timestamp: 1000,
            },
            Trade {
                price: dec!(50000.5),
                quantity: dec!(5.0),
                side: "sell".to_string(),
                timestamp: 1001,
            },
            Trade {
                price: dec!(50000.2),
                quantity: dec!(5.0),
                side: "buy".to_string(),
                timestamp: 1002,
            },
        ];

        let patterns = detect_absorption(&trades, dec!(10.0), dec!(1.0));
        assert_eq!(patterns.len(), 1);

        if let Pattern::Absorption { volume, .. } = &patterns[0] {
            assert_eq!(*volume, dec!(15.0));
        } else {
            panic!("Expected Absorption pattern");
        }
    }
}
