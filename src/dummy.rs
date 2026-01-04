use std::io::{Read, Write};
use std::time::Duration;
use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPort, StopBits};
use log::{error, warn};
use crate::atmega1284p::DataRegister;
use crate::b15f::Board;
use crate::error::Error;

pub struct Dummy {
    registers: [u8; 4],
    io_prop: [u8; 4],
    log: bool,
}

impl Dummy {
    pub fn new(logging: bool) -> Dummy {
        Self {
            registers: [0u8; 4],
            io_prop: [0u8; 4],
            log: logging,
        }
    }
}

impl Board for Dummy {
    fn get_register(&mut self, register: DataRegister) -> Result<u8, Error> {
        _ = DataRegister::try_from(register).or(Err(Error::new("invalid register")))?;
        let reg = (register as u8 - 0x20)/3;
        let r = match register as u8 % 3 {
            2 => Ok(self.registers[reg as usize] & (!self.io_prop[reg as usize])),
            _ => Err(Error::new(format!("invalid get register: {:?}", register).as_str())),
        };
        if self.log {
            println!("Getting Register: {:?}", register);
            println!("{:?}", r);
        }
        r
    }

    fn set_register(&mut self, register: DataRegister, value: u8) -> Result<(), Error> {
        _ = DataRegister::try_from(register).or(Err(Error::new("invalid register")))?;
        let reg = (register as u8 - 0x20)/3;
        if self.log {
            println!("Setting Register: {:?}, as {:08b}", register, reg);
        }
        match register as u8 % 3 {
            0 => self.io_prop[reg as usize] = value,
            1 => self.registers[reg as usize] = value & self.io_prop[reg as usize],
            _ => return Err(Error::new(format!("cannot set register: {:?}", register).as_str())),
        };
        Ok(())
    }
}

impl Read for Dummy {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

impl Write for Dummy {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl SerialPort for Dummy {
    fn name(&self) -> Option<String> {
        todo!()
    }

    fn baud_rate(&self) -> serialport::Result<u32> {
        todo!()
    }

    fn data_bits(&self) -> serialport::Result<DataBits> {
        todo!()
    }

    fn flow_control(&self) -> serialport::Result<FlowControl> {
        todo!()
    }

    fn parity(&self) -> serialport::Result<Parity> {
        todo!()
    }

    fn stop_bits(&self) -> serialport::Result<StopBits> {
        todo!()
    }

    fn timeout(&self) -> Duration {
        todo!()
    }

    fn set_baud_rate(&mut self, baud_rate: u32) -> serialport::Result<()> {
        todo!()
    }

    fn set_data_bits(&mut self, data_bits: DataBits) -> serialport::Result<()> {
        todo!()
    }

    fn set_flow_control(&mut self, flow_control: FlowControl) -> serialport::Result<()> {
        todo!()
    }

    fn set_parity(&mut self, parity: Parity) -> serialport::Result<()> {
        todo!()
    }

    fn set_stop_bits(&mut self, stop_bits: StopBits) -> serialport::Result<()> {
        todo!()
    }

    fn set_timeout(&mut self, timeout: Duration) -> serialport::Result<()> {
        todo!()
    }

    fn write_request_to_send(&mut self, level: bool) -> serialport::Result<()> {
        todo!()
    }

    fn write_data_terminal_ready(&mut self, level: bool) -> serialport::Result<()> {
        todo!()
    }

    fn read_clear_to_send(&mut self) -> serialport::Result<bool> {
        todo!()
    }

    fn read_data_set_ready(&mut self) -> serialport::Result<bool> {
        todo!()
    }

    fn read_ring_indicator(&mut self) -> serialport::Result<bool> {
        todo!()
    }

    fn read_carrier_detect(&mut self) -> serialport::Result<bool> {
        Err(serialport::Error::new(serialport::ErrorKind::Unknown, "not implemented yet"))
    }

    fn bytes_to_read(&self) -> serialport::Result<u32> {
        Err(serialport::Error::new(serialport::ErrorKind::Unknown, "not implemented yet"))
    }

    fn bytes_to_write(&self) -> serialport::Result<u32> {
        Err(serialport::Error::new(serialport::ErrorKind::Unknown, "not implemented yet"))
    }

    fn clear(&self, buffer_to_clear: ClearBuffer) -> serialport::Result<()> {
        error!("clear should not be called on dummy");
        Err(serialport::Error::new(serialport::ErrorKind::Unknown, "not implemented yet"))
    }

    fn try_clone(&self) -> serialport::Result<Box<dyn SerialPort>> {
        Err(serialport::Error::new(serialport::ErrorKind::Unknown, "This dummy is not supposed to be cloned"))
    }

    fn set_break(&self) -> serialport::Result<()> {
        Err(serialport::Error::new(serialport::ErrorKind::Unknown, "why are you doing this?"))
    }

    fn clear_break(&self) -> serialport::Result<()> {
        Err(serialport::Error::new(serialport::ErrorKind::Unknown, "this is not supposed to be called on a dummy"))
    }
}