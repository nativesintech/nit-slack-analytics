pub mod channels;
pub mod members;
pub mod util;

pub fn run() -> std::io::Result<()> {
  members::members_data_io::run("./src/data/members-all-time.json")?;
  channels::channels_data_io::run("./src/data/channels-all-time.json")?;

  Ok(())
}
