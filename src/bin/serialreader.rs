use idtap::{lookup_name_by_serial, IDTapError, Result};
use rusqlite::Connection;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::borrow::Cow;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Duration;

pub struct Reader {
    conn: Connection,
    port: BufReader<Box<dyn SerialPort>>,
}

impl Reader {
    pub fn new<'a>(
        database_path: impl AsRef<Path>,
        port_name: impl Into<Cow<'a, str>>,
    ) -> Result<Self> {
        Ok(Self {
            conn: Connection::open(database_path)?,
            port: BufReader::new(
                serialport::new(port_name, 9600)
                    .data_bits(DataBits::Eight)
                    .flow_control(FlowControl::None)
                    .parity(Parity::None)
                    .stop_bits(StopBits::One)
                    .timeout(Duration::from_millis(100))
                    .open()?,
            ),
        })
    }

    pub fn read(&mut self) -> Result<Option<String>> {
        let mut str = String::new();
        self.port.read_line(&mut str)?;
        let serial = str.parse()?;
        lookup_name_by_serial(serial, &self.conn).map_err(IDTapError::from)
    }
}

fn main() -> Result<()> {
    // Replace "COM1" with the actual serial port name on your system.
    let mut reader = Reader::new("your_database.sqlite", "COM1")?;

    // just keep reading numbers forever
    loop {
        match reader.read()? {
            Some(name) => {
                println!("Name for serial number: {}", name)
            }
            None => println!("No name found for serial number :("),
        }
    }
}
