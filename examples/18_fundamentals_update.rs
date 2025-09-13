use gurufocus_api as gfapi;
use std::{convert::TryFrom, env};
use time::{Date, UtcDateTime};

type UpdatedStocks = Vec<String>;

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

    let now = UtcDateTime::now().date();
    let one_months_ago = get_months_before(now, 6);
    let stocks = gf_connect.get_updated_stocks(one_months_ago).await.unwrap();

    let stocks: UpdatedStocks = serde_json::from_value(stocks).unwrap();
    println!(
        "List of stocks with updated fundamental data since {}\n{:#?}",
        one_months_ago, stocks
    );
}
