#![no_main]
#![no_std]

use cortex_m_rt::entry;
use msp432p401r::Peripherals;
use panic_halt as _;

fn stop_watchdog_timer(peripherals: &Peripherals) {
    peripherals.WDT_A.wdtctl.modify(|r, w| unsafe {
        let watchdog_password: u16 = 0x5A00;
        let hold: u16 = (r.bits() | 0x0080) & 0x00FF; // Set bit 7 to one, everything else stays the same
        w.bits(watchdog_password + hold)
    });
}

fn set_p1_0_output_dir(peripherals: &Peripherals) {
    peripherals.DIO.padir.modify(|r, w| unsafe {
        w.p1dir().bits(r.p1dir().bits() | 0x01)
    });
}

fn toggle_p1_0_output(peripherals: &Peripherals) {
    peripherals.DIO.paout.modify(|r, w| unsafe {
        w.p1out().bits(r.p1out().bits() ^ 0x01)
    });
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    stop_watchdog_timer(&peripherals);
    set_p1_0_output_dir(&peripherals);

    loop {
        toggle_p1_0_output(&peripherals);

        let mut delay = 100000;
        while delay > 0 {
            delay = delay - 1;
        }
    }
}
