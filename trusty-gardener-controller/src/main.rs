#![no_main]
#![no_std]

use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal as hal;
use crate::hal::prelude::*;


#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Hello world!");

    loop {}
}
