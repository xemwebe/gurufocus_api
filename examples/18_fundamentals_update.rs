extern crate chrono;

use chrono::{Datelike, NaiveDate, Utc};
use gurufocus_api as gfapi;
use std::env;

type UpdatedStocks = Vec<String>;

fn get_days_from_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .unwrap()
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days() as u32
}

fn month_before(date: NaiveDate, period: u32) -> NaiveDate {
    let mut day = date.day();
    let mut month = date.month();
    let mut year = date.year();
    if month <= period {
        year -= 1;
        month += 12 - period;
    } else {
        month -= period;
    }

    if day > 28 {
        let last_date_of_month = get_days_from_month(year, month);
        day = std::cmp::max(day, last_date_of_month);
    }
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let now = Utc::now().naive_local().date();
    let one_months_ago = month_before(now, 6);
    let stocks = gf_connect.get_updated_stocks(one_months_ago).await.unwrap();

    let stocks: UpdatedStocks = serde_json::from_value(stocks).unwrap();
    println!(
        "List of stocks with updated fundamental data since {}\n{:#?}",
        one_months_ago, stocks
    );
}
