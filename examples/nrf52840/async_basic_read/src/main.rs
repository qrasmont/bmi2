#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::twim::{self, Twim};
use embassy_nrf::{bind_interrupts, peripherals};
use embassy_time::{Delay, Timer};
use static_cell::ConstStaticCell;
use {defmt_rtt as _, panic_probe as _};

use bmi2::config;
use bmi2::Builder;
use bmi2::{types::Burst, types::PwrCtrl, I2cAddr};

bind_interrupts!(struct Irqs {
    TWISPI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    info!("Initializing TWI...");
    let mut config = twim::Config::default();
    config.frequency = twim::Frequency::K100;
    config.sda_pullup = true;
    config.scl_pullup = true;

    static RAM_BUFFER: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
    let twi = Twim::new(p.TWISPI0, Irqs, p.P0_26, p.P0_27, config, RAM_BUFFER.take());
    let delay = Delay;

    let mut config_buf = [0u8; 256];
    let mut bmi = Builder::i2c(twi, delay, I2cAddr::Alternative, Burst::new(255))
        .config(&config::BMI270_CONFIG_FILE)
        .pwr_ctrl(PwrCtrl {
            aux_en: false,
            gyr_en: true,
            acc_en: true,
            temp_en: false,
        })
        .init(&mut config_buf)
        .await
        .unwrap();

    loop {
        let data = bmi.get_data().await.unwrap();
        defmt::info!(
            "data: acc_x:{}, acc_y:{}, acc_z:{}, gyr_x:{}, gyr_y:{}, gyr_z:{}",
            data.acc.x,
            data.acc.y,
            data.acc.z,
            data.gyr.x,
            data.gyr.y,
            data.gyr.z
        );

        Timer::after_millis(500).await;
    }
}
