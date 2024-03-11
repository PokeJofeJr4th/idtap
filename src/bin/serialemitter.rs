use std::io;
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    // Simulate generating serial numbers and outputting them over a serial interface.
    let mut rng = rand::thread_rng();

    loop {
        let serial_number: i32 = rng.gen_range(100000, 999999);
        println!("Generated Serial Number: {}", serial_number);

        // Simulate delay between serial number transmissions.
        thread::sleep(Duration::from_secs(2));
    }
}
