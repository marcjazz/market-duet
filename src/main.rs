mod market;
mod strategy;
mod risk;
mod pnl;

use market::{Exchange, Side};
use risk::{RiskEngine, RiskLimits};
use pnl::PnlTracker;
use strategy::{FEE, SLIPPAGE, Outcome};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    // Setup
    let mut exchange_a = Exchange::new("Exchange A", 100.0, 0.1);
    let mut exchange_b = Exchange::new("Exchange B", 100.0, 0.1);

    let limits = RiskLimits {
        max_position: 10.0,
        max_drawdown: 20.0,
        max_exposure_per_outcome: 5.0,
    };
    println!("Starting Arbitrage Simulation...");
    println!("Limits: Max Pos: {}, Max Drawdown: {}, Max Outcome Exposure: {}",
        limits.max_position, limits.max_drawdown, limits.max_exposure_per_outcome);

    let mut risk_engine = RiskEngine::new(limits);
    let mut pnl_tracker = PnlTracker::new();

    let iterations = 200; // Reduced for faster final verification
    let sleep_duration = Duration::from_millis(50);
    let mut total_trades = 0;
    println!();

    for i in 1..=iterations {
        // 1. Update prices
        exchange_a.update_price();
        exchange_b.update_price();

        // 2. Run strategy
        let book_a = &exchange_a.order_book;
        let book_b = &exchange_b.order_book;
        let target_quantity = 1.0;

        if let Some(signal) = strategy::detect_arbitrage(
            &exchange_a.name,
            book_a,
            &exchange_b.name,
            book_b,
            target_quantity,
        ) {
            // 3. Check risk
            // For arbitrage, we check if we can take the initial leg
            if risk_engine.check_trade(signal.outcome, signal.quantity) {
                // 4. Execute trade
                total_trades += 1;

                // 4. Execute trade
                // Market impact: fill orders in the books
                let (buy_exchange, sell_exchange) = if signal.buy_exchange == exchange_a.name {
                    (&mut exchange_a, &mut exchange_b)
                } else {
                    (&mut exchange_b, &mut exchange_a)
                };

                let buy_fills = buy_exchange.fill_order(Side::Buy, signal.quantity);
                let sell_fills = sell_exchange.fill_order(Side::Sell, signal.quantity);

                // Calculate actual average prices from fills
                let actual_buy_price = buy_fills.iter().map(|(p, q)| p * q).sum::<f64>() / signal.quantity;
                let actual_sell_price = sell_fills.iter().map(|(p, q)| p * q).sum::<f64>() / signal.quantity;

                // Buy leg
                let buy_price_with_slippage = actual_buy_price * (1.0 + SLIPPAGE);
                let buy_fee = buy_price_with_slippage * FEE;
                
                // Sell leg
                let sell_price_with_slippage = actual_sell_price * (1.0 - SLIPPAGE);
                let sell_fee = sell_price_with_slippage * FEE;

                // Process trades in PnL tracker
                // Leg 1: Buy
                pnl_tracker.process_trade(signal.outcome, signal.quantity, buy_price_with_slippage + buy_fee);
                // Leg 2: Sell
                let realized_pnl = pnl_tracker.process_trade(signal.outcome, -signal.quantity, sell_price_with_slippage - sell_fee);

                // Update Risk Engine
                // Since it's an arbitrage, net position change is 0, but we update PnL
                risk_engine.update(
                    pnl_tracker.get_total_position_abs(), 
                    pnl_tracker.realized_pnl,
                    signal.outcome,
                    0.0 // Net exposure change is 0 for arbitrage
                );

                // Log trade
                println!("[TRADE] #{}", total_trades);
                println!("Outcome: {:?}", signal.outcome);
                println!("Buy {} @ {:.2} | Sell {} @ {:.2}",
                    signal.buy_exchange, actual_buy_price,
                    signal.sell_exchange, actual_sell_price);
                println!("Signal Net Profit: {:.4}", signal.net_profit);
                println!("Realized PnL: {:.4} | Total PnL: {:.4}", realized_pnl, pnl_tracker.realized_pnl);
                println!("-----------------------------------");
            }
        }

        // 5. Periodic summary every 50 iterations
        if i % 50 == 0 {
            let tob_a = exchange_a.get_tob();
            let mid_a = (tob_a.best_bid + tob_a.best_ask) / 2.0;
            
            let tob_b = exchange_b.get_tob();
            let mid_b = (tob_b.best_bid + tob_b.best_ask) / 2.0;
            
            // Use average mid price for unrealized PnL estimation
            let avg_mid = (mid_a + mid_b) / 2.0;
            let mut prices = HashMap::new();
            prices.insert(Outcome::Yes, avg_mid);
            prices.insert(Outcome::No, avg_mid); // In a real scenario, Yes/No would have different prices
            
            let unrealized_pnl = pnl_tracker.get_total_unrealized_pnl(&prices);
            let drawdown = risk_engine.peak_pnl - risk_engine.current_pnl;

            println!("[SUMMARY] Iteration {}/{}", i, iterations);
            println!("Total Trades: {}", total_trades);
            println!("Realized PnL: {:.4}", pnl_tracker.realized_pnl);
            println!("Unrealized PnL: {:.4}", unrealized_pnl);
            println!("Max Drawdown: {:.4}", drawdown);
            println!("Current Exposure: {:.1}", pnl_tracker.get_total_position_abs());
            println!();
        }

        thread::sleep(sleep_duration);
    }

    println!("Simulation complete.");
    println!("Final Realized PnL: {:.4}", pnl_tracker.realized_pnl);
}
