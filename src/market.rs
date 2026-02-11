use rand::distr::{Distribution, Uniform};

#[derive(Debug, Clone, Copy)]
pub struct Level {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }

    pub fn best_bid(&self) -> Option<f64> {
        self.bids.first().map(|l| l.price)
    }

    pub fn best_ask(&self) -> Option<f64> {
        self.asks.first().map(|l| l.price)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

pub struct Exchange {
    pub name: String,
    pub order_book: OrderBook,
    mid_price: f64,
    spread: f64,
    depth: usize,
}

impl Exchange {
    pub fn new(name: &str, initial_price: f64, spread: f64) -> Self {
        let mut exchange = Self {
            name: name.to_string(),
            order_book: OrderBook::new(),
            mid_price: initial_price,
            spread,
            depth: 10, // Default depth for simulation
        };
        exchange.generate_order_book();
        exchange
    }

    fn generate_order_book(&mut self) {
        let mut bids = Vec::with_capacity(self.depth);
        let mut asks = Vec::with_capacity(self.depth);
        
        let tick_size = 0.01;
        let base_qty = 10.0;

        for i in 0..self.depth {
            let offset = (i as f64) * tick_size;
            
            // Bids
            bids.push(Level {
                price: self.mid_price - (self.spread / 2.0) - offset,
                quantity: base_qty + (i as f64), // Increasing liquidity deeper in the book
            });

            // Asks
            asks.push(Level {
                price: self.mid_price + (self.spread / 2.0) + offset,
                quantity: base_qty + (i as f64),
            });
        }

        self.order_book.bids = bids;
        self.order_book.asks = asks;
    }

    pub fn update_price(&mut self) {
        let mut rng = rand::rng();
        let dist = Uniform::new(-0.05, 0.05).unwrap(); // Smaller steps for more realism
        let change: f64 = dist.sample(&mut rng);
        
        self.mid_price += change;
        
        // Ensure price doesn't go too low
        if self.mid_price < self.spread {
            self.mid_price = self.spread;
        }

        self.generate_order_book();
    }

    /// Simulates filling an order against the order book.
    /// Returns a vector of (price, quantity) fills.
    pub fn fill_order(&mut self, side: Side, mut quantity: f64) -> Vec<(f64, f64)> {
        let mut fills = Vec::new();
        
        match side {
            Side::Buy => {
                // Buy from asks
                let mut i = 0;
                while quantity > 0.0 && i < self.order_book.asks.len() {
                    let level = &mut self.order_book.asks[i];
                    let fill_qty = f64::min(quantity, level.quantity);
                    
                    fills.push((level.price, fill_qty));
                    level.quantity -= fill_qty;
                    quantity -= fill_qty;
                    
                    if level.quantity <= 0.0 {
                        i += 1;
                    }
                }
                // Remove empty levels
                self.order_book.asks.drain(0..i);
            }
            Side::Sell => {
                // Sell to bids
                let mut i = 0;
                while quantity > 0.0 && i < self.order_book.bids.len() {
                    let level = &mut self.order_book.bids[i];
                    let fill_qty = f64::min(quantity, level.quantity);
                    
                    fills.push((level.price, fill_qty));
                    level.quantity -= fill_qty;
                    quantity -= fill_qty;
                    
                    if level.quantity <= 0.0 {
                        i += 1;
                    }
                }
                // Remove empty levels
                self.order_book.bids.drain(0..i);
            }
        }
        
        fills
    }
}

// Compatibility struct for existing strategy logic
#[derive(Debug, Clone, Copy)]
pub struct TopOfBook {
    pub best_bid: f64,
    pub best_ask: f64,
}

impl Exchange {
    pub fn get_tob(&self) -> TopOfBook {
        TopOfBook {
            best_bid: self.order_book.best_bid().unwrap_or(0.0),
            best_ask: self.order_book.best_ask().unwrap_or(0.0),
        }
    }
}
