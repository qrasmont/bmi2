pub use crate::interface_common::{I2cAddr, I2cInterface, SpiInterface};

use crate::types::Error;
use embedded_hal::i2c::I2c;
use embedded_hal::spi::SpiDevice;

pub trait WriteData {
    type Error;
    fn write(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
    fn write_reg(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
}

pub trait ReadData {
    type Error;
    fn read(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
    fn read_reg(&mut self, register: u8) -> Result<u8, Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: I2c<Error = E>,
{
    type Error = Error<I2C::Error>;
    fn write(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write(self.address, payload).map_err(Error::Comm)
    }

    fn write_reg(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.i2c.write(self.address, &payload).map_err(Error::Comm)
    }
}

impl<SPI, CommE> WriteData for SpiInterface<SPI>
where
    SPI: SpiDevice<Error = CommE>,
{
    type Error = Error<CommE>;
    fn write(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        payload[0] += 0x80;

        // `write` asserts and deasserts CS for us. No need to do it manually!

        self.spi.write(payload).map_err(Error::Comm)
    }

    fn write_reg(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register + 0x80, data];

        // `write` asserts and deasserts CS for us. No need to do it manually!

        self.spi.write(&payload).map_err(Error::Comm)
    }
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: I2c<Error = E>,
{
    type Error = Error<I2C::Error>;
    fn read(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c
            .write_read(self.address, &[payload[0]], &mut payload[1..])
            .map_err(Error::Comm)
    }

    fn read_reg(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }
}

impl<SPI, CommE> ReadData for SpiInterface<SPI>
where
    SPI: SpiDevice<Error = CommE>,
{
    type Error = Error<CommE>;
    fn read(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        // `read` asserts and deasserts CS for us. No need to do it manually!
        let res = self.spi.read(payload).map_err(Error::Comm);

        res?;
        Ok(())
    }

    fn read_reg(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut payload = [register, 0];

        // `read` asserts and deasserts CS for us. No need to do it manually!
        let res = self.spi.read(&mut payload).map_err(Error::Comm);

        match res {
            Ok(_) => Ok(payload[1]),
            Err(e) => Err(e),
        }
    }
}
