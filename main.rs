#![no_std]
#![no_main]


use esp_hal::gpio::{Level, Output, OutputConfig};

use esp_hal::clock::CpuClock;
use beeper_library::Beeper;
use esp_hal::timer::timg::TimerGroup;


use embassy_executor::Spawner;

use esp_println as _;
use defmt::info;
use defmt::error;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let _ = spawner;


    let buzzer = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());
    let mut beeper = Beeper::new(buzzer, true).await;

    loop {
        beeper.is_lost();
    }
}
