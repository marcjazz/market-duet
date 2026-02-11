# AGENTS.md (Ask Mode)

## Contextual Knowledge
- **MPAE Purpose:** A proof-of-concept for professional trading systems architecture in Rust.
- **PnL Logic:** Realized PnL is calculated only on sells using weighted average entry price.
- **Market Simulation:** Uses a 100ms random walk; no real-world data or APIs.

## Key Definitions
- **Spread:** `ExchangeB.best_bid - ExchangeA.best_ask`.
- **Net Profit:** `Spread - Fees (0.1%) - Slippage (0.05%)`.
- **Risk Gating:** The mandatory check in `risk.rs` before any trade execution.
