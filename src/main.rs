#![no_main]
#![no_std]

extern crate msp432p401r_hal as hal;

use cortex_m_rt::entry;
use hal::gpio::{Output, ToggleableOutputPin, PrimaryModuleFunction, GPIO};
use hal::gpio;
use hal::gpio::porta::P1_0;
use hal::watchdog::{Disable, Enabled, WatchdogTimer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let watchdog: WatchdogTimer<Enabled> = WatchdogTimer::<Enabled>::new();
    watchdog.try_disable().unwrap();

    let mut p1_0: P1_0<GPIO<Output>> = gpio::porta::P1_0::<GPIO<Output>>::into_output();

    loop {
        p1_0.try_toggle().unwrap();
        let mut delay = 100000;
        while delay > 0 {
            delay = delay - 1;
        }
    }
}
