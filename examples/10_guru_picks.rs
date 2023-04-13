extern crate chrono;

use chrono::{Datelike, NaiveDate, Utc};
use gurufocus_api as gfapi;
use std::env;

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
    ).unwrap()
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

    // Buffett, Soros and Klarman
    let gurus = ["7", "16", "28"];
    let now = Utc::now().naive_local().date();
    let three_months_ago = month_before(now, 3);
    let page = 1;
    let trades = gf_connect
        .get_guru_picks(&gurus, three_months_ago, page)
        .await
        .unwrap();

    println!(
        "List of trades by a set of gurus (Warren Buffett, George Soros, and Seth Klarman) since {}\n{:#?}", three_months_ago, trades);
}
