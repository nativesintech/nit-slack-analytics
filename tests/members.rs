use nit_slack_data::members;

#[test]
fn test_members_io() {
  let io = members::members_data_io::run("./tests/data/members-all-time.json");
  assert!(io.is_ok());
}

#[test]
fn test_members_cacl() {
  let data = members::members_data_calc::run("./tests/data/members-all-time.json").unwrap();
  assert_eq!(data.total_members, 2);
}
