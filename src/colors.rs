#[cfg(windows)] extern crate winapi;


#[cfg(windows)]
mod colors {
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

  pub fn blue() -> String {
    unsafe {
      CONSOLE_CONFIG |= winapi::um::wincon::FOREGROUND_BLUE;
      let handle = get_handle();
      match handle {
        Ok(h) => { winapi::um::wincon::SetConsoleTextAttribute(h, CONSOLE_CONFIG); }
        Err(e) => { return format!("{}", e) }
      };
    }
    "".to_string()
  }
}

#[cfg(not(windows))]
mod colors {
  fn blue() -> String { "\033[34m" }
}

// hack to not have to write the cfg every line  
pub use colors::*;