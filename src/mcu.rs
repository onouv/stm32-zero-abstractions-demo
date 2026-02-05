mod register;

pub use register::*;

pub type Address = *mut u32;

mod addresses {
    // AHB Peripheral Clock Enable Register
    pub const RCC_BASE_ADDR: u32 = 0x4002_1000;
    pub const RCC_AHBENR_OFFSET: u32 = RCC_BASE_ADDR + 0x14;

    // GPIO port control registers
    pub const GPIO_BASE_ADDR: u32 = 0x4800_0000;
    pub const GPIO_MODER_OFFSET: u32 = 0;
    pub const GPIO_OTYPER_OFFSET: u32 = 0x4;
    pub const GPIO_SPEEDR_OFFSER: u32 = 0x8;
    pub const GPIO_PUPDR_OFFSET: u32 = 0xC;
    pub const GPIO_IDR_OFFSET: u32 = 0x10;
    pub const GPIO_ODR_OFFSET: u32 = 0x14;
    pub const GPIO_BSRR_OFFSET: u32 = 0x18;
}
