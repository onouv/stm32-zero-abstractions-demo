#![allow(unused)]
mod board;
mod gpio;
mod mcu;

use board::{BOARD, Board};
use gpio::*;

use core::mem::size_of;

fn demo_memory_usages() {
    println!("DisabledInput {}", size_of::<DisabledInput>());
    println!("InputFloating {}", size_of::<InputFloating>());
    println!("InputPullDown {}", size_of::<InputPullDown>());
    println!("Board {}", size_of::<Board>());
    println!("DisabledOutput {}", size_of::<DisabledOutput>());
    println!("OutputPushPullPullUp {}", size_of::<OutputPushPullPullUp>());
    println!(
        "OutputPushPullPullDown {}",
        size_of::<OutputPushPullPullDown>()
    );
    println!(
        "OutputOpenDrainPullUp {}",
        size_of::<OutputOpenDrainPullUp>()
    );
    println!(
        "OutputOpenDrainPullDown {}",
        size_of::<OutputOpenDrainPullDown>()
    );
}

fn demo_singleton_usage() {
    #[allow(static_mut_refs)]
    {
        println!("Demonstrating any configured input pin can read its state:");
        let pa0 = unsafe { BOARD.take_input(&GpioId::A, 0).unwrap().into_floating() };
        pa0.pin_is_high();

        let pa1 = unsafe { BOARD.take_input(&GpioId::A, 1).unwrap().into_pulled_down() };
        pa1.pin_is_high();

        let pa2 = unsafe { BOARD.take_input(&GpioId::A, 2).unwrap().into_pulled_up() };
        pa2.pin_is_high();

        println!("Demonstrating any configured output pin can set its state:");
        let pe0 = unsafe { BOARD.take_output(&GpioId::E, 0).unwrap().into_pushpull_pulled_up() };
        pe0.set_high();

        let pe1 = unsafe { BOARD.take_output(&GpioId::E, 1).unwrap().into_pushpull_pulled_down() };
        pe1.set_low();

        let pe2 = unsafe { BOARD.take_output(&GpioId::E, 2).unwrap().into_open_drain_pull_up() };
        pe2.set_high();

        let pe3 = unsafe { BOARD.take_output(&GpioId::E, 3).unwrap().into_open_drain_pull_down() };
        pe3.set_low();
    }
}

fn main() {
    demo_memory_usages();
    demo_singleton_usage();
}
