#![no_std]


use embassy_time::{Duration, Timer};

pub struct Beeper {
    pin: esp_hal::gpio::Output<'static>,
    boat_lost: bool,
}

impl Beeper {
    pub async fn new(
        pin: esp_hal::gpio::Output<'static>,
        boat_lost: bool,
    ) -> Self {
        Self {
            pin,
            boat_lost,
        }
    }

    // when the fisherman reports that the boat is lost this function is used
    pub fn is_lost(&mut self) {
        if self.boat_lost == true{
            self.pin.set_high();
            Timer::after(Duration::from_secs(1)).await;
            self.pin.set_low();
            Timer::after(Duration::from_secs(1)).await;
        } 
    }


    // the battery library will call this from the beeper when its battery level is low
    pub async fn battery_alarm(&mut self) {
        self.pin.set_high();
        Timer::after(Duration::from_secs(1)).await;
        self.pin.set_low();
        Timer::after(Duration::from_secs(4)).await;
    }
}
