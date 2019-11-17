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
fn fatal() {
  log_fatal!("this is an fatal!");
}

#[test]
fn should_not_print() {
  aver::set_log_level(aver::LogLevel::Fatal);
  log_error!("this should not print");
  aver::set_log_level(aver::LogLevel::Info);
}

#[test]
fn trace_and_debug_print() {
  aver::set_log_level(aver::LogLevel::All);
  log_trace!("this is a trace");
  log_debug!("this is a debug");
  aver::set_log_level(aver::LogLevel::Info);
}

#[test]
fn colors() {
  log!("\n"); // make it not print on the same line
  log!(aver::colors::grey(),    "grey ",    aver::colors::reset(), aver::colors::on_grey(),    "on grey",    aver::colors::reset(), "\n");
  log!(aver::colors::red(),     "red ",     aver::colors::reset(), aver::colors::on_red(),     "on red",     aver::colors::reset(), "\n");
  log!(aver::colors::yellow(),  "yellow ",  aver::colors::reset(), aver::colors::on_yellow(),  "on yellow",  aver::colors::reset(), "\n");
  log!(aver::colors::green(),   "green ",   aver::colors::reset(), aver::colors::on_green(),   "on green",   aver::colors::reset(), "\n");
  log!(aver::colors::cyan(),    "cyan ",    aver::colors::reset(), aver::colors::on_cyan(),    "on cyan",    aver::colors::reset(), "\n");
  log!(aver::colors::blue(),    "blue ",    aver::colors::reset(), aver::colors::on_blue(),    "on blue",    aver::colors::reset(), "\n");
  log!(aver::colors::magenta(), "magenta ", aver::colors::reset(), aver::colors::on_magenta(), "on magenta", aver::colors::reset(), "\n");
  log!(aver::colors::white(),   "white ",   aver::colors::reset(), aver::colors::on_white(),   "on white",   aver::colors::reset(), "\n");
  log!(aver::colors::reset(),   "reset\n");
}