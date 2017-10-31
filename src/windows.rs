extern crate winapi;
extern crate kernel32;

use std::io;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

use self::winapi::{BOOL, DWORD, TRUE, FALSE};
use self::kernel32::SetConsoleCtrlHandler;

use ::Flag;
use ::SignalBool;

#[derive(PartialEq)]
pub enum Signal {
  SIGINT,
}

static SIGNAL: AtomicBool = ATOMIC_BOOL_INIT;

extern "system" fn os_handler(_: DWORD) -> BOOL {
  SIGNAL.store(true, Ordering::Relaxed);
  TRUE
}

impl SignalBool {
  /// Register an array of signals to set the internal flag to true when received.
  pub fn new(signals: &[Signal], _flag: Flag) -> io::Result<Self> {
    if signals != [Signal::SIGINT] {
      return Err(io::Error::new(
          io::ErrorKind::InvalidInput, "invalid signals"));
    }

    unsafe {
      if SetConsoleCtrlHandler(Some(os_handler), TRUE) == FALSE {
        return Err(io::Error::last_os_error());
      }
    }

    Ok(SignalBool)
  }

  /// Reset the internal flag to false.
  pub fn reset(&mut self) {
    SIGNAL.store(false, Ordering::Relaxed);
  }

  /// Check whether we've caught a registered signal.
  pub fn caught(&self) -> bool {
    SIGNAL.load(Ordering::Relaxed)
  }
}
