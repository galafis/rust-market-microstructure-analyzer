use anyhow::Result;
use market_microstructure_analyzer::*;
use rust_decimal_macros::dec;

fn main() -> Result<()> {
    println!("=== Tape Reading & Metrics Analysis Demo ===\n");

    // Simulate a series of trades
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
        Trade {
            price: dec!(50003.0),
            quantity: dec!(0.3),
            side: "sell".to_string(),
            timestamp: 1696435203,
        },
        Trade {
            price: dec!(50004.0),
            quantity: dec!(1.5),
            side: "buy".to_string(),
            timestamp: 1696435204,
        },
        Trade {
            price: dec!(50005.0),
            quantity: dec!(0.8),
            side: "buy".to_string(),
            timestamp: 1696435205,
        },
        Trade {
            price: dec!(50004.5),
            quantity: dec!(0.4),
            side: "sell".to_string(),
            timestamp: 1696435206,
        },
        Trade {
            price: dec!(50006.0),
            quantity: dec!(10.0),
            side: "buy".to_string(),
            timestamp: 1696435207,
        }, // Block trade
    ];

    // Display trades
    println!("📊 Recent Trades:");
    println!("─────────────────────────────────────────");
    visualization::print_trades(&trades, 5);

    // Calculate trade pressure
    println!("\n💪 Trade Pressure Analysis:");
    println!("─────────────────────────────────────────");
    let (buy_vol, sell_vol, net_vol) = tape::calculate_trade_pressure(&trades);

    println!("  Buy Volume:  {:.2}", buy_vol);
    println!("  Sell Volume: {:.2}", sell_vol);
    println!("  Net Volume:  {:.2}", net_vol);

    let pressure_ratio = buy_vol / (buy_vol + sell_vol) * dec!(100);
    println!("\n  Buy Pressure: {:.1}%", pressure_ratio);
    println!("  Sell Pressure: {:.1}%", dec!(100) - pressure_ratio);

    if net_vol > dec!(0) {
        println!("  → Market sentiment: BULLISH 🟢");
    } else {
        println!("  → Market sentiment: BEARISH 🔴");
    }

    // Calculate aggression ratio
    println!("\n⚔️  Aggression Analysis:");
    println!("─────────────────────────────────────────");
    let aggression = tape::calculate_aggression_ratio(&trades);
    println!("  Aggression Ratio: {:.2}", aggression);

    if aggression > dec!(0.6) {
        println!("  → High buying aggression - buyers in control");
    } else if aggression < dec!(0.4) {
        println!("  → High selling aggression - sellers in control");
    } else {
        println!("  → Balanced aggression");
    }

    // Identify block trades
    println!("\n🐋 Block Trades:");
    println!("─────────────────────────────────────────");
    let blocks = tape::identify_block_trades(&trades, dec!(5.0));

    if blocks.is_empty() {
        println!("  No block trades detected (threshold: 5.0)");
    } else {
        println!("  {} block trade(s) detected:", blocks.len());
        for trade in blocks {
            println!(
                "    {} {} @ ${} (size: {})",
                if trade.side == "buy" { "BUY " } else { "SELL" },
                trade.quantity,
                trade.price,
                trade.quantity
            );
        }
    }

    // Calculate VWAP
    println!("\n📍 VWAP (Volume Weighted Average Price):");
    println!("─────────────────────────────────────────");
    if let Some(vwap) = tape::calculate_vwap(&trades) {
        println!("  VWAP: ${:.2}", vwap);

        let last_price = trades.last().unwrap().price;
        let distance = ((last_price - vwap) / vwap * dec!(100)).abs();

        println!("  Last Price: ${:.2}", last_price);
        println!("  Distance from VWAP: {:.3}%", distance);

        if last_price > vwap {
            println!("  → Price above VWAP (potential resistance)");
        } else {
            println!("  → Price below VWAP (potential support)");
        }
    }

    // Calculate Delta and CVD
    println!("\n📈 Delta & CVD Analysis:");
    println!("─────────────────────────────────────────");
    let delta = metrics::calculate_delta(&trades);
    println!("  Delta Volume: {:.2}", delta);

    let cvd_data = metrics::calculate_cvd(&trades);
    println!("\n  Cumulative Volume Delta:");
    for (i, (_timestamp, cvd)) in cvd_data.iter().enumerate().rev().take(5).rev() {
        println!("    Trade {}: {:.2}", i + 1, cvd);
    }

    if let Some((_, final_cvd)) = cvd_data.last() {
        if *final_cvd > dec!(0) {
            println!("\n  → CVD trending up - accumulation phase");
        } else {
            println!("\n  → CVD trending down - distribution phase");
        }
    }

    // Volume Profile
    println!("\n📊 Volume Profile:");
    println!("─────────────────────────────────────────");
    let profile = metrics::calculate_volume_profile(&trades, dec!(1.0));

    if let Some(poc) = profile.poc {
        println!("  Point of Control (POC): ${:.2}", poc);
        println!("  → Price with highest volume");
    }

    if let Some(vah) = profile.vah {
        println!("  Value Area High (VAH): ${:.2}", vah);
    }

    if let Some(val) = profile.val {
        println!("  Value Area Low (VAL): ${:.2}", val);
    }

    println!("\n  Top Volume Levels:");
    let mut sorted_levels: Vec<_> = profile.levels.iter().collect();
    sorted_levels.sort_by(|a, b| b.1.cmp(a.1));

    for (i, (price, volume)) in sorted_levels.iter().take(3).enumerate() {
        println!("    {}. ${:.2} - volume: {:.2}", i + 1, price, volume);
    }

    // Detect trade clusters
    println!("\n🔥 Trade Clusters:");
    println!("─────────────────────────────────────────");
    let clusters = tape::detect_trade_clusters(&trades, 2, 3);

    if clusters.is_empty() {
        println!("  No significant clusters detected");
    } else {
        println!("  {} cluster(s) of rapid trading:", clusters.len());
        for cluster_start in clusters {
            println!("    Starting at trade #{}", cluster_start + 1);
        }
    }

    println!("\n=== Analysis Complete ===");
    Ok(())
}
