#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate msp432p401r;

use rt::entry;
use msp432p401r::Peripherals;
use cortex_m::asm;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let dio = peripherals.DIO;
    let timer_a0 = &peripherals.TIMER_A0;
    let wdt_a = peripherals.WDT_A;
    let wdtctl = &wdt_a.wdtctl;
    
    wdtctl.write(|w| unsafe { w.wdtpw().bits(0x5A).wdthold().wdthold_1() });

    
    dio.padir.write(|w| unsafe { w.p1dir().bits(1).p2dir().bits(0b111) });
    dio.paout.write(|w| unsafe { w.p1out().bits(1).p2out().bits(0) });

    /*dio.padir.write(|w| unsafe { w.p2dir().bits(0b111) });
    dio.pads.write_with_zero(|w| unsafe { w.p2ds().bits(1) });
    dio.paout.write(|w| unsafe { w.p2out().bits(0b111) });
    */
    loop{
        dio.paout.write(|w| unsafe { w.p2out().bits(0) });
        asm::delay(10000000);
        dio.paout.write(|w| unsafe { w.p2out().bits(0b111) });
        asm::delay(10000000);
    }
}