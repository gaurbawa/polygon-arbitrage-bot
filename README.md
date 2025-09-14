# Polygon Arbitrage Bot

A Rust application that simulates the detection of arbitrage opportunities between Uniswap V3 and QuickSwap on the Polygon network. Built as a take-home assignment for Alfred Capital.

**Disclaimer: This is a simulation-only bot. It does not execute real trades or connect live to the blockchain. It demonstrates the core logic and structure for arbitrage detection.**

## How It Works

This bot is a learning project that simulates the first critical steps of a real arbitrage system:

1.  **Simulates Price Checks:** It generates simulated exchange rates for the WETH/USDC trading pair on Uniswap V3 and QuickSwap.
2.  **Finds Price Differences:** It calculates the difference between these two prices.
3.  **Calculates Profit:** It figures out the simulated profit from buying low on one DEX and selling high on the other, accounting for simulated gas fees.
4.  **Logs Opportunities:** If the profit is high enough, it prints a detailed opportunity alert to the console.

## My Learning Journey & Project Approach

This was my first hands-on project with both Rust and blockchain development. Here is my step-by-step approach:

1.  **Foundation First:** I knew the most important thing was to get the core application logic right. I started by building the entire program flow—configuration, main loop, profit calculation, and logging—using simulated data. This ensured the "brain" of the bot worked correctly before tackling the complex task of live data fetching.
2.  **Environment Setup:** I learned to set up a Rust development environment, including solving common hurdles like installing the Microsoft C++ Build Tools on Windows, which is necessary for Rust to compile.
3.  **Rust Basics:** I became familiar with Rust's syntax, its strict compiler, and how to manage dependencies using `Cargo.toml`.
4.  **Blockchain Concepts:** I researched how DEXes work, what arbitrage is, and the role of router contracts and RPC providers.

## Architecture

The bot is structured to be easily expanded with real blockchain data:
```plaintext
+-------------------+      +-----------------------+
|   My Rust Bot     |      | (Simulated Price Feed)|
| (Core Logic)      |      |                       |
+-------------------+      +-----------------------+
         |                            |
         | (gets simulated prices)    |
         |                            |
+--------v---------+        +---------v-----------+
| Uniswap V3 (Sim) |        | QuickSwap (Sim)     |
+------------------+        +---------------------+
```


## How to Run This Project

### Prerequisites:
1.  **Install Rust:** [https://rustup.rs](https://rustup.rs)
2.  **(Windows Only) Install Build Tools:** Download [Visual Studio Build Tools](https://aka.ms/buildtools) and install the **"Desktop development with C++"** workload. This is required for Rust to work on Windows.

### Steps:
1.  **Download the Project.** Ensure you have the `Cargo.toml`, `src/main.rs`, `config.json`, and `.env` files.
2.  **Open a terminal** (Command Prompt, PowerShell, or bash).
3.  **Navigate to the project folder:**
    ```bash
    cd polygon-arbitrage-bot
    ```
4.  **Run the bot:**
    ```bash
    cargo run
    ```
5.  You will see output like this, updating every 15 seconds:
    ```
    Starting Polygon Arbitrage Bot...
    Bot configured. Monitoring prices...
    Uniswap V3 Price: 3505.12 USDC
    QuickSwap Price: 3502.87 USDC
    Price Difference: 2.25 USDC
    Simulated Net Profit: $0.25
    No significant opportunity found.
    ```

## The Logical Next Steps

The simulation works. The next clear steps to turn this into a real bot would be:

1.  **Integrate Real Blockchain Data:** Replace the simulated price function with real calls to the Polygon blockchain using the `ethers::contract` library. This involves:
    *   Using the actual router contract addresses from the `config.json` file.
    *   Using the contracts' ABIs (Application Binary Interfaces) to call the correct function (e.g., `getAmountsOut`) to get real prices.
2.  **Add Error Handling:** Build robust error handling for failed network requests.
3.  **Database Integration:** Instead of just printing to the console, log the opportunities to a database for analysis.

This project provided the essential foundation for all of these next steps. Thank you for the opportunity to learn and build.