use anyhow::Result;
use market_microstructure_analyzer::*;
use rust_decimal_macros::dec;

fn main() -> Result<()> {
    println!("=== Advanced Pattern Detection Demo ===\n");

    // Simulate trades that might indicate an iceberg order
    let trades = vec![
        Trade {
            price: dec!(50000.0),
            quantity: dec!(0.1),
            side: "buy".to_string(),
            timestamp: 1000,
        },
        Trade {
            price: dec!(50000.0),
            quantity: dec!(0.12),
            side: "buy".to_string(),
            timestamp: 1001,
        },
        Trade {
            price: dec!(50000.0),
            quantity: dec!(0.11),
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
            quantity: dec!(0.13),
            side: "buy".to_string(),
            timestamp: 1004,
        },
    ];

    println!("🔍 Detecting Iceberg Orders...");
    let icebergs = patterns::detect_iceberg_orders(&trades, 3, dec!(1.0));

    if !icebergs.is_empty() {
        println!("  ⚠️  Potential iceberg order detected!");
        for pattern in icebergs {
            if let patterns::Pattern::IcebergOrder {
                price,
                estimated_size,
            } = pattern
            {
                println!("    Price: ${}", price);
                println!("    Estimated Total Size: {}", estimated_size);
                println!("    Individual fills: {} trades", trades.len());
            }
        }
    } else {
        println!("  ✓ No iceberg orders detected");
    }

    // Create order book with potential spoofing
    let orderbook = OrderBook {
        bids: vec![
            Level {
                price: dec!(50000.0),
                quantity: dec!(1.0),
            },
            Level {
                price: dec!(49999.0),
                quantity: dec!(100.0),
            }, // Suspiciously large
            Level {
                price: dec!(49998.0),
                quantity: dec!(0.5),
            },
        ],
        asks: vec![
            Level {
                price: dec!(50001.0),
                quantity: dec!(1.0),
            },
            Level {
                price: dec!(50002.0),
                quantity: dec!(0.8),
            },
        ],
        timestamp: 1000,
    };

    println!("\n🕵️  Detecting Spoofing...");
    let spoofing = patterns::detect_spoofing(&orderbook, dec!(50.0));

    if !spoofing.is_empty() {
        println!("  ⚠️  Potential spoofing detected!");
        for pattern in spoofing {
            if let patterns::Pattern::Spoofing { price, side } = pattern {
                println!("    {} side at ${}", side, price);
                println!("    Large order far from best price - might be fake");
            }
        }
    } else {
        println!("  ✓ No spoofing detected");
    }

    // Detect support and resistance levels
    println!("\n📊 Support & Resistance Levels...");
    let levels = patterns::detect_support_resistance(&orderbook, dec!(5.0));

    let mut supports = Vec::new();
    let mut resistances = Vec::new();

    for pattern in &levels {
        match pattern {
            patterns::Pattern::Support { price, strength } => {
                supports.push((price, strength));
            }
            patterns::Pattern::Resistance { price, strength } => {
                resistances.push((price, strength));
            }
            _ => {}
        }
    }

    println!("  🟢 Support Levels:");
    for (price, strength) in supports {
        println!("    ${} (volume: {})", price, strength);
    }

    println!("  🔴 Resistance Levels:");
    for (price, strength) in resistances {
        println!("    ${} (volume: {})", price, strength);
    }

    // Test absorption detection
    let absorption_trades = vec![
        Trade {
            price: dec!(50000.0),
            quantity: dec!(5.0),
            side: "buy".to_string(),
            timestamp: 1000,
        },
        Trade {
            price: dec!(50000.2),
            quantity: dec!(4.5),
            side: "sell".to_string(),
            timestamp: 1001,
        },
        Trade {
            price: dec!(50000.1),
            quantity: dec!(6.0),
            side: "buy".to_string(),
            timestamp: 1002,
        },
    ];

    println!("\n💧 Detecting Absorption...");
    let absorption = patterns::detect_absorption(&absorption_trades, dec!(10.0), dec!(1.0));

    if !absorption.is_empty() {
        println!("  ⚠️  Absorption detected!");
        for pattern in absorption {
            if let patterns::Pattern::Absorption { price, volume } = pattern {
                println!("    Large volume ({}) absorbed near ${}", volume, price);
                println!("    Price barely moved - strong buyer/seller present");
            }
        }
    } else {
        println!("  ✓ No significant absorption");
    }

    println!("\n=== Pattern Detection Complete ===");
    Ok(())
}
