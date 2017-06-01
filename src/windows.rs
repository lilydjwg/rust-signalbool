extern crate winapi;
extern crate kernel32;

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::mem::transmute;

use self::winapi::{BOOL, DWORD, TRUE, FALSE};
use self::kernel32::SetConsoleCtrlHandler;

use ::Flag;
use ::SignalBool;

#[derive(PartialEq)]
pub enum Signal {
  SIGINT,
}

static mut SIGNAL: usize = 0;

extern "system" fn os_handler(_: DWORD) -> BOOL {
  let sb: Arc<AtomicBool> = unsafe {
    transmute(SIGNAL)
  };
  sb.store(true, Ordering::Relaxed);
  TRUE
}

impl SignalBool {
  /// Register an array of signals to set the internal flag to true when received.
  pub fn new(signals: &[Signal], _flag: Flag) -> io::Result<Self> {
    let sb = SignalBool(Arc::new(AtomicBool::new(false)));
    if signals != [Signal::SIGINT] {
      return Err(io::Error::new(
          io::ErrorKind::InvalidInput, "invalid signals"));
    }

    unsafe {
      if SetConsoleCtrlHandler(Some(os_handler), TRUE) == FALSE {
        return Err(io::Error::last_os_error());
      }
      SIGNAL = transmute(sb.clone());
    }

    Ok(sb)
  }
}
