A simple crate to catch signals and set a boolean flag for later use.

This crate doesn't create threads behind the scene.

# Example

Here is a program that sleeps until it receives three `SIGINT` signals.

```
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
