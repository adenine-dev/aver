#![feature(test)]

#[macro_use] extern crate aver;

/// These tests should be run with `cargo test -- --test-threads 1 --nocapture`
/// this is done so you can see the output and the output doesn't mess with the automatically generated test output

#[test]
fn single_arg() {
  log_info!("this is a single arguement");
}

#[test]
fn multi_arg() {
  log_info!("these are ", "multiple ", "arguements");
}

#[test]
fn multi_type() {
  log_info!("does this have ", 3, " types? ", true);
}

#[test]
fn warning() {
  log_warn!("this is a warning that ", 3, " does not equal ", 4);
}

#[test]
fn error() {
  log_error!("this is an error!");
}

#[test]
fn should_not_print() {
  aver::set_log_level(aver::LogLevel::Fatal);
  log_error!("this should not print");
  aver::set_log_level(aver::LogLevel::Info);
}