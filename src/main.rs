mod member_struct;
mod members_data_calc;
mod members_data_io;
mod members_data_table;

fn main() -> std::io::Result<()> {
    members_data_io::run()?;

    Ok(())
}
