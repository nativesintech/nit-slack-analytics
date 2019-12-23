mod channels_struct {
  use serde::{Deserialize, Serialize};
  use std::collections::HashMap;

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Channel {
    #[serde(alias = "Name")]
    name: String,
    #[serde(alias = "Created")]
    created: String,
    #[serde(alias = "Total membership")]
    total_members: u32,
    #[serde(alias = "Messages posted")]
    messages_posted: u32,
  }

  pub type Channels = Vec<Channel>;

  pub trait DataTransforms {
    fn total_messages(&self) -> u32;
    fn messages_by_channel(&self) -> HashMap<String, u32>;
  }

  impl DataTransforms for Channels {
    fn total_messages(&self) -> u32 {
      self.iter().fold(0, |acc, curr| acc + curr.messages_posted)
    }

    fn messages_by_channel(&self) -> HashMap<String, u32> {
      let mut hash_map = HashMap::new();

      for channel in self.iter() {
        hash_map.insert(channel.name.to_owned(), channel.messages_posted);
      }

      hash_map
    }
  }

  pub struct ChannelsData {
    pub total_messages: u32,
    pub messages_by_channel: HashMap<String, u32>,
  }
}

mod channels_data_calc {
  use crate::channels::channels_struct::{Channels, ChannelsData, DataTransforms};
  use crate::util::io::parse_file;

  pub fn run(path: &'static str) -> std::io::Result<ChannelsData> {
    let data = parse_file::<Channels>(path)?;

    let total_messages = data.total_messages();
    let messages_by_channel = data.messages_by_channel();

    let data = ChannelsData {
      total_messages,
      messages_by_channel,
    };

    Ok(data)
  }
}

mod channles_data_table {
  use crate::channels::channels_struct::ChannelsData;
  use prettytable::{format::Alignment, Cell, Row, Table};

  pub fn run(channels_data: ChannelsData) -> std::io::Result<Table> {
    let total_messages = channels_data.total_messages;
    let messages_by_channel = channels_data.messages_by_channel;

    let mut messages_by_channel_key_values: Vec<_> = messages_by_channel.iter().collect();
    messages_by_channel_key_values.sort_by(|a, b| b.1.cmp(a.1));

    let mut table = Table::new();

    let mut messages_by_channel_header = vec![];
    let mut messages_by_channel_value = vec![];
    for (k, v) in messages_by_channel_key_values.iter() {
      messages_by_channel_header.push(Cell::new(&k.to_string()));
      messages_by_channel_value.push(Cell::new(&v.to_string()));
    }

    messages_by_channel_header.truncate(5);
    messages_by_channel_header.reverse();
    messages_by_channel_value.truncate(5);
    messages_by_channel_value.reverse();

    messages_by_channel_header.push(Cell::new("Total"));
    messages_by_channel_value.push(Cell::new(&total_messages.to_string()));

    table.add_row(Row::new(messages_by_channel_header));
    table.add_row(Row::new(messages_by_channel_value));

    table.set_titles(Row::new(vec![Cell::new_align(
      "Posted Messages by Channel",
      Alignment::CENTER,
    )
    .with_hspan(6)]));

    Ok(table)
  }
}

pub mod channels_data_io {
  pub fn run(path: &'static str) -> std::io::Result<()> {
    use crate::channels::channels_data_calc;
    use crate::channels::channles_data_table;

    let data = channels_data_calc::run(path)?;
    let table = channles_data_table::run(data)?;

    table.printstd();

    Ok(())
  }
}
