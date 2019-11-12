use std::collections::HashMap;
use serde::Deserialize;

/// Structure holding basic data for a single Guru.
#[derive(Deserialize, Debug)]
pub struct Guru {
    /// Unique identifier for a Guru.
    pub id: String,
    /// The Gurus name, could a person or an institution
    pub name: String,
    /// A link to the Guru's homepage, if available, otherwise None
    pub url: Option<String>,
    /// The organisation the Guru is working for.
    pub company: String,
    /// The number of stock holdings tracked by GuruFocus.
    pub num_of_stocks: String,
    /// Total investment value in million US$.
    pub value: String,
    /// Turnover rate in %.
    pub turnover: String,
    /// Date of latest update of this data.
    pub latest_update: String,
}

/// Container for all basic Guru data.
#[derive(Deserialize, Debug)]
pub struct Gurus {
    /// Map holding the list of Gurus per country.
    pub all: HashMap<String, Vec<Guru> >,
    /// For each personal list of Gurus (e.g. default and custom list),
    /// a vector of Guru IDs is stored.
    pub my:  HashMap<String, Vec<String> >
}

