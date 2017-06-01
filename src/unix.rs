extern crate nix;

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::os::raw::c_int;
use std::mem::transmute;

use self::nix::sys::signal::*;
pub use self::nix::sys::signal::Signal;

use ::Flag;
use ::SignalBool;

const SIGNUM: usize = 32;

static mut SIGNALS: [usize; SIGNUM] = [0; SIGNUM];

extern "C" fn os_handler(sig: c_int) {
  let sb: Arc<AtomicBool> = unsafe {
     transmute(SIGNALS[sig as usize])
  };
  sb.store(true, Ordering::Relaxed);
}

impl SignalBool {
  /// Register an array of signals to set the internal flag to true when received.
  pub fn new(signals: &[Signal], flag: Flag) -> io::Result<Self> {
    let flags = match flag {
      Flag::Restart => SA_RESTART,
      Flag::Interrupt => SaFlags::empty(),
    };
    let handler = SigHandler::Handler(os_handler);
    let sa = SigAction::new(handler, flags, SigSet::empty());
    let sb = SignalBool(Arc::new(AtomicBool::new(false)));

    for signal in signals {
      unsafe {
        if let Err(e) = sigaction(*signal, &sa) {
          return Err(io::Error::from_raw_os_error(e.errno() as i32));
        }
        SIGNALS[*signal as usize] = transmute(sb.clone());
      }
    }

    Ok(sb)
  }
}
