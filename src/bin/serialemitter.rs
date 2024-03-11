use serialport::{SerialPortSettings, open_with_settings};
use std::io::{self, Write};
use std::time::Duration;

fn main() {
    // Replace "COM1" with the actual serial port name on your system.
    let port_name = "COM1";

    let settings = SerialPortSettings {
        baud_rate: 9600,
        data_bits: serialport::DataBits::Eight,
        flow_control: serialport::FlowControl::None,
        parity: serialport::Parity::None,
        stop_bits: serialport::StopBits::One,
        timeout: Duration::from_millis(100),
    };

    match open_with_settings(port_name, &settings) {
        Ok(mut port) => {
            println!("Serial port opened successfully. Writing serial numbers...");

            // Replace this loop with your actual serial number generation logic.
            for serial_number in 1..=10 {
                let data = format!("{}\n", serial_number);
                if let Err(e) = port.write_all(data.as_bytes()) {
                    eprintln!("Error writing to serial port: {}", e);
                    break;
                }
                // Sleep for a short time before sending the next serial number.
                std::thread::sleep(Duration::from_millis(100));
            }

            println!("Serial numbers sent successfully.");
        }
        Err(e) => eprintln!("Error opening serial port: {}", e),
    }
}
