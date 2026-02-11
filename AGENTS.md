# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Project Context

- **Name:** Minimum Professional Arbitrage Engine (MPAE)
- **Stack:** Rust (Minimal, synchronous execution preferred for MVP)
- **Goal:** Simulate arbitrage between two exchanges with risk management and PnL tracking.

## Core Modules

- `market.rs`: Price simulation (random walk, 100ms updates).
- `strategy.rs`: Arbitrage detection logic.
- `risk.rs`: Risk gating (max position, max drawdown).
- `pnl.rs`: Inventory-based PnL accounting (realized/unrealized).

## Key Patterns

- **No Async:** Avoid `tokio` unless explicitly required; keep execution synchronous.
- **No External APIs:** All market data and execution are simulated in-memory.
- **Risk First:** Every trade must pass through `risk.rs` before execution.
- **Fee Modeling:** Always apply fees (0.1%) and slippage (0.05%) to trades.
