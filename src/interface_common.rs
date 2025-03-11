use embedded_hal::i2c::SevenBitAddress;

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
