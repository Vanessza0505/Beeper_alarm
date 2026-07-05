#![no_std]


use embassy_time::{Duration, Timer};

pub struct Beeper {
    pin: esp_hal::gpio::Output<'static>,
    button: bool,
    boat_lost: bool,
    battery: u8,
}

impl Beeper {
    pub async fn new(
        pin: esp_hal::gpio::Output<'static>,
        button: bool,
        boat_lost: bool,
        battery: u8
    ) -> Self {
        Self {
            pin,
            button,
            boat_lost,
            battery
        }
    }

    pub fn is_lost(&mut self) {
        self.pin.set_high();
    }

    pub async fn battery_alarm(&mut self) {
        self.pin.set_high();
        Timer::after(Duration::from_secs(1)).await;
        self.pin.set_low();
        Timer::after(Duration::from_secs(4)).await;
    }

    pub async fn main_function(&mut self){
        if self.button == true{
            if self.boat_lost == true{
                self.is_lost();
            }
            if self.battery <= 10{
                self.battery_alarm().await;
            }
        }
    }
}
