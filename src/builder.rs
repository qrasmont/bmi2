use crate::bmi2::Bmi2;
use crate::interface::{I2cAddr, I2cInterface, SpiInterface};
use crate::interface::{ReadData, WriteData};
use crate::types::{AccConf, AccRange, Burst, Error, GyrConf, GyrRange, PwrCtrl};

#[cfg(feature = "blocking")]
use embedded_hal::delay::DelayNs;
#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs;

pub struct Builder<'a, I, D> {
    iface: I,
    delay: D,
    max_burst: u16,
    config_file: Option<&'a [u8]>,
    pwr_ctrl: Option<PwrCtrl>,
    acc_conf: Option<AccConf>,
    acc_range: Option<AccRange>,
    gyr_conf: Option<GyrConf>,
    gyr_range: Option<GyrRange>,
}

impl<'a, I2C, D> Builder<'a, I2cInterface<I2C>, D> {
    pub fn i2c(i2c: I2C, delay: D, address: I2cAddr, burst: Burst) -> Self {
        Builder {
            iface: I2cInterface {
                i2c,
                address: address.addr(),
            },
            delay,
            max_burst: burst.val(),
            config_file: None,
            pwr_ctrl: None,
            acc_conf: None,
            acc_range: None,
            gyr_conf: None,
            gyr_range: None,
        }
    }
}

impl<'a, SPI, D> Builder<'a, SpiInterface<SPI>, D> {
    pub fn spi(spi: SPI, delay: D, burst: Burst) -> Self {
        Builder {
            iface: SpiInterface { spi },
            delay,
            max_burst: burst.val(),
            config_file: None,
            pwr_ctrl: None,
            acc_conf: None,
            acc_range: None,
            gyr_conf: None,
            gyr_range: None,
        }
    }
}

impl<'a, I, D> Builder<'a, I, D> {
    pub fn config(mut self, config: &'a [u8]) -> Self {
        self.config_file = Some(config);
        self
    }

    pub fn pwr_ctrl(mut self, pwr_ctrl: PwrCtrl) -> Self {
        self.pwr_ctrl = Some(pwr_ctrl);
        self
    }

    pub fn acc_conf(mut self, acc_conf: AccConf) -> Self {
        self.acc_conf = Some(acc_conf);
        self
    }

    pub fn acc_range(mut self, acc_range: AccRange) -> Self {
        self.acc_range = Some(acc_range);
        self
    }

    pub fn gyr_conf(mut self, gyr_conf: GyrConf) -> Self {
        self.gyr_conf = Some(gyr_conf);
        self
    }

    pub fn gyr_range(mut self, gyr_range: GyrRange) -> Self {
        self.gyr_range = Some(gyr_range);
        self
    }
}

#[maybe_async::maybe_async(AFIT)]
impl<'a, I, D, CommE> Builder<'a, I, D>
where
    I: ReadData<Error = Error<CommE>> + WriteData<Error = Error<CommE>>,
    D: DelayNs,
{
    pub async fn init(self, buf: &mut [u8]) -> Result<Bmi2<I, D>, Error<CommE>> {
        let config_file = self.config_file.ok_or(Error::MissingConfig)?;

        let mut bmi = Bmi2::from_parts(self.iface, self.delay, self.max_burst);

        bmi.init(config_file, buf).await?;

        if let Some(acc_conf) = self.acc_conf {
            bmi.set_acc_conf(acc_conf).await?
        }

        if let Some(acc_range) = self.acc_range {
            bmi.set_acc_range(acc_range).await?
        }

        if let Some(gyr_conf) = self.gyr_conf {
            bmi.set_gyr_conf(gyr_conf).await?
        }

        if let Some(gyr_range) = self.gyr_range {
            bmi.set_gyr_range(gyr_range).await?
        }

        if let Some(pwr_ctrl) = self.pwr_ctrl {
            bmi.set_pwr_ctrl(pwr_ctrl).await?
        }

        Ok(bmi)
    }
}
