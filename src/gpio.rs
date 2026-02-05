use crate::mcu::{ InputRegisterBlock, OutputRegisterBlock, Register };

#[derive(Debug, Clone, Copy)]
pub enum GpioId {
    A,
    B,
    C,
    D,
    E,
    F,
}

pub type DisabledInput = Port<Input, DontCare, DontCare, InputRegisterBlock>;
type InputPullDown = Port<Input, PinPullDown, DontCare, InputRegisterBlock>;
type InputPullUp = Port<Input, PinPullUp, DontCare, InputRegisterBlock>;
type InputFloating = Port<Input, PinFloating, DontCare, InputRegisterBlock>;
pub type DontCareOutput = Port<Output, DontCare, DontCare, OutputRegisterBlock>;
type OutputPushPullPullUp = Port<Output, PinPullUp, PushPull, OutputRegisterBlock>;
type OutputPushPullPullDown = Port<Output, PinPullDown, PushPull, OutputRegisterBlock>;
type OutputOpenDrainPullUp = Port<Output, PinPullUp, OpenDrain, OutputRegisterBlock>;
type OutputOpenDrainPullDown = Port<Output, PinPullDown, OpenDrain, OutputRegisterBlock>;

pub enum Gpio {
    InputPullUp(InputPullUp),
    InputPullDown(InputPullDown),
    OutputPushPullPullUp(OutputPushPullPullUp),
    OutputPushPullPullDown(OutputPushPullPullDown),
    OutputOpenDrainPullUp(OutputOpenDrainPullUp),
    OutputOpenDrainPullDown(OutputOpenDrainPullDown),
}

pub struct Input;
pub struct Output;
pub struct Enabled;
pub struct DontCare;
pub struct PushPull;
pub struct OpenDrain;
pub struct PinPullUp;
pub struct PinPullDown;
pub struct PinFloating;

enum PinMode {
    PullUp,
    PullDown,
    Floating,
}

enum OutputType {
    PushPull,
    OpenDrain,
}

pub struct Port<Direction, PinMode, OutputType, REGISTERS> {
    gpio: GpioId,
    pin: u8,
    direction: Direction,
    pin_mode: PinMode,
    otype: OutputType,
    registers: REGISTERS,
}

pub fn new_input(gpio: &GpioId, pin: u8) -> DisabledInput {
    Port {
        direction: Input,
        pin_mode: DontCare,
        otype: DontCare,
        registers: InputRegisterBlock::new(),
        pin,
        gpio: *gpio,
    }
}

impl<PIN> Port<Input, PIN, DontCare, InputRegisterBlock> {
    pub fn into_floating(self) -> InputFloating {
        Port {
            direction: Input,
            pin_mode: PinFloating,
            otype: DontCare,
            registers: self.registers, 
            gpio: self.gpio,
            pin: self.pin,
        }
    }

    pub fn into_pulled_up(self) -> InputPullUp {
        Port {
            direction: Input,
            pin_mode: PinPullUp,
            otype: DontCare,
            registers: self.registers,
            gpio: self.gpio,
            pin: self.pin,
        }
    }

    pub fn into_pulled_down(self) -> InputPullDown {
        Port {
            direction: Input,
            pin_mode: PinPullDown,
            otype: DontCare,
            registers: self.registers,
            gpio: self.gpio,
            pin: self.pin,
        }
    }

    pub fn pin_is_high(&self) -> bool {


        // TODO: WRONG, just for testing, remove
        self.registers.idr.set_bit(self.gpio, self.pin as u32, 0x32);
        true
    }
}
