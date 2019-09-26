#![feature(test)]
/// These tests should be run with `cargo test -- --no-capture` (not a typo) so you can see the output
#[macro_use] extern crate aver;

extern crate test;

#[test]
fn single_arg() {
  log!("this is a single arguement");
}

#[test]
fn multi_arg() {
  log!("these are", "multiple", "arguements");
}

#[test]
fn multi_type() {
  log!("these have", 3, "types", true);
}