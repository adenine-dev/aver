#[macro_export]
macro_rules! log {
  ($arg:expr) => {
    $crate::_log($arg);
  }
}

pub fn _log<T: std::fmt::Display>(a: T) {
  print!("{}", a);
}