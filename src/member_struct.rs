use chrono::{prelude::*, Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MembersData {
  pub total_members: u32,
  pub joined_this_year: u32,
  pub joined_by_year: HashMap<i32, u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
  #[serde(alias = "Name")]
  name: String,
  #[serde(alias = "What I Do")]
  occupation: String,
  #[serde(alias = "Account type")]
  account_type: String,
  #[serde(alias = "Account created")]
  account_created: String,
  #[serde(alias = "Days active")]
  days_active: String,
  #[serde(alias = "Messages posted")]
  messages_posted: u32,
}

pub type Members = Vec<Member>;

pub trait DataTransforms {
  fn total_members(&self) -> u32;
  fn joined_this_year(&self) -> u32;
  fn joined_by_year(&self) -> HashMap<i32, u32>;
}

impl DataTransforms for Members {
  fn total_members(&self) -> u32 {
    self.iter().fold(0, |acc, s| {
      if s.account_type == "Member" {
        let res = acc + 1;
        res
      } else {
        acc
      }
    })
  }

  fn joined_this_year(&self) -> u32 {
    self.iter().fold(0, |acc, s| {
      let joined_this_year = NaiveDate::parse_from_str(&s.account_created, "%b %e, %Y")
        .map(|d| d.year() == Utc::now().year())
        .unwrap_or_default();

      if joined_this_year {
        let res = acc + 1;
        res
      } else {
        acc
      }
    })
  }

  fn joined_by_year(&self) -> HashMap<i32, u32> {
    let mut joined_by_year = HashMap::new();

    for member in self.iter() {
      let year_joined = NaiveDate::parse_from_str(&member.account_created, "%b %e, %Y")
        .map(|d| d.year())
        .unwrap_or_default();

      match joined_by_year.get_mut(&year_joined) {
        None => {
          joined_by_year.insert(year_joined, 1);
        }
        Some(count) => {
          *count = *count + 1;
        }
      };
    }

    joined_by_year
  }
}
