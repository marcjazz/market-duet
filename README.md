# Minimum Professional Arbitrage Engine (MPAE)

A minimal, high-performance Rust-based engine for simulating arbitrage between two exchanges. This project focuses on synchronous execution, robust risk management, and accurate PnL tracking in an in-memory simulation environment.

## Goals

- **Arbitrage Simulation:** Detect and execute arbitrage opportunities between two simulated exchanges.
- **Risk Management:** Implement strict risk gating to prevent excessive exposure and drawdowns.
- **PnL Tracking:** Provide detailed inventory-based accounting for both realized and unrealized profit and loss.
- **Efficiency:** Utilize synchronous execution for predictable performance in an MVP setting.

## Core Modules

- [`market.rs`](src/market.rs): Price simulation engine using a random walk model with 100ms updates.
- [`strategy.rs`](src/strategy.rs): Core logic for arbitrage detection and signal generation.
- [`risk.rs`](src/risk.rs): Critical risk gating layer enforcing maximum position limits and drawdown thresholds.
- [`pnl.rs`](src/pnl.rs): Inventory-based accounting system for tracking trading performance.

## Key Patterns

- **No Async:** Synchronous execution is preferred to maintain simplicity and predictability (avoids `tokio`).
- **No External APIs:** Entirely self-contained; market data and trade execution are simulated in-memory.
- **Risk First:** Architectural requirement that every trade must pass through the Risk module before execution.
- **Realistic Modeling:** All trades incorporate fee modeling (0.1% fees) and slippage (0.05%) for realistic simulation results.

## Requirements

- Rust (latest stable)
- Cargo

## Usage

### Run the Simulation
To start the arbitrage engine simulation:
```bash
cargo run
```

### Run Tests
To execute the project's test suite:
```bash
cargo test
```

## Project Structure

```text
├── src/
│   ├── main.rs      # Application entry point
│   ├── market.rs    # Price simulation
│   ├── strategy.rs  # Arbitrage logic
│   ├── risk.rs      # Risk management
│   └── pnl.rs       # PnL accounting
├── AGENTS.md        # Agent-specific guidance and context
└── Cargo.toml       # Project dependencies and configuration
```
