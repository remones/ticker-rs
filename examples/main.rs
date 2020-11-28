extern crate ticker;

use std::thread;
use std::time::Duration;

fn main() {
    let d = Duration::from_millis(1000);
    let mut t: ticker::Ticker = ticker::Ticker::new(d);
    let mut c: u8 = 0;
    for _ in &t {
        c += 1;
        if c == 3 {
            break;
        }
        println!("ticker!");
    }

    let d = Duration::from_millis(100);
    t.reset(d);

    c = 0;
    for _ in &t {
        c += 1;
        if c == 3 {
            break;
        }
        println!("ticker!");
    }

    t.stop();
    thread::sleep(Duration::new(1, 0));
    println!("ticker has been dropped");
}
