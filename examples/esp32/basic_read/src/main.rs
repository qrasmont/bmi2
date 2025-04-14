//! This example runs on the ESP32-S3-DevKitC-1 board, wired to a BMI270 via SPI
//! It demonstrates simple IMU data read functionality.
#![no_std]
#![no_main]

use bmi2::config;
use bmi2::types::Burst;
use bmi2::types::PwrCtrl;
use bmi2::Bmi2;

use embassy_executor::Spawner;
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::{master::Config, Mode};
use esp_hal::time::Rate;
use esp_hal::{clock::CpuClock, timer::timg::TimerGroup};
use esp_println as _;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // Set up ESP32
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    let timer_group = TimerGroup::new(peripherals.TIMG0);

    esp_hal_embassy::init(timer_group.timer1);

    // Initialize SPI
    let cs = Output::new(peripherals.GPIO3, Level::High, OutputConfig::default());
    let sclk = peripherals.GPIO8;
    let mosi = peripherals.GPIO2;
    let miso = peripherals.GPIO1;

    let spi_bus = esp_hal::spi::master::Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(100))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sclk)
    .with_mosi(mosi)
    .with_miso(miso);

    // Create the SPI device with CS pin management
    let spi_device = ExclusiveDevice::new_no_delay(spi_bus, cs).unwrap();

    const BUFFER_SIZE: usize = 256;

    let mut bmi = Bmi2::<_, _, BUFFER_SIZE>::new_spi(spi_device, Delay, Burst::new(255));

    let chip_id = match bmi.get_chip_id() {
        Ok(0) => defmt::panic!("Chip ID is 0, which is invalid"),
        Ok(id) => id,
        Err(_) => defmt::panic!("Failed to get chip ID"),
    };

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

    let mut delay = esp_hal::delay::Delay::new();

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
        delay.delay_ms(500);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::info!("Panic: {}", info);

    loop {}
}
