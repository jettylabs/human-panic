use assert_cli;

#[test]
fn release() {
  assert_cli::Assert::command(&["cargo", "run", "--release"])
    .stderr()
    .contains("passthrough-panic-test")
    .stdout()
    .contains("Something goes here")
    .fails_with(101)
    .unwrap();
}

#[test]
fn debug() {
  assert_cli::Assert::command(&["cargo", "run"])
    .stderr()
    .contains("OMG EVERYTHING IS ON FIRE!!!")
    .stdout()
    .doesnt_contain("Something goes here")
    .fails_with(101)
    .unwrap();
}
