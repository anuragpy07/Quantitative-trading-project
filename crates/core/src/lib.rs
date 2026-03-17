use polars::prelude::*;
use log::{info};

pub struct Backtest {
    data: DataFrame,
    strategy: Box<dyn Strategy>,
    initial_capital: f64,
    capital: f64,
    positions: Vec<Position>,
    equity_curve: Vec<f64>,
}

pub struct Position {
    #[allow(dead_code)]
    entry_price: f64,
    exit_price: Option<f64>,
    size: f64,
}

pub trait Strategy {
    fn generate_signals(&self, data: &DataFrame) -> DataFrame;
}

impl Backtest {
    pub fn new(data: DataFrame, strategy: Box<dyn Strategy>, initial_capital: f64) -> Self {
        Self {
            data,
            strategy,
            initial_capital,
            capital: initial_capital,
            positions: Vec::new(),
            equity_curve: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let signals = self.strategy.generate_signals(&self.data);
        let close_prices = self.data.column("close").unwrap().f64().unwrap();

        for i in 0..self.data.height() {
            if let (Some(signal), Some(close_price)) = (
                signals.column("signal").unwrap().i32().unwrap().get(i),
                close_prices.get(i),
            ) {
                match signal {
                    1 => { // Buy signal
                        if self.capital > 0.0 {
                            let position = Position {
                                entry_price: close_price,
                                exit_price: None,
                                size: self.capital / close_price,
                            };
                            self.positions.push(position);
                            self.capital = 0.0;
                            info!("Buying at {}", close_price);
                        }
                    }
                    -1 => { // Sell signal
                        if let Some(mut position) = self.positions.pop() {
                            position.exit_price = Some(close_price);
                            self.capital = position.size * close_price;
                            info!("Selling at {}", close_price);
                            self.positions.push(position);
                        }
                    }
                    _ => {} // Hold
                }

                let equity = self.calculate_equity(close_price);
                self.equity_curve.push(equity);
            } else {
                // If no price or signal, carry forward the last equity value
                let last_equity = self.equity_curve.last().cloned().unwrap_or(self.initial_capital);
                self.equity_curve.push(last_equity);
            }
        }
    }

    fn calculate_equity(&self, current_price: f64) -> f64 {
        let mut total_equity = self.capital;
        for position in &self.positions {
            if position.exit_price.is_none() {
                total_equity += position.size * current_price;
            }
        }
        total_equity
    }

    pub fn results(&self) -> DataFrame {
        DataFrame::new(vec![
            Series::new("equity_curve", self.equity_curve.clone()),
        ]).unwrap()
    }
}

