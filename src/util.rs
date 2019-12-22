pub mod slack {
  use chrono::{NaiveDate, ParseResult};

  pub fn parse_slack_date(slack_date: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(slack_date, "%b %e, %Y")
  }
}

pub mod io {
  use serde::de::DeserializeOwned;
  use std::fs::File;
  use std::io::prelude::*;

  pub fn parse_file<T>(path: &'static str) -> std::io::Result<T>
  where
    T: DeserializeOwned,
  {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let data = serde_json::from_str(&contents)?;

    Ok(data)
  }
}
