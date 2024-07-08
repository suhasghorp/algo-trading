#![allow(dead_code)]
#![allow(unused_variables)]

use app_config::app_config::AppConfig;

mod backtest_historical_data;
mod backtest_service;
mod market_data_manager;

fn main() {
    let config = AppConfig::new().expect("Could not load config");
    println!("Config:\n{:?}", config);

    let access_token = config.access_token;
    let sandbox_token = config.sandbox_token;
}
