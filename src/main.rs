#![no_std]
#![no_main]

use cortex_m::asm::nop;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f3::stm32f303;
use cortex_m::delay;

#[entry]
fn main() -> ! {
    let perip =stm32f303::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut delay = delay::Delay::new(cp.SYST, 8_000_000);  //HSI 8MHz

    perip.RCC.ahbenr.modify(|_, w| w.iopben().set_bit());   //GPIOB clock enable

    let gpiob= &perip.GPIOB;
    gpiob.moder.modify(|_, w| w.moder3().output()); //PB3 as output
    
    loop {
        gpiob.bsrr.write(|w| w.bs3().set()); // Set PB3 high
        delay.delay_ms(1000);
        gpiob.bsrr.write(|w| w.br3().reset()); // Set PB3 low
        delay.delay_ms(1000);
    }
}
