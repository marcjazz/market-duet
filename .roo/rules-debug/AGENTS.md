# AGENTS.md (Debug Mode)

## Debugging Focus
- **Price Drift:** Verify random walk logic in `market.rs` doesn't lead to infinite divergence.
- **PnL Discrepancies:** Check `pnl.rs` for floating point errors; ensure weighted average entry price is updated correctly on every buy.
- **Risk Rejections:** If trades aren't executing, check `risk.rs` limits (max position/drawdown).

## Trace Points
- Log raw `TopOfBook` prices before strategy evaluation.
- Log `RiskLimits` state when a trade is rejected.
- Verify `[TRADE]` log format matches `README.md` exactly for parser compatibility.
