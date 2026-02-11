# AGENTS.md (Code Mode)

## Coding Standards
- **Synchronous Rust:** Use standard library types; avoid `async/await` or `tokio` unless strictly necessary for the MVP.
- **Deterministic Logic:** Ensure market simulation and strategy execution are deterministic for testing.
- **Risk Gating:** All trade execution logic MUST be preceded by a call to `risk.rs` validation.

## Implementation Details
- **Fee/Slippage:** Hardcode 0.1% fee and 0.05% slippage in `strategy.rs` or `pnl.rs`.
- **PnL Math:** Use weighted average entry price for realized PnL calculations.
- **Logging:** Use structured logging for `[TRADE]` and `[SUMMARY]` outputs as defined in `README.md`.
