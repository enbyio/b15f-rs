use std::thread::sleep;
use std::time::Duration;
use b15f::b15f::{Board, B15F};
use b15f::atmega1284p;
use b15f::atmega1284p::DataRegister;
use b15f::error::Error;

fn main() -> Result<(), Error> {
    let mut board = b15f::dummy::Dummy::new();

    board.set_register(DataRegister::DDRA, 0b1111_1111)?;
    board.set_register(DataRegister::PortA, 42)?;
    board.set_register(DataRegister::DDRA, 0)?;
    println!("{}", board.get_register(DataRegister::PinA)?);
    Ok(())
    /*let mut drv = B15F::new()?;
    let mut inputs = drv.get_register(atmega1284p::DataRegister::DDRB)?;
    inputs |= 1<<0;
    drv.set_register(atmega1284p::DataRegister::DDRB, inputs)?;

    let mut tmp = drv.get_register(atmega1284p::DataRegister::PortB)?;
    drv.set_register(atmega1284p::DataRegister::PortB, tmp|(1<<0))?;

    sleep(Duration::from_millis(300));

    tmp = drv.get_register(atmega1284p::DataRegister::PortB)?;
    tmp &= !1;
    Ok(drv.set_register(atmega1284p::DataRegister::PortB, tmp&!1)?)*/
}