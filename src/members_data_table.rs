use prettytable::{format::Alignment, Cell, Row, Table};

use crate::member_struct::MembersData;

pub fn run(members_data: MembersData) -> std::io::Result<Table> {
  let mut joined_by_year_keys_values: Vec<(&i32, &u32)> =
    members_data.joined_by_year.iter().collect();
  joined_by_year_keys_values.sort_by_key(|(k, _v)| k.abs());

  let mut joined_by_year_keys = vec![];
  let mut joined_by_year_values = vec![];

  let mut joined_by_year = Table::new();
  for (key, value) in joined_by_year_keys_values.iter() {
    joined_by_year_keys.push(Cell::new(&key.to_string()));
    joined_by_year_values.push(Cell::new(&value.to_string()));
  }
  joined_by_year_keys.push(Cell::new("Total"));
  let joined_by_year_keys_len = joined_by_year_keys.len();
  joined_by_year_values.push(Cell::new(&members_data.total_members.to_string()));
  joined_by_year.add_row(Row::new(joined_by_year_keys));
  joined_by_year.add_row(Row::new(joined_by_year_values));
  joined_by_year.set_titles(Row::new(vec![Cell::new_align(
    "Members Joined By Year",
    Alignment::CENTER,
  )
  .with_hspan(joined_by_year_keys_len)]));

  Ok(joined_by_year)
}
