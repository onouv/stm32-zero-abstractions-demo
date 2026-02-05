
#![allow(clippy::upper_case_acronyms, non_camel_case_types)]
use std::ops::Add;

use crate::{gpio::GpioId, mcu::Address};

use super::addresses::*;
pub trait Register {
    fn set_bit(&self, gpio: GpioId, bit_pos: u32, bit_val: u32);
}

pub struct RCC_AHBENR;// AHB Peripheral Clock Enable Register

fn calc_gpio_offset(gpio: &GpioId) -> u32 {
    match gpio {
        GpioId::A => 0,
        GpioId::B => 0x400,
        GpioId::C => 0x800,
        GpioId::D => 0xC00,
        GpioId::E => 0x1000,
        GpioId::F => 0x1400
    }
}

impl Register for RCC_AHBENR {
    fn set_bit(&self, gpio: GpioId, bit_pos: u32, bit_val: u32) {
        let _address =(RCC_BASE_ADDR + RCC_AHBENR_OFFSET) as Address;
        println!("set bit {} to {} in AHBENR ({:?})", bit_pos, bit_val, gpio);
    }
}

pub struct GPIOx_MODER;
impl Register for GPIOx_MODER {
    fn set_bit(&self, gpio: GpioId, bit_pos: u32, bit_val: u32) {
        let _address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_MODER_OFFSET) as Address;
        println!("set bit {} to {} in AHBENR ({:?})", bit_pos, bit_val, gpio);
    }
}

pub struct GPIOx_OTYPER;
impl Register for GPIOx_OTYPER {
    fn set_bit(&self, gpio: GpioId, bit_pos: u32, bit_val: u32) {
        let _address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_OTYPER_OFFSET) as Address;
        println!("set bit {} to {} in AHBENR ({:?})", bit_pos, bit_val, gpio);
    }
}

pub struct GPIOx_PUPDR;
impl Register for GPIOx_PUPDR {
    fn set_bit(&self, gpio: GpioId, bit_pos: u32, bit_val: u32) {
        let _address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_PUPDR_OFFSET) as Address;
        println!("set bit {} to {} in AHBENR ({:?})", bit_pos, bit_val, &gpio);
    }
}
pub struct GPIOx_IDR;
impl Register for GPIOx_IDR {
    fn set_bit(&self, gpio: GpioId, bit_pos: u32, bit_val: u32) {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_IDR_OFFSET) as Address;
        println!("set bit {} to {} in AHBENR ({:?}) @{}", bit_pos, bit_val, &gpio, address as u32);
    }
}
pub struct GPIOx_ODR;
impl Register for GPIOx_ODR {
    fn set_bit(&self, gpio: GpioId, bit_pos: u32, bit_val: u32) {
        let _address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_ODR_OFFSET) as Address;
        println!("set bit {} to {} in AHBENR ({:?})", bit_pos, bit_val, &gpio);
    }
}
pub struct GPIOx_BSRR;
impl Register for GPIOx_BSRR {
    fn set_bit(&self, gpio: GpioId, bit_pos: u32, bit_val: u32) {
        let _address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_BSRR_OFFSET) as Address;
        println!("set bit {} to {} in AHBENR ({:?})", bit_pos, bit_val, &gpio);
    }
}
pub struct InputRegisterBlock {
    ahbenr: RCC_AHBENR,
    moder: GPIOx_MODER,
    pub otyper: GPIOx_OTYPER,
    pub idr: GPIOx_IDR
}

impl InputRegisterBlock {
    pub fn new() -> Self {
        Self {
            ahbenr: RCC_AHBENR {},
            moder: GPIOx_MODER {},
            otyper: GPIOx_OTYPER {},
            idr: GPIOx_IDR {}
        }
    }
}

pub struct OutputRegisterBlock {
    pub ahbenr: RCC_AHBENR,
    pub moder: GPIOx_MODER,
    pub otyper: GPIOx_OTYPER,
    pub pupdr: GPIOx_PUPDR,
    pub odr: GPIOx_ODR,
    pub bsrr: GPIOx_BSRR
}

