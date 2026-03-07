use crate::types::Error;
#[cfg(feature = "blocking")]
use embedded_hal::i2c::{I2c, SevenBitAddress};
#[cfg(feature = "async")]
use embedded_hal_async::i2c::{I2c, SevenBitAddress};

#[cfg(feature = "blocking")]
use embedded_hal::spi::SpiDevice;

#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiDevice;

/// Default I2C address of BMI270
const BMI270_I2C_ADDR: u8 = 0x68;
/// Alternative I2C address when SDO is pulled high
const BMI270_I2C_ADDR_ALT: u8 = 0x69;

pub struct I2cInterface<I2C> {
    pub i2c: I2C,
    pub address: u8,
}

pub struct SpiInterface<SPI> {
    pub spi: SPI,
}

/// I2c address.
#[derive(Debug, Default, Clone, Copy)]
pub enum I2cAddr {
    /// Use the default i2c address, 0x68.
    #[default]
    Default,
    /// Use alternative 0x69 as the i2c address (selected when SDO is pulled high).
    Alternative,
}

impl I2cAddr {
    pub fn addr(self) -> SevenBitAddress {
        match self {
            I2cAddr::Default => BMI270_I2C_ADDR,
            I2cAddr::Alternative => BMI270_I2C_ADDR_ALT,
        }
    }
}

#[allow(async_fn_in_trait)]
#[maybe_async::maybe_async(AFIT)]
pub trait WriteData {
    type Error;
    async fn write(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
    async fn write_reg(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
}

#[allow(async_fn_in_trait)]
#[maybe_async::maybe_async(AFIT)]
pub trait ReadData {
    type Error;
    async fn read(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
    async fn read_reg(&mut self, register: u8) -> Result<u8, Self::Error>;
}

#[maybe_async::maybe_async(AFIT)]
impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: I2c<Error = E>,
{
    type Error = Error<I2C::Error>;
    async fn write(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c
            .write(self.address, payload)
            .await
            .map_err(Error::Comm)
    }

    async fn write_reg(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.i2c
            .write(self.address, &payload)
            .await
            .map_err(Error::Comm)
    }
}

#[maybe_async::maybe_async(AFIT)]
impl<SPI, CommE> WriteData for SpiInterface<SPI>
where
    SPI: SpiDevice<Error = CommE>,
{
    type Error = Error<CommE>;
    async fn write(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        payload[0] += 0x80;

        // `write` asserts and deasserts CS for us. No need to do it manually!
        self.spi.write(payload).await.map_err(Error::Comm)
    }

    async fn write_reg(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register + 0x80, data];

        // `write` asserts and deasserts CS for us. No need to do it manually!
        self.spi.write(&payload).await.map_err(Error::Comm)
    }
}

#[maybe_async::maybe_async(AFIT)]
impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: I2c<Error = E>,
{
    type Error = Error<I2C::Error>;
    async fn read(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c
            .write_read(self.address, &[payload[0]], &mut payload[1..])
            .await
            .map_err(Error::Comm)
    }

    async fn read_reg(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .await
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }
}

#[maybe_async::maybe_async(AFIT)]
impl<SPI, CommE> ReadData for SpiInterface<SPI>
where
    SPI: SpiDevice<Error = CommE>,
{
    type Error = Error<CommE>;
    async fn read(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        // `read` asserts and deasserts CS for us. No need to do it manually!
        let res = self.spi.read(payload).await.map_err(Error::Comm);

        res?;
        Ok(())
    }

    async fn read_reg(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut payload = [register, 0];

        // `read` asserts and deasserts CS for us. No need to do it manually!
        let res = self.spi.read(&mut payload).await.map_err(Error::Comm);

        match res {
            Ok(_) => Ok(payload[1]),
            Err(e) => Err(e),
        }
    }
}
