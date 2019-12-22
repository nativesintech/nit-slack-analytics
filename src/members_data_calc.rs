use std::fs::File;
use std::io::prelude::*;

use crate::member_struct::{DataTransforms, Members, MembersData};

pub fn run() -> std::io::Result<MembersData> {
  let mut file = File::open("./src/data/members-all-time.json")?;
  let mut contents = String::new();

  file.read_to_string(&mut contents)?;

  let data: Members = serde_json::from_str(&contents)?;

  let total_members = data.total_members();
  let joined_this_year = data.joined_this_year();
  let joined_by_year = data.joined_by_year();

  let data = MembersData {
    total_members,
    joined_this_year,
    joined_by_year,
  };

  Ok(data)
}
