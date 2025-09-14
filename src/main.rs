use ethers::{
    providers::{Http, Provider},
    types::{Address, U256},
};
use std::{str::FromStr, sync::Arc};
use tokio::time::{sleep, Duration};
use serde::Deserialize;
use std::fs;

// This struct defines the shape of our config.json file
#[derive(Debug, Deserialize)]
struct Config {
    rpc_url: String,
    dexes: Vec<Dex>,
    tokens: Tokens,
    min_profit_threshold_usd: f64,
    fixed_trade_amount_weth: String,
}

#[derive(Debug, Deserialize)]
struct Dex {
    name: String,
    router_address: String,
}

#[derive(Debug, Deserialize)]
struct Tokens {
    usdc: String,
    weth: String,
}

// The main function is the entry point of our program
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Starting Polygon Arbitrage Bot...");

    // 1. Load configuration from files
    let config = load_config().await?;
    let provider = setup_provider(&config.rpc_url).await?;

    // Convert token addresses from String to the Address type Rust understands
    let usdc_address = Address::from_str(&config.tokens.usdc)?;
    let weth_address = Address::from_str(&config.tokens.weth)?;
    let fixed_trade_amount = config.fixed_trade_amount_weth.parse::<f64>()?;

    println!("Bot configured. Monitoring prices...");

    // 2. Main loop: runs forever, checking prices every 15 seconds
    loop {
        // Check prices on both DEXes at the same time
        let (price_dex_a, price_dex_b) = tokio::join!(
            get_price(&provider, &config.dexes[0], weth_address, usdc_address),
            get_price(&provider, &config.dexes[1], weth_address, usdc_address)
        );

        // If we got prices from both DEXes, check for arbitrage
        if let (Ok(price_a), Ok(price_b)) = (price_dex_a, price_dex_b) {
            println!("{}: 1 WETH = {:.2} USDC", config.dexes[0].name, price_a);
            println!("{}: 1 WETH = {:.2} USDC", config.dexes[1].name, price_b);

            // 3. Check which DEX is cheaper
            if price_a > price_b {
                check_arbitrage_opportunity(
                    &config.dexes[0].name,
                    price_a,
                    &config.dexes[1].name,
                    price_b,
                    fixed_trade_amount,
                    config.min_profit_threshold_usd,
                );
            } else {
                check_arbitrage_opportunity(
                    &config.dexes[1].name,
                    price_b,
                    &config.dexes[0].name,
                    price_a,
                    fixed_trade_amount,
                    config.min_profit_threshold_usd,
                );
            }
        } else {
            // Handle errors if price fetching failed
            println!("Error fetching prices from one or more DEXes.");
        }

        // Wait for 15 seconds before checking again
        sleep(Duration::from_secs(15)).await;
    }
}

// Loads the config.json file
async fn load_config() -> Result<Config, anyhow::Error> {
    let config_contents = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_contents)?;
    Ok(config)
}

// Sets up the connection to the Polygon blockchain
async fn setup_provider(rpc_url: &str) -> Result<Arc<Provider<Http>>, anyhow::Error> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    Ok(Arc::new(provider))
}

// Fetches the price of WETH in USDC from a specific DEX
// This is a simplified version. A real bot would use the DEX's specific Router contract functions.
async fn get_price(
    provider: &Arc<Provider<Http>>,
    dex: &Dex,
    _token_in: Address, // WETH
    _token_out: Address, // USDC
) -> Result<f64, anyhow::Error> {
    // SIMPLIFICATION FOR DEMONSTRATION
    // In reality, here you would use the `ethers::contract` crate to create a contract instance
    // and call a function on the router contract like `getAmountsOut`.
    // This requires the specific ABI (Application Binary Interface) of the contract.
    // For this demo, we'll simulate a price.

    let simulated_price = match dex.name.as_str() {
        "Uniswap V3" => 3500.0 + (rand::random::<f64>() * 10.0), // e.g., between 3500 and 3510
        "QuickSwap" => 3500.0 + (rand::random::<f64>() * 10.0),  // e.g., between 3500 and 3510
        _ => 3500.0,
    };

    Ok(simulated_price)
}

// Checks if an arbitrage opportunity exists and logs it
fn check_arbitrage_opportunity(
    expensive_dex: &str,
    expensive_price: f64,
    cheap_dex: &str,
    cheap_price: f64,
    trade_amount: f64,
    min_profit_threshold: f64,
) {
    let price_difference = expensive_price - cheap_price;
    let gross_profit = price_difference * trade_amount;

    // Simplified simulated gas cost (e.g., $2 worth of MATIC)
    let simulated_gas_cost_usd = 2.0;
    let net_profit = gross_profit - simulated_gas_cost_usd;

    println!("Price Difference: {:.2} USDC per WETH", price_difference);
    println!("Simulated Net Profit: ${:.2}", net_profit);

    if net_profit > min_profit_threshold {
        println!("🚀 ARBITRAGE OPPORTUNITY DETECTED!");
        println!("   Buy  {} WETH on {} for ${:.2}", trade_amount, cheap_dex, cheap_price * trade_amount);
        println!("   Sell {} WETH on {} for ${:.2}", trade_amount, expensive_dex, expensive_price * trade_amount);
        println!("   Estimated Profit: ${:.2}\n", net_profit);
    } else {
        println!("No significant opportunity found.\n");
    }
}