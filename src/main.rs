use std::{sync::Arc, thread};
use std::time::Duration;
use esp_idf_sys as _;
use esp_idf_hal::ledc::*;
use esp_idf_hal::prelude::*;

use crate::rgb::RainbowRGB;
mod rgb;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let config = config::TimerConfig::new().frequency(255.kHz().into());
    let timer = Arc::new(LedcTimerDriver::new(peripherals.ledc.timer0, &config)?);

    let mut red_pwm = LedcDriver::new(peripherals.ledc.channel0, timer.clone(), pins.gpio15)?;
    let mut green_pwm = LedcDriver::new(peripherals.ledc.channel1, timer.clone(), pins.gpio2)?;
    let mut blue_pwm = LedcDriver::new(peripherals.ledc.channel2, timer.clone(), pins.gpio4)?;

    let mut rgb = RainbowRGB::new();

    loop {
        rgb.next_color();

        red_pwm.set_duty(rgb.get_r())?;
        green_pwm.set_duty(rgb.get_g())?;
        blue_pwm.set_duty(rgb.get_b())?;

        thread::sleep(Duration::from_millis(10));
    }
}

