//! 6-2 LEDとボタン/GPIOのサンプルコードです。
//! ボタン1 (一番右のボタン) を押している間、ユーザーLEDが点灯します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-2-led_and_button
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::pac::Peripherals;
use wio::prelude::*; // 主要な構造体やトレイトをインポートする

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut pins = wio::Pins::new(peripherals.PORT);

    // set user led to output
    let mut led = pins.user_led.into_push_pull_output(&mut pins.port);

    // set button 1 to input
    let button1 = pins.button1.into_floating_input(&mut pins.port);

    loop {
        if button1.is_low().unwrap() {
            // turn on led if button is pressed
            led.set_high().unwrap();
        } else {
            // turn off led if button is not pressed
            led.set_low().unwrap();
        }
    }
}
