use polars::prelude::*;
use reqwest;
use serde::Deserialize;

use anyhow::Result;

#[derive(Debug, Deserialize)]
struct YahooFinanceResponse {
    chart: Chart,
}

#[derive(Debug, Deserialize)]
struct Chart {
    result: Vec<ChartResult>,
}

#[derive(Debug, Deserialize)]
struct ChartResult {
    timestamp: Vec<i64>,
    indicators: Indicators,
}

#[derive(Debug, Deserialize)]
struct Indicators {
    quote: Vec<Quote>,
}

#[derive(Debug, Deserialize)]
struct Quote {
    open: Vec<Option<f64>>,
    high: Vec<Option<f64>>,
    low: Vec<Option<f64>>,
    close: Vec<Option<f64>>,
    volume: Vec<Option<i64>>,
}

pub async fn fetch_yahoo_finance_data(symbol: &str, from: &str, to: &str) -> Result<DataFrame> {
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?period1={}&period2={}&interval=1d",
        symbol,
        from,
        to
    );

    let response = reqwest::get(&url).await?.text().await?;
    let data: YahooFinanceResponse = serde_json::from_str(&response)?;

    let chart_result = &data.chart.result[0];
    let timestamps = &chart_result.timestamp;
    let quotes = &chart_result.indicators.quote[0];

    let df = DataFrame::new(vec![
        Series::new("timestamp", timestamps),
        Series::new("open", quotes.open.clone()),
        Series::new("high", quotes.high.clone()),
        Series::new("low", quotes.low.clone()),
        Series::new("close", quotes.close.clone()),
        Series::new("volume", quotes.volume.clone()),
    ])?;

    Ok(df)
}

