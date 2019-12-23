use nit_slack_data::channels;

#[test]
fn test_channels_io() {
  let io = channels::channels_data_io::run("./tests/data/channels-all-time.json");
  assert!(io.is_ok());
}
