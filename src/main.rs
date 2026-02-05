#![allow(unused)]
mod board;
mod gpio;
mod mcu;

use board::{BOARD, Board};
use gpio::*;

fn demo_memory_usages() {
    let s = core::mem::size_of::<DisabledInput>();
    println!("DisabledInput {}", s);
    let e = core::mem::size_of::<Gpio>();
    println!("Gpio {}", s);
    let b = core::mem::size_of::<Board>();
    println!("Board {}", b);
    println!("Option<()> {}", core::mem::size_of::<Option<()>>());
}

fn demo_singleton_usage() {
    #[allow(static_mut_refs)]
    let pa0 = unsafe { BOARD.take_input(&GpioId::A, 0).unwrap().into_floating() };
    pa0.pin_is_high();
}

fn main() {
    demo_memory_usages();
    demo_singleton_usage();
}
