#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use msp432p401r as pac;
use panic_halt as _;
use msp432p401r::{WDT_A, DIO, Peripherals};

fn stop_watchdog_timer(peripherals: &Peripherals) {
    hprintln!("Stopping watchdog timer").unwrap();
    peripherals.WDT_A.wdtctl.modify(|r, w| unsafe {
        let hold: u16 = (r.bits() | 0x0080) & 0x00FF;
        w.bits(0x5A00 + hold)
    });
}

fn set_output_dir(peripherals: &Peripherals) {
    hprintln!("Setting output dir").unwrap();
    peripherals.DIO.padir.modify(|r, w| unsafe {
        w.p1dir().bits(r.p1dir().bits() | 0x01)
    });
}

#[entry]
fn main() -> ! {
    hprintln!("Program start").unwrap();

    let peripherals = pac::Peripherals::take().unwrap();

    stop_watchdog_timer(&peripherals);
    set_output_dir(&peripherals);

    loop {
        hprintln!("Loop start").unwrap();

        peripherals.DIO.paout.modify(|r, w| unsafe { w.p1out().bits(r.p1out().bits() | 0x01) });
        let mut delay = 50000;
        while delay > 0 {
            delay = delay - 1;
        }
        hprintln!("Loop end").unwrap();
        peripherals.DIO.paout.modify(|r, w| unsafe { w.p1out().bits(r.p1out().bits() & 0x00) });
    }
}
