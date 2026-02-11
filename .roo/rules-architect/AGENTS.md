# AGENTS.md (Architect Mode)

## Design Principles
- **Separation of Concerns:** Keep market simulation, strategy, risk, and PnL in distinct modules.
- **Synchronous Flow:** Maintain a linear execution loop (Market -> Strategy -> Risk -> PnL -> Log).
- **Simulation First:** Design for in-memory execution; avoid external dependencies or async overhead.

## Module Responsibilities
- `market.rs`: State owner for exchange prices.
- `strategy.rs`: Pure logic for signal generation.
- `risk.rs`: State owner for limits and trade validation.
- `pnl.rs`: State owner for inventory and accounting.

## Constraints
- No multi-threading or async unless strictly required for performance benchmarks.
- All trade signals MUST be validated by `risk.rs` before affecting `pnl.rs` or `market.rs` state.
