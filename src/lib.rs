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
  //HACK: the first arg should be the token $, this is a hack to make nested macros work
  ($d:tt, $name:ident, $prefix:expr) => {
    #[macro_export]
    macro_rules! $name {
      ($arg:expr) => {
        log!($prefix, ": ", file!(), ":", line!(), " - ", $arg, "\n");
      };

      ($d($d args:expr),+) => {
        log!($prefix, ": ", file!(), ":", line!(), " - ", $d($d args),+, "\n");
      };
    } 
  }
}

__generic_log!($, log_trace, "[TRACE]");
__generic_log!($, log_info,  "[INFO]");
__generic_log!($, log_warn,  "[WARN]");
__generic_log!($, log_error, "[ERROR]");