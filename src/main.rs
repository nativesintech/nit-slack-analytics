mod members_data_calc;
mod members_data_io;
mod members_data_table;
mod members_struct;

fn main() -> std::io::Result<()> {
    members_data_io::run()?;

    Ok(())
}
