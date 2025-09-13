use gurufocus_api as gfapi;
use std::{convert::TryFrom, env};
use time::{Date, UtcDateTime};

fn get_months_before(date: Date, months: u8) -> Date {
    let mut day = date.day();
    let mut month = date.month() as u8;
    let mut year = date.year();

    while month + 1 < months {
        month += 12;
        year -= 1;
    }

    month -= months;
    let month: time::Month = time::Month::try_from(month).unwrap();
    day = day.min(month.length(year));

    Date::from_calendar_date(year, month, day).unwrap()
}

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    // Buffett, Soros and Klarman
    let gurus = ["7", "16", "28"];
    let now = UtcDateTime::now();
    let three_months_ago = get_months_before(now.date(), 3);
    let page = 1;
    let trades = gf_connect
        .get_guru_picks(&gurus, three_months_ago, page)
        .await
        .unwrap();

    println!(
        "List of trades by a set of gurus (Warren Buffett, George Soros, and Seth Klarman) since {}\n{:#?}", three_months_ago, trades);
}
