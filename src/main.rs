mod members;

fn main() -> std::io::Result<()> {
    members::members_data_io::run()?;

    Ok(())
}
