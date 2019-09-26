#![feature(test)]

#[macro_use] extern crate aver;

/// These tests should be run with `cargo test -- --no-capture` (not a typo) so you can see the output

#[test]
fn single_arg() {
  log_info!("this is a single arguement");
}

#[test]
fn multi_arg() {
  log_info!("these are", "multiple", "arguements");
}

#[test]
fn multi_type() {
  log_info!("these have", 3, "types", true);
}