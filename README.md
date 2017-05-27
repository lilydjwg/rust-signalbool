A simple crate to catch signals and set a boolean flag for later use.

This crate doesn't create threads behind the scene.

[![Crates.io Version](https://img.shields.io/crates/v/signalbool.svg)](https://crates.io/crates/signalbool)
[![GitHub stars](https://img.shields.io/github/stars/lilydjwg/rust-signalbool.svg?style=social&label=Star)](https://github.com/lilydjwg/rust-signalbool)

# Example

Here is a program that sleeps until it receives three `SIGINT` signals.

```rust
extern crate signalbool;
extern crate nix;

use nix::unistd::sleep;

fn main() {
  let mut sb = signalbool::SignalBool::new(
    &[signalbool::Signal::SIGINT], signalbool::Flag::Interrupt,
  ).unwrap();
  let mut count = 0;
    
  loop {
    sleep(10);
    if sb.caught() {
      println!("Caught SIGINT.");
      count += 1;
      sb.reset();
      if count == 3 {
        break;
      }
    }
  }
}
```
