#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;

use nrf52840_hal::{
    gpio::*,
    pac,
    timer::Timer,
    twim::{self, Twim},
};

use defmt_rtt as _;

use bmi2::config;
use bmi2::Bmi2;
use bmi2::{types::Burst, types::PwrCtrl, I2cAddr};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut timer = Timer::new(p.TIMER0);
    let port0 = p0::Parts::new(p.P0);

    let scl = port0.p0_27.into_floating_input().degrade();
    let sda = port0.p0_26.into_floating_input().degrade();

    let twim_pins = twim::Pins { scl, sda };

    let i2c = Twim::new(p.TWIM0, twim_pins, twim::Frequency::K100);

    let mut bmi = Bmi2::new_i2c(i2c, I2cAddr::Alternative, Burst::Other(255));
    let chip_id = bmi.get_chip_id().unwrap();
    defmt::info!("chip id: {}", chip_id);

    bmi.init(&config::BMI270_CONFIG_FILE).unwrap();

    // Enable power for the accelerometer and the gyroscope.
    let pwr_ctrl = PwrCtrl {
        aux_en: false,
        gyr_en: true,
        acc_en: true,
        temp_en: false,
    };
    bmi.set_pwr_ctrl(pwr_ctrl).unwrap();

    loop {
        let data = bmi.get_data().unwrap();
        defmt::info!(
            "data: acc_x:{}, acc_y:{}, acc_z:{}, gyr_x:{}, gyr_y:{}, gyr_z:{}",
            data.acc.x,
            data.acc.y,
            data.acc.z,
            data.gyr.x,
            data.gyr.y,
            data.gyr.z
        );
        timer.delay_ms(500_u16);
    }
}
