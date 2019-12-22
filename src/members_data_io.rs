use crate::members_data_calc;
use crate::members_data_table;

pub fn run() -> std::io::Result<()> {
  let members_data = members_data_calc::run()?;
  let table = members_data_table::run(members_data)?;

  table.printstd();

  Ok(())
}
