use printf_rs::*;
use rand::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        let x: u32 = rand::thread_rng().gen_range(0x1F600, 0x1F64F);
        let emoji = std::char::from_u32(x).unwrap();
        printf!("\r %lc  ", emoji);
        thread::sleep(Duration::from_secs(1));
    }
}
