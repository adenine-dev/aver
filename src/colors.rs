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
    pub fn $name() -> String { 
      $linux_code.to_string()
    }

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

struct ColorString(String);

impl std::fmt::Display for ColorString {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      f.write_str(&self.0)?;
      Ok(())
  }
}

impl From<String> for ColorString {
  #[inline]
  fn from(s: String) -> Self {
      Self {
        0: s
      }
  }
}

impl ColorString {
  pub fn new(s: &str) -> Self {
      Self {
        0: s.to_string()
      }
  }
}


make_color_fns!(reset, ColorString::new("\x1B[00m"), winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_GREEN | winapi::um::wincon::FOREGROUND_BLUE, 0);

make_color_fns!(white, ColorString::new("\x1B[97m"), winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_GREEN | winapi::um::wincon::FOREGROUND_BLUE, -1);
make_color_fns!(grey, ColorString::new("\x1B[37m"), 0 | winapi::um::wincon::FOREGROUND_INTENSITY, -1);
make_color_fns!(black, ColorString::new("\x1B[30m"), 0, -1);
make_color_fns!(red, ColorString::new("\x1B[31m"), winapi::um::wincon::FOREGROUND_RED, -1);
make_color_fns!(yellow, ColorString::new("\x1B[33m"), winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_GREEN, -1);
make_color_fns!(green, ColorString::new("\x1B[32m"), winapi::um::wincon::FOREGROUND_GREEN, -1);
make_color_fns!(cyan, ColorString::new("\x1B[36m"), winapi::um::wincon::FOREGROUND_GREEN | winapi::um::wincon::FOREGROUND_BLUE, -1);
make_color_fns!(blue, ColorString::new("\x1B[34m"), winapi::um::wincon::FOREGROUND_BLUE, -1);
make_color_fns!(magenta, ColorString::new("\x1B[35m"), winapi::um::wincon::FOREGROUND_RED | winapi::um::wincon::FOREGROUND_BLUE, -1);

make_color_fns!(on_white, ColorString::new("\x1B[107m"), -1, winapi::um::wincon::BACKGROUND_RED | winapi::um::wincon::BACKGROUND_GREEN | winapi::um::wincon::BACKGROUND_BLUE);
make_color_fns!(on_grey, ColorString::new("\x1B[100m"), -1, 0 | winapi::um::wincon::BACKGROUND_INTENSITY);
make_color_fns!(on_black, ColorString::new("\x1B[40m"), -1, 0);
make_color_fns!(on_red, ColorString::new("\x1B[41m"), -1, winapi::um::wincon::BACKGROUND_RED);
make_color_fns!(on_yellow, ColorString::new("\x1B[43m"), -1, winapi::um::wincon::BACKGROUND_RED | winapi::um::wincon::BACKGROUND_GREEN);
make_color_fns!(on_green, ColorString::new("\x1B[42m"), -1, winapi::um::wincon::BACKGROUND_GREEN);
make_color_fns!(on_cyan, ColorString::new("\x1B[46m"), -1, winapi::um::wincon::BACKGROUND_GREEN | winapi::um::wincon::BACKGROUND_BLUE);
make_color_fns!(on_blue, ColorString::new("\x1B[44m"), -1, winapi::um::wincon::BACKGROUND_BLUE);
make_color_fns!(on_magenta, ColorString::new("\x1B[45m"), -1, winapi::um::wincon::BACKGROUND_RED | winapi::um::wincon::BACKGROUND_BLUE);
