use std::collections::HashMap;
use crate::strategy::Outcome;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub quantity: f64,
    pub avg_entry_price: f64,
}

pub struct PnlTracker {
    pub positions: HashMap<Outcome, Position>,
    pub realized_pnl: f64,
}

impl PnlTracker {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
            realized_pnl: 0.0,
        }
    }

    pub fn process_trade(&mut self, outcome: Outcome, quantity: f64, price: f64) -> f64 {
        if quantity == 0.0 {
            return 0.0;
        }

        let position = self.positions.entry(outcome).or_insert(Position {
            quantity: 0.0,
            avg_entry_price: 0.0,
        });

        let current_qty = position.quantity;
        let mut realized = 0.0;

        // Check if we are increasing or decreasing/flipping position
        if current_qty == 0.0 || (current_qty > 0.0 && quantity > 0.0) || (current_qty < 0.0 && quantity < 0.0) {
            // Increasing position
            let new_qty = current_qty + quantity;
            position.avg_entry_price = (current_qty * position.avg_entry_price + quantity * price) / new_qty;
            position.quantity = new_qty;
        } else {
            // Decreasing or flipping
            if quantity.abs() <= current_qty.abs() {
                // Pure decrease
                // Realized PnL = (exit_price - entry_price) * quantity_closed
                // If we are long (current_qty > 0), and we sell (quantity < 0), 
                // realized = (price - avg_entry) * -quantity
                realized = (price - position.avg_entry_price) * -quantity;
                position.quantity += quantity;
                if position.quantity == 0.0 {
                    position.avg_entry_price = 0.0;
                }
            } else {
                // Flip
                let qty_to_close = -current_qty;
                let qty_remaining = quantity - qty_to_close;
                
                realized = (price - position.avg_entry_price) * current_qty;
                
                // New position in opposite direction
                position.quantity = qty_remaining;
                position.avg_entry_price = price;
            }
        }

        self.realized_pnl += realized;
        realized
    }

    pub fn get_total_unrealized_pnl(&self, prices: &HashMap<Outcome, f64>) -> f64 {
        let mut total = 0.0;
        for (outcome, position) in &self.positions {
            if let Some(&price) = prices.get(outcome) {
                total += (price - position.avg_entry_price) * position.quantity;
            }
        }
        total
    }
    
    pub fn get_total_position_abs(&self) -> f64 {
        self.positions.values().map(|p| p.quantity.abs()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pnl_tracker_multi_outcome() {
        let mut tracker = PnlTracker::new();
        
        // Buy 10 Yes @ 0.6
        tracker.process_trade(Outcome::Yes, 10.0, 0.6);
        // Buy 10 No @ 0.4
        tracker.process_trade(Outcome::No, 10.0, 0.4);
        
        assert_eq!(tracker.get_total_position_abs(), 20.0);
        
        // Sell 5 Yes @ 0.7
        let r1 = tracker.process_trade(Outcome::Yes, -5.0, 0.7);
        assert!((r1 - 0.5).abs() < 1e-10); // (0.7 - 0.6) * 5 = 0.5
        
        let mut prices = HashMap::new();
        prices.insert(Outcome::Yes, 0.8);
        prices.insert(Outcome::No, 0.3);
        
        // Unrealized:
        // Yes: (0.8 - 0.6) * 5 = 1.0
        // No: (0.3 - 0.4) * 10 = -1.0
        // Total = 0.0
        assert!((tracker.get_total_unrealized_pnl(&prices)).abs() < 1e-10);
    }
}
