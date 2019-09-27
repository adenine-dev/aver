# Aver
A dead simple logging library for rust. Currently supports colors, logging levels, and a simpler syntax. This library requires no dependencies (except for winapi if you are using windows.)

## Installation
simply add this to your Cargo.toml
```
aver = "0.1.1"
```

## Usage
You can just start using aver in your project by including it with `extern crate aver`. Then using one of its commands.
```rust
log_trace!("this is a trace method, it is disabled by default (see below)");
log_debug!("same as above!");
log_info!("any of these can take ", "any ", "number of arguments");
log_warn!("can they be of any type? ", true);
log_error!("An error");
log_fatal!("Even worse than ana error!");
```

### Log levels
Aver comes with 8 logging levels: `All`, `Trace`, `Debug`, `Info`, `Warn`, `Error`, `Fatal`, `Off`. In order of increasing precedence. You can set the log level with `aver::set_log_level(LogLevel)`, by default aver uses `Info` meaning that trace and debug will be disabled.