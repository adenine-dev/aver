#![feature(test)]

#[macro_use] extern crate aver;

extern crate test;

#[test]
fn general_log() {
  log!("oof");
}