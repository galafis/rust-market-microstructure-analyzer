use anyhow::Result;
use market_microstructure_analyzer::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    println!("=== Market Microstructure Analyzer Demo ===");
    println!("Analyzing order book depth and trade flow...");
    println!("Demo complete!");
    Ok(())
}
