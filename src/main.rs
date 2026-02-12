use anyhow::Result;
use market_microstructure_analyzer::*;
use rust_decimal_macros::dec;

fn main() -> Result<()> {
    env_logger::init();
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   Market Microstructure Analyzer - Demo          ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

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

    // Sample trades
    let trades = vec![
        Trade {
            price: dec!(50000.0),
            quantity: dec!(1.0),
            side: "buy".to_string(),
            timestamp: 1696435200,
        },
        Trade {
            price: dec!(50001.0),
            quantity: dec!(0.5),
            side: "sell".to_string(),
            timestamp: 1696435201,
        },
        Trade {
            price: dec!(50002.0),
            quantity: dec!(2.0),
            side: "buy".to_string(),
            timestamp: 1696435202,
        },
    ];

    // Display order book
    println!("📊 Order Book Analysis:");
    println!("─────────────────────────────────────────────────\n");

    if let Some((spread, spread_pct)) = orderbook::calculate_spread(&orderbook) {
        println!("  💰 Spread: ${} ({:.4}%)", spread, spread_pct);
    }

    let imbalance = orderbook::calculate_imbalance(&orderbook, None);
    println!("  ⚖️  Imbalance: {:.4}", imbalance);

    if imbalance > dec!(0) {
        println!("     → More buying pressure (bullish)");
    } else if imbalance < dec!(0) {
        println!("     → More selling pressure (bearish)");
    } else {
        println!("     → Market balanced");
    }

    if let Some(mid) = orderbook::mid_price(&orderbook) {
        println!("  📍 Mid Price: ${}", mid);
    }

    println!("\n📈 Tape Analysis:");
    println!("─────────────────────────────────────────────────\n");

    let (buy_vol, sell_vol, net_vol) = tape::calculate_trade_pressure(&trades);
    println!("  🟢 Buy Volume: {}", buy_vol);
    println!("  🔴 Sell Volume: {}", sell_vol);
    println!(
        "  📊 Net Volume: {} ({})",
        net_vol,
        if net_vol > dec!(0) {
            "bullish"
        } else {
            "bearish"
        }
    );

    if let Some(vwap) = tape::calculate_vwap(&trades) {
        println!("  📌 VWAP: ${:.2}", vwap);
    }

    println!("\n🔍 Pattern Detection:");
    println!("─────────────────────────────────────────────────\n");

    let support_resistance = patterns::detect_support_resistance(&orderbook, dec!(1.0));
    println!(
        "  Found {} support/resistance levels",
        support_resistance.len()
    );

    for pattern in &support_resistance {
        match pattern {
            patterns::Pattern::Support { price, strength } => {
                println!("  🟩 Support at ${} (strength: {})", price, strength);
            }
            patterns::Pattern::Resistance { price, strength } => {
                println!("  🟥 Resistance at ${} (strength: {})", price, strength);
            }
            _ => {}
        }
    }

    println!("\n✅ Analysis Complete!");
    println!("\n💡 Tip: Run 'cargo run --example orderbook_analysis' for more examples");

    Ok(())
}
