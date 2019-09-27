#[cfg(windows)] extern crate winapi;
#[cfg(windows)] use std::io::Write;

static mut CONSOLE_CONFIG: winapi::shared::minwindef::WORD = 0;

fn get_handle() -> std::io::Result<winapi::um::winnt::HANDLE> {
  use winapi::um::winnt::{ FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE };

  let handle = unsafe {
    winapi::um::fileapi::CreateFileA(
      b"CONOUT$\0".as_ptr() as *const i8, 
      GENERIC_READ | GENERIC_WRITE, 
      FILE_SHARE_WRITE, 
      std::ptr::null_mut(), 
      winapi::um::fileapi::OPEN_EXISTING, 
      0, 
      std::ptr::null_mut(),
    )
  };
  if handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
    Err(std::io::Error::last_os_error())
  } else { Ok(handle) }
}

macro_rules! make_color_fns {
  ($name:ident, $linux_code:expr, $win_code:expr) => {
    #[cfg(not(windows))]
    pub fn $name() -> String { $linux_code }

    #[cfg(windows)]
    pub fn $name() -> String {
      std::io::stdout().flush().expect("Flush stdout failed");
      unsafe {
        CONSOLE_CONFIG = $win_code;
        let handle = get_handle();
        match handle {
          Ok(h) => { winapi::um::wincon::SetConsoleTextAttribute(h, CONSOLE_CONFIG); }
          Err(e) => { return format!("{}", e) }
        };
      }
      "".to_string()
    }
  };
}

make_color_fns!(reset, "\033[00m", winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_GREEN | winapi::um::wincon::FOREGROUND_BLUE);
make_color_fns!(grey, "\033[30m", 0);
make_color_fns!(gray, "\033[30m", 0); // alternate spelling because i am never sure which is right
make_color_fns!(red, "\033[31m", winapi::um::wincon::FOREGROUND_RED);
make_color_fns!(yellow, "\033[33m", winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_GREEN);
make_color_fns!(green, "\033[32m", winapi::um::wincon::FOREGROUND_GREEN);
make_color_fns!(cyan, "\033[36m", winapi::um::wincon::FOREGROUND_GREEN | winapi::um::wincon::FOREGROUND_BLUE);
make_color_fns!(blue, "\033[34m", winapi::um::wincon::FOREGROUND_BLUE);
make_color_fns!(magenta, "\033[45m", winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_BLUE);
