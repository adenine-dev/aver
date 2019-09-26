pub fn _log<T: std::fmt::Display>(a: T) {
  print!("{}", a);
}

#[macro_export]
macro_rules! log {
  ($arg:expr) => {
    $crate::_log($arg);
  };

  ($arg0:expr, $($args:expr),+) => {
    log!($arg0);
    log!($($args),+);
  };
}

#[macro_export]
macro_rules! __generic_log {
  ($prefix:expr, $arg:expr) => {
    log!($prefix, ": ", file!(), ":", line!(), " - ", $arg, "\n");
  };

  ($prefix:expr, $($args:expr),+) => {
    log!($prefix, ": ", file!(), ":", line!(), " - ", $($args),+, "\n");
  };
}

#[macro_export]
macro_rules! log_info {
  ($arg:expr) => {
    __generic_log!("[INFO]", $arg);
  };

  ($($args:expr),+) => {
    __generic_log!("[INFO]", $($args),+);
  };
}