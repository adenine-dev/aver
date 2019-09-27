#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum LogLevel {
  All, // practically equivalent to Trace, here for convenience
  Trace,
  Debug,
  Info,
  Warn,
  Error,
  Fatal,
  Off
}

static mut LOG_LEVEL: LogLevel = LogLevel::Info;

/// sets the log level
/// any log with a lower value than the current level will not print
/// ordered by `All` < `Trace` < `Debug` < `Info` < `Warn` < `Error` < `Fatal` < `Off`.
/// off will prevent any logging from taking place.
/// **this is unsafe**
pub fn set_log_level(level: LogLevel) {
  unsafe { LOG_LEVEL = level; }
}

/// Returns the current log level. 
/// The initial value is `LogLevel::Info`. 
/// **this is unsafe**
pub fn get_log_level() -> LogLevel {
  unsafe { return LOG_LEVEL; };
}

/// logs a single value that implements Display.
pub fn _log<T: std::fmt::Display>(a: T) {
  print!("{}", a);
}

/// logs any number of values that implements Display.
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

macro_rules! _make_log_level {
  //HACK: the first arg should be the token $, this is a hack to make nested macros work
  ($d:tt, $name:ident, $prefix:expr, $level:ty) => {
    #[macro_export]
    macro_rules! $name {
      ($arg:expr) => {
        if($crate::$level >= $crate::get_log_level()) {
          log!($prefix, ": ", file!(), ":", line!(), " - ", $arg, "\n");
        }
      };

      ($d($d args:expr),+) => {
        if($crate::$level >= $crate::get_log_level()) {
          log!($prefix, ": ", file!(), ":", line!(), " - ", $d($d args),+, "\n");
        }
      };
    } 
  }
}

_make_log_level!($, log_trace, "[TRACE]", LogLevel::Trace);
_make_log_level!($, log_debug, "[DEBUG]", LogLevel::Debug);
_make_log_level!($, log_info,  "[INFO]",  LogLevel::Info);
_make_log_level!($, log_warn,  "[WARN]",  LogLevel::Warn);
_make_log_level!($, log_error, "[ERROR]", LogLevel::Error);
_make_log_level!($, log_fatal, "[FATAL]", LogLevel::Fatal);