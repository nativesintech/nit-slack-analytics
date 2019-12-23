mod members_struct {
    use crate::util::slack::parse_slack_date;
    use chrono::{prelude::*, Datelike};
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
                    acc + 1
                } else {
                    acc
                }
            })
        }

        fn joined_this_year(&self) -> u32 {
            self.iter().fold(0, |acc, s| {
                let joined_this_year = parse_slack_date(&s.account_created)
                    .map(|d| d.year() == Utc::now().year())
                    .unwrap_or_default();

                if joined_this_year {
                    acc + 1
                } else {
                    acc
                }
            })
        }

        fn joined_by_year(&self) -> HashMap<i32, u32> {
            let mut joined_by_year = HashMap::new();

            for member in self.iter() {
                let year_joined = parse_slack_date(&member.account_created)
                    .map(|d| d.year())
                    .unwrap_or_default();

                match joined_by_year.get_mut(&year_joined) {
                    None => {
                        joined_by_year.insert(year_joined, 1);
                    }
                    Some(count) => {
                        *count += 1;
                    }
                };
            }

            joined_by_year
        }
    }
}

pub mod members_data_calc {
    use crate::members::members_struct::{DataTransforms, Members, MembersData};
    use crate::util::io::parse_file;

    pub fn run(path: &'static str) -> std::io::Result<MembersData> {
        let data: Members = parse_file::<Members>(path)?;

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
}

mod members_data_table {
    use crate::members::members_struct::MembersData;
    use prettytable::{format::Alignment, Cell, Row, Table};

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
}

pub mod members_data_io {
    use crate::members::members_data_calc;
    use crate::members::members_data_table;

    pub fn run(path: &'static str) -> std::io::Result<()> {
        let members_data = members_data_calc::run(path)?;
        let table = members_data_table::run(members_data)?;

        table.printstd();

        Ok(())
    }
}
