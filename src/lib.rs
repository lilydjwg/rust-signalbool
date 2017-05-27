//! A simple crate to catch signals and set a boolean flag for later use.
//!
//! This crate doesn't create threads behind the scene.
//!
//! [![Crates.io Version](https://img.shields.io/crates/v/signalbool.svg)](https://crates.io/crates/signalbool)
//! [![GitHub stars](https://img.shields.io/github/stars/lilydjwg/signalbool.svg?style=social&label=Star)](https://github.com/lilydjwg/signalbool)
//!
//! # Example
//!
//! Here is a program that sleeps until it receives three `SIGINT` signals.
//!
//! ```
//! extern crate signalbool;
//! extern crate nix;
//! 
//! use nix::unistd::sleep;
//! 
//! fn main() {
//!   let mut sb = signalbool::SignalBool::new(
//!     &[signalbool::Signal::SIGINT], signalbool::Flag::Interrupt,
//!   ).unwrap();
//!   let mut count = 0;
//!     
//!   loop {
//!     sleep(10);
//!     if sb.caught() {
//!       println!("Caught SIGINT.");
//!       count += 1;
//!       sb.reset();
//!       if count == 3 {
//!         break;
//!       }
//!     }
//!   }
//! }
//! ```

extern crate nix;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::os::raw::c_int;
use std::mem::transmute;

use nix::Result;
use nix::sys::signal::*;

pub use nix::sys::signal::Signal;

/// A struct that catches specified signals and sets its internal flag to `true`.
///
/// Note: any previously-registered signal handlers will be lost.
#[derive(Clone)]
pub struct SignalBool(Arc<AtomicBool>);

/// flag controlling the restarting behavior.
///
/// Note that most functions in `std` ignore `EINTR` and continue their operations.
///
/// See manpage [`signal(7)`](http://man7.org/linux/man-pages/man7/signal.7.html) for details.
pub enum Flag {
  /// Blocking syscalls will be interrupted.
  Interrupt,
  /// Use SA_RESTART so that syscalls don't get interrupted.
  Restart,
}

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
  pub fn new(signals: &[Signal], flag: Flag) -> Result<Self> {
    let flags = match flag {
      Flag::Restart => SA_RESTART,
      Flag::Interrupt => SaFlags::empty(),
    };
    let handler = SigHandler::Handler(os_handler);
    let sa = SigAction::new(handler, flags, SigSet::empty());
    let sb = SignalBool(Arc::new(AtomicBool::new(false)));

    for signal in signals {
      unsafe {
        sigaction(*signal, &sa)?;
        SIGNALS[*signal as usize] = transmute(sb.clone());
      }
    }

    Ok(sb)
  }

  /// Reset the internal flag to false.
  pub fn reset(&mut self) {
    self.0.store(false, Ordering::Relaxed);
  }

  /// Check whether we've caught a registered signal.
  pub fn caught(&self) -> bool {
    self.0.load(Ordering::Relaxed)
  }
}
