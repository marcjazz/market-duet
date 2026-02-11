use std::collections::HashMap;
use crate::strategy::Outcome;

pub struct RiskLimits {
    pub max_position: f64,
    pub max_drawdown: f64,
    pub max_exposure_per_outcome: f64,
}

pub struct RiskEngine {
    pub limits: RiskLimits,
    pub current_position: f64,
    pub peak_pnl: f64,
    pub current_pnl: f64,
    pub exposure_per_outcome: HashMap<Outcome, f64>,
}

impl RiskEngine {
    pub fn new(limits: RiskLimits) -> Self {
        Self {
            limits,
            current_position: 0.0,
            peak_pnl: 0.0,
            current_pnl: 0.0,
            exposure_per_outcome: HashMap::new(),
        }
    }

    pub fn check_trade(&self, outcome: Outcome, quantity: f64) -> bool {
        // 1. Check total position limit
        let new_position = (self.current_position + quantity).abs();
        if new_position > self.limits.max_position {
            return false;
        }

        // 2. Check exposure per outcome
        let current_exposure = self.exposure_per_outcome.get(&outcome).cloned().unwrap_or(0.0);
        let new_exposure = (current_exposure + quantity).abs();
        if new_exposure > self.limits.max_exposure_per_outcome {
            return false;
        }

        // 3. Check drawdown
        if !self.check_drawdown() {
            return false;
        }

        true
    }

    pub fn check_drawdown(&self) -> bool {
        let drawdown = self.peak_pnl - self.current_pnl;
        drawdown <= self.limits.max_drawdown
    }

    pub fn update(&mut self, position: f64, pnl: f64, outcome: Outcome, outcome_qty: f64) {
        self.current_position = position;
        self.current_pnl = pnl;
        if self.current_pnl > self.peak_pnl {
            self.peak_pnl = self.current_pnl;
        }
        
        let entry = self.exposure_per_outcome.entry(outcome).or_insert(0.0);
        *entry += outcome_qty;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_limits() {
        let limits = RiskLimits {
            max_position: 10.0,
            max_drawdown: 100.0,
            max_exposure_per_outcome: 5.0,
        };
        let mut engine = RiskEngine::new(limits);

        // Initial state
        assert!(engine.check_trade(Outcome::Yes, 5.0));
        assert!(!engine.check_trade(Outcome::Yes, 6.0)); // Exceeds outcome exposure
        assert!(engine.check_drawdown());

        // Update state
        engine.update(5.0, 50.0, Outcome::Yes, 5.0);
        assert_eq!(engine.current_position, 5.0);
        assert_eq!(engine.current_pnl, 50.0);
        assert_eq!(engine.peak_pnl, 50.0);
        assert_eq!(*engine.exposure_per_outcome.get(&Outcome::Yes).unwrap(), 5.0);

        // Check trade with existing position
        assert!(!engine.check_trade(Outcome::Yes, 1.0)); // Exceeds outcome exposure (5+1 > 5)
        assert!(engine.check_trade(Outcome::No, 5.0)); // OK

        // Drawdown test
        engine.update(5.0, -60.0, Outcome::Yes, 0.0); // PnL dropped to -60
        assert!(!engine.check_drawdown()); // Drawdown is 110, limit is 100
        assert!(!engine.check_trade(Outcome::No, 1.0)); // Should fail due to drawdown
    }
}
