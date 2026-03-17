use polars::prelude::*;
use rqt_core::{Backtest, Strategy};
use rqt_indicators::sma;
use rqt_utils::setup_logger;
use anyhow::Result;

struct SmaCrossover {
    short_window: usize,
    long_window: usize,
}

impl Strategy for SmaCrossover {
    fn generate_signals(&self, data: &DataFrame) -> DataFrame {
        let close_series = data.column("close").unwrap();
        let close: Vec<f64> = close_series.f64().unwrap().into_no_null_iter().collect();

        let short_sma = sma(&close, self.short_window);
        let long_sma = sma(&close, self.long_window);

        let mut signal_values = vec![0i32; data.height()];

        for i in self.long_window..data.height() {
            if let (Some(short_val), Some(long_val)) = (short_sma[i], long_sma[i]) {
                if short_val > long_val {
                    signal_values[i] = 1;
                } else if short_val < long_val {
                    signal_values[i] = -1;
                }
            }
        }

        let signal_series = Series::new("signal", signal_values);
        DataFrame::new(vec![signal_series]).unwrap()
    }
}

fn main() -> Result<()> {
    setup_logger();

    // Load data from CSV file
    let data = CsvReader::from_path("data/sample_data.csv")?
        .finish()?;

    println!("Loaded {} rows of data", data.height());

    let strategy = Box::new(SmaCrossover { short_window: 5, long_window: 10 });

    let mut backtest = Backtest::new(data.clone(), strategy, 10000.0);
    backtest.run();

    let results = backtest.results();
    println!("\nBacktest Results:\n{}", results);

    Ok(())
}
