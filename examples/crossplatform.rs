extern crate signalbool;

use std::thread::sleep;
use std::time::Duration;

fn main() {
  let mut sb = signalbool::SignalBool::new(
    &[signalbool::Signal::SIGINT], signalbool::Flag::Restart,
  ).unwrap();
  let mut count = 0;
    
  loop {
    sleep(Duration::from_millis(100));
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
