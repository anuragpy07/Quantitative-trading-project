<p align="center">
  <img 
    src="https://readme-typing-svg.herokuapp.com?font=Fira+Code&weight=700&size=50&pause=1000&color=FF4B11&center=true&vCenter=true&width=1000&height=70&lines=📊+Quantitative+Trading+Framework" 
  />
  <br/>
  <img 
    src="https://readme-typing-svg.herokuapp.com?font=Fira+Code&size=24&duration=3000&pause=1000&color=E2E8F0&center=true&vCenter=true&width=1000&lines=Memory-Safe+High-Frequency+Backtesting;Real-time+Alpha+Generation;Ultra-Low+Latency+Execution;Zero-Cost+Abstractions+for+FinTech" 
  />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge" />
  <img src="https://img.shields.io/badge/polars-data-orange.svg?style=for-the-badge" />
  <img src="https://img.shields.io/badge/build-passing-brightgreen.svg?style=for-the-badge" />
</p>

<div align="center">
  <img src="docs/images/hero.jpg" alt="Quantitative Trading" width="800"/>
</div>

<div align="center">
  <h3>🚀 A high-performance quantitative trading framework built in Rust</h3>
  <p>Develop, backtest, and optimize trading strategies with blazing speed and type safety</p>
</div>

---

## 📝 Description

`quantitative-trading-rust` is a comprehensive open-source framework for developing quantitative trading strategies. Built in Rust, it offers **high performance**, **type safety**, and **modular architecture** for market analysis, technical indicator calculation, and strategy backtesting.

### ✨ Key Features

* 🎯 **Efficient Backtesting Engine** - Simulate strategies with historical data
* 📈 **Technical Indicators** - SMA, EMA, RSI implemented natively
* 💾 **Data Management** - Integration with market APIs and CSV
* 🏗️ **Modular Architecture** - Cargo workspace with independent crates
* ⚡ **High Performance** - Leverage Rust's speed

---

## 🚀 Quick Start

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) 1.70+
* Git

### Installation

```bash
# Clone the repository
git clone https://github.com/anuragpy07/quantitative-trading-rust.git
cd quantitative-trading-rust

# Run the SMA crossover example
cargo run --example sma_crossover
```

### Example Output

```
Loaded 20 rows of data

Backtest Results:
shape: (20, 1)
┌──────────────┐
│ equity_curve │
│ ---          │
│ f64          │
╞══════════════╡
│ 10000.0      │
│ 10000.0      │
│ 10000.0      │
│ ...          │
└──────────────┘
```

---

## 📚 Usage Example

### Creating a Simple Moving Average Strategy

```rust
use rqt_indicators::sma;
use rqt_core::{Backtest, Strategy};
use polars::prelude::*;

struct SmaCrossover {
    short_window: usize,
    long_window: usize,
}

impl Strategy for SmaCrossover {
    fn generate_signals(&self, data: &DataFrame) -> DataFrame {
        let close_series = data.column("close").unwrap();
        let close: Vec<f64> = close_series.f64().unwrap()
            .into_no_null_iter().collect();

        let short_sma = sma(&close, self.short_window);
        let long_sma = sma(&close, self.long_window);

        // Generate buy/sell signals based on SMA crossover
        // ...
    }
}

fn main() {
    let data = CsvReader::from_path("data/sample_data.csv")?.finish()?;
    let strategy = Box::new(SmaCrossover { 
        short_window: 5, 
        long_window: 10 
    });
    
    let mut backtest = Backtest::new(data, strategy, 10000.0);
    backtest.run();
    
    println!("{}", backtest.results());
}
```

---

## 🏗️ Architecture

The project follows a modular workspace structure with clear separation of concerns:

```mermaid
flowchart LR
    A[Data Feed\nCSV / Market API] --> B[Quantitative Analysis\nSMA / EMA / RSI]
    B --> C[Strategy\nSMA Crossover / Custom]
    C --> D[Signal Generation\nBuy / Sell / Hold]
    D --> E[Execution\nBacktest Engine]
    E --> F[P&L\nEquity Curve / Metrics]
```

### Project Structure

```
quantitative-trading-rust/
├── crates/
│   ├── core/          # Backtesting engine & portfolio management
│   ├── data/          # Data loading & API integrations
│   ├── indicators/    # Technical indicators library
│   └── utils/         # Logging & utilities
├── examples/          # Example strategies
├── data/              # Sample data files
└── docs/              # Documentation & images
```

### Crate Descriptions

| Crate              | Description                                           |
| ------------------ | ----------------------------------------------------- |
| **rqt-core**       | Backtesting engine, position tracking, strategy trait |
| **rqt-data**       | Data loading from CSV and external APIs               |
| **rqt-indicators** | Technical indicators (SMA, EMA, RSI)                  |
| **rqt-utils**      | Logging and utility functions                         |

---

## 📊 Supported Indicators

* ✅ **SMA** (Simple Moving Average)
* ✅ **EMA** (Exponential Moving Average)
* ✅ **RSI** (Relative Strength Index)
* 🔜 **MACD** (Moving Average Convergence Divergence)
* 🔜 **Bollinger Bands**
* 🔜 **Stochastic Oscillator**

---

## 🛣️ Roadmap

* [ ] Add more technical indicators (MACD, Bollinger Bands, Stochastic)
* [ ] Implement advanced backtesting features (slippage, commissions)
* [ ] Add risk management modules (stop-loss, position sizing)
* [ ] Integration with live trading APIs (Binance, Interactive Brokers)
* [ ] Performance metrics (Sharpe Ratio, Sortino, Max Drawdown)
* [ ] Web dashboard for strategy visualization
* [ ] Machine learning integration for signal prediction

---

## 🤝 Contributing

Contributions are welcome! Feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👨‍💻 Author

**Anurag Kumar**
* 💼 Quantitative Researcher | Problem Solver | Software Developer
* 🔗 https://github.com/anuragpy07

---

## 🙏 Acknowledgments

* Built with Rust
* Data processing with Polars
* Inspired by quantitative finance best practices

---

<div align="center">
  <p>Made with ❤️ and Rust</p>
  <p>⭐ Star this repository if you find it useful!</p>
</div>
