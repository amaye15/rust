use chrono::NaiveDate;
use polars::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use std::fs::File;



#[derive(Deserialize, Debug)]
struct StockData {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
}

async fn fetch_stock_data() -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
    let api_key = "SPE2MOF2KPKKHSHH"; // Replace with your actual API key
    let symbol = "MSFT";
    let function = "TIME_SERIES_DAILY";
    let output_size = "full";
    let url = format!(
        "https://www.alphavantage.co/query?function={}&symbol={}&outputsize={}&apikey={}",
        function, symbol, output_size, api_key
    );

    let resp = reqwest::get(&url).await?.text().await?;
    let v: Value = serde_json::from_str(&resp)?;

    let time_series = v["Time Series (Daily)"]
        .as_object()
        .ok_or("Data format error")?;

    let mut stock_data = Vec::new();
    for (date, data) in time_series {
        let open = data["1. open"].as_str().unwrap_or("0").parse::<f64>()?;
        let high = data["2. high"].as_str().unwrap_or("0").parse::<f64>()?;
        let low = data["3. low"].as_str().unwrap_or("0").parse::<f64>()?;
        let close = data["4. close"].as_str().unwrap_or("0").parse::<f64>()?;
        let volume = data["5. volume"].as_str().unwrap_or("0").parse::<i64>()?;

        stock_data.push(StockData {
            date: date.clone(),
            open,
            high,
            low,
            close,
            volume,
        });
    }

    Ok(stock_data)
}

fn stock_data_to_dataframe(stock_data: Vec<StockData>) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let dates: Vec<NaiveDate> = stock_data
        .iter()
        .map(|d| NaiveDate::parse_from_str(&d.date, "%Y-%m-%d").unwrap())
        .collect();
    let opens: Vec<f64> = stock_data.iter().map(|d| d.open).collect();
    let highs: Vec<f64> = stock_data.iter().map(|d| d.high).collect();
    let lows: Vec<f64> = stock_data.iter().map(|d| d.low).collect();
    let closes: Vec<f64> = stock_data.iter().map(|d| d.close).collect();
    let volumes: Vec<i64> = stock_data.iter().map(|d| d.volume).collect();

    let df = DataFrame::new(vec![
        DateChunked::from_naive_date("date", dates).into_series(),
        Series::new("open", &opens),
        Series::new("high", &highs),
        Series::new("low", &lows),
        Series::new("close", &closes),
        Series::new("volume", &volumes),
    ])?;

    Ok(df)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stock_data = fetch_stock_data().await?;
    let mut df = stock_data_to_dataframe(stock_data)?;

    // Specify the file path where you want to save the Parquet file
    let file_path = "stock_data.parquet";
    let mut file = File::create(file_path).unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();
    println!("");
    println!("DataFrame saved as Parquet file at: {}", file_path);
    println!("{:?}", df);
    println!("");


    Ok(())

}
