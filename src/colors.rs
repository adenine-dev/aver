#[cfg(windows)] extern crate winapi;
#[cfg(windows)] use std::io::Write;

#[cfg(windows)]
static mut CONSOLE_CONFIG: winapi::shared::minwindef::WORD = 0;

#[cfg(windows)]
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

/// creates the console color, preserving the pervious color settings if an arg is set to -1
#[cfg(windows)]
fn make_win_color(fg: i16, bg: i16) -> winapi::shared::minwindef::WORD {
  unsafe {
    let mut new_color: winapi::shared::minwindef::WORD = CONSOLE_CONFIG;
    if fg != -1 {
      new_color &= !(new_color & 0x0F);
      new_color |= fg as winapi::shared::minwindef::WORD;
    }
    if bg != -1 {
      new_color &= !(new_color & 0xF0);
      new_color |= bg as winapi::shared::minwindef::WORD;
    }
    new_color
  }
}

macro_rules! make_color_fns {
  ($name:ident, $linux_code:expr, $win_code_fg:expr, $win_code_bg:expr) => {
    #[cfg(not(windows))]
    pub fn $name() -> String { $linux_code }

    #[cfg(windows)]
    pub fn $name() -> String {
      std::io::stdout().flush().expect("Flush stdout failed!");
      unsafe {
        CONSOLE_CONFIG = make_win_color($win_code_fg as i16, $win_code_bg as i16);
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

make_color_fns!(reset, "\033[00m", winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_GREEN | winapi::um::wincon::FOREGROUND_BLUE, 0);

make_color_fns!(white, "\033[30m", winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_GREEN | winapi::um::wincon::FOREGROUND_BLUE, -1);
make_color_fns!(grey, "\033[30m", 0 | winapi::um::wincon::FOREGROUND_INTENSITY, -1);
make_color_fns!(black, "\033[30m", 0, -1);
make_color_fns!(red, "\033[31m", winapi::um::wincon::FOREGROUND_RED, -1);
make_color_fns!(yellow, "\033[33m", winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_GREEN, -1);
make_color_fns!(green, "\033[32m", winapi::um::wincon::FOREGROUND_GREEN, -1);
make_color_fns!(cyan, "\033[36m", winapi::um::wincon::FOREGROUND_GREEN | winapi::um::wincon::FOREGROUND_BLUE, -1);
make_color_fns!(blue, "\033[34m", winapi::um::wincon::FOREGROUND_BLUE, -1);
make_color_fns!(magenta, "\033[35m", winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_BLUE, -1);

make_color_fns!(on_white, "\033[107m", -1, winapi::um::wincon::BACKGROUND_RED | winapi::um::wincon::BACKGROUND_GREEN | winapi::um::wincon::BACKGROUND_BLUE);
make_color_fns!(on_grey, "\033[100m", -1, 0 | winapi::um::wincon::BACKGROUND_INTENSITY);
make_color_fns!(on_black, "\033[40m", -1, 0);
make_color_fns!(on_red, "\033[41m", -1, winapi::um::wincon::BACKGROUND_RED);
make_color_fns!(on_yellow, "\033[43m", -1, winapi::um::wincon::BACKGROUND_RED | winapi::um::wincon::BACKGROUND_GREEN);
make_color_fns!(on_green, "\033[42m", -1, winapi::um::wincon::BACKGROUND_GREEN);
make_color_fns!(on_cyan, "\033[46m", -1, winapi::um::wincon::BACKGROUND_GREEN | winapi::um::wincon::BACKGROUND_BLUE);
make_color_fns!(on_blue, "\033[44m", -1, winapi::um::wincon::BACKGROUND_BLUE);
make_color_fns!(on_magenta, "\033[45m", -1, winapi::um::wincon::BACKGROUND_RED | winapi::um::wincon::BACKGROUND_BLUE);
