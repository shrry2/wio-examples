//! 6-2 LEDとボタン/GPIOのサンプルコードです。
//! ボタン1 (一番右のボタン) を押している間、ユーザーLEDが点灯します。
//! LEDドライバとボタンドライバを導入したバージョンです。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-2-led_and_button_driver
//! ```

#![no_std]
#![no_main]
#![allow(dead_code)] // 使用しないメソッドでコンパイラが警告を出さないようにする

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::gpio::*; // GPIOの構造体やトレイトをインポートする
use wio::pac::Peripherals;
use wio::prelude::*; // 主要な構造体やトレイトをインポートする

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut pins = wio::Pins::new(peripherals.PORT);

    // initialize led driver object
    let mut led = Led::new(pins.user_led, &mut pins.port);

    // initialize button1 driver object
    let button1 = Button1::new(pins.button1, &mut pins.port);

    loop {
        if button1.is_pressed() {
            led.turn_on();
        } else {
            led.turn_off();
        }
    }
}

// Wio Terminalのボタン1ドライバ
struct Button1 {
    pin: Pc26<Input<Floating>>,
}

impl Button1 {
    // set pc26 as input
    fn new(pin: Pc26<Input<Floating>>, port: &mut Port) -> Self {
        Self {
            pin: pin.into_floating_input(port),
        }
    }

    // return true if button is pressed
    fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    // return true if button is not pressed
    fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}

// Wio TerminalのユーザーLEDドライバ
struct Led {
    pin: Pa15<Output<PushPull>>,
}

impl Led {
    // set pa15 as output
    fn new(pin: Pa15<Input<Floating>>, port: &mut Port) -> Self {
        Self {
            pin: pin.into_push_pull_output(port),
        }
    }

    // turn on led
    fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    // turn off led
    fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }

    // toggle led
    fn toggle(&mut self) {
        self.pin.toggle();
    }
}
