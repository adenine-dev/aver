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
macro_rules! _make_log_level {
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

_make_log_level!($, log_trace, "[TRACE]");
_make_log_level!($, log_info,  "[INFO]");
_make_log_level!($, log_warn,  "[WARN]");
_make_log_level!($, log_error, "[ERROR]");