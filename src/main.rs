mod channels;
mod members;
mod util;

fn main() -> std::io::Result<()> {
    members::members_data_io::run()?;
    channels::channels_data_io::run()?;

    Ok(())
}
