use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::os::raw::c_int;

use nix::sys::signal::*;
pub use nix::sys::signal::{Signal, sigaction};

use crate::Flag;
use crate::SignalBool;

static SIGNALS: AtomicUsize = AtomicUsize::new(0);

extern "C" fn os_handler(sig: c_int) {
  SIGNALS.fetch_or(1 << sig, Ordering::Relaxed);
}

impl SignalBool {
  /// Register an array of signals to set the internal flag to true when received. A signal
  /// registered with multiple `SignalBool`s will interfere with each other.
  pub fn new(signals: &[Signal], flag: Flag) -> io::Result<Self> {
    let flags = match flag {
      Flag::Restart => SaFlags::SA_RESTART,
      Flag::Interrupt => SaFlags::empty(),
    };
    let handler = SigHandler::Handler(os_handler);
    let sa = SigAction::new(handler, flags, SigSet::empty());
    let mut mask = 0;

    for signal in signals {
      if *signal as u32 >= usize::BITS {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, "unsupported large signal"));
      }
      unsafe {
        if let Err(errno) = sigaction(*signal, &sa) {
          return Err(io::Error::from_raw_os_error(errno as i32));
        }
      }
      mask |= 1 << *signal as usize;
    }

    Ok(SignalBool(mask))
  }

  /// Reset the internal flag to false.
  pub fn reset(&mut self) {
    SIGNALS.fetch_and(!self.0, Ordering::Relaxed);
  }

  /// Check whether we've caught a registered signal.
  pub fn caught(&self) -> bool {
    SIGNALS.load(Ordering::Relaxed) & self.0 != 0
  }
}
