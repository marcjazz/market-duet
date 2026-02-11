use crate::market::OrderBook;

pub const FEE: f64 = 0.001;
pub const SLIPPAGE: f64 = 0.0005;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Outcome {
    Yes,
    No,
}

#[derive(Debug)]
pub struct TradeSignal {
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub outcome: Outcome,
    pub quantity: f64,
    pub net_profit: f64,
}

pub fn detect_arbitrage(
    name_a: &str,
    book_a: &OrderBook,
    name_b: &str,
    book_b: &OrderBook,
    target_quantity: f64,
) -> Option<TradeSignal> {
    let outcomes = [Outcome::Yes, Outcome::No];
    
    for outcome in outcomes {
        // Case 1: Buy A, Sell B
        if let Some(signal) = check_arb(name_a, book_a, name_b, book_b, outcome, target_quantity) {
            return Some(signal);
        }
        // Case 2: Buy B, Sell A
        if let Some(signal) = check_arb(name_b, book_b, name_a, book_a, outcome, target_quantity) {
            return Some(signal);
        }
    }

    None
}

fn check_arb(
    buy_name: &str,
    buy_book: &OrderBook,
    sell_name: &str,
    sell_book: &OrderBook,
    outcome: Outcome,
    quantity: f64,
) -> Option<TradeSignal> {
    // Calculate VWAP for buying 'quantity' from buy_book (asks)
    let avg_buy_price = calculate_vwap(&buy_book.asks, quantity)?;
    
    // Calculate VWAP for selling 'quantity' to sell_book (bids)
    let avg_sell_price = calculate_vwap(&sell_book.bids, quantity)?;

    let buy_cost = avg_buy_price * quantity;
    let sell_proceeds = avg_sell_price * quantity;
    
    let fees = (buy_cost * FEE) + (sell_proceeds * FEE);
    let slippage_cost = (buy_cost * SLIPPAGE) + (sell_proceeds * SLIPPAGE);
    
    let net_profit = sell_proceeds - buy_cost - fees - slippage_cost;

    if net_profit > 0.0 {
        Some(TradeSignal {
            buy_exchange: buy_name.to_string(),
            sell_exchange: sell_name.to_string(),
            outcome,
            quantity,
            net_profit,
        })
    } else {
        None
    }
}

fn calculate_vwap(levels: &[crate::market::Level], quantity: f64) -> Option<f64> {
    let mut total_cost = 0.0;
    let mut remaining_qty = quantity;
    
    for level in levels {
        let fill_qty = f64::min(remaining_qty, level.quantity);
        total_cost += fill_qty * level.price;
        remaining_qty -= fill_qty;
        
        if remaining_qty <= 0.0 {
            break;
        }
    }
    
    if remaining_qty > 0.0 {
        // Not enough liquidity
        None
    } else {
        Some(total_cost / quantity)
    }
}
