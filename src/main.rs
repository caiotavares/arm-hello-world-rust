#![no_main]
#![no_std]

use cortex_m_rt::entry;
use msp432p401r::Peripherals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    lib::stop_watchdog_timer(&peripherals);
    lib::set_p1_0_output_dir(&peripherals);

    loop {
        lib::toggle_p1_0_output(&peripherals);

        let mut delay = 100000;
        while delay > 0 {
            delay = delay - 1;
        }
    }
}
