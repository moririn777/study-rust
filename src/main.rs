#![no_std]
#![no_main]

//use cortex_m::asm::nop;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f3xx_hal :: {pac, prelude::*};

#[entry]
fn main() -> ! {
    let perip =pac::Peripherals::take().unwrap();

    let mut rcc =perip.RCC.constrain();
    let mut gpiob =perip.GPIOB.split(&mut rcc.ahb);
    
    let mut led =gpiob
        .pb3 
        .into_push_pull_output(&mut gpiob.moder,&mut gpiob.otyper);

    loop {
        led.toggle().unwrap();
        asm::delay(1_000_000);
    }
}
