use crate::mcu::{InputRegisterBlock, OutputRegisterBlock, Register};

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
pub type InputPullDown = Port<Input, PinPullDown, DontCare, InputRegisterBlock>;
pub type InputPullUp = Port<Input, PinPullUp, DontCare, InputRegisterBlock>;
pub type InputFloating = Port<Input, PinFloating, DontCare, InputRegisterBlock>;

pub type DisabledOutput = Port<Output, DontCare, DontCare, OutputRegisterBlock>;
pub type OutputPushPull = Port<Output, DontCare, PushPull, OutputRegisterBlock>;
pub type OutputOpenDrain = Port<Output, DontCare, OpenDrain, OutputRegisterBlock>;
pub type OutputPushPullPullUp = Port<Output, PinPullUp, PushPull, OutputRegisterBlock>;
pub type OutputPushPullPullDown = Port<Output, PinPullDown, PushPull, OutputRegisterBlock>;
pub type OutputOpenDrainPullUp = Port<Output, PinPullUp, OpenDrain, OutputRegisterBlock>;
pub type OutputOpenDrainPullDown = Port<Output, PinPullDown, OpenDrain, OutputRegisterBlock>;

pub struct Input;
pub struct Output;
pub struct Enabled;
pub struct DontCare;
pub struct PushPull;
pub struct OpenDrain;
pub struct PinPullUp;
pub struct PinPullDown;
pub struct PinFloating;

// Marker trait to indicate the input pin mode is configured (not `DontCare`).
pub trait ConfiguredInput {}
impl ConfiguredInput for PinPullUp {}
impl ConfiguredInput for PinPullDown {}
impl ConfiguredInput for PinFloating {}

// marker trait to indicate output pins are configured (not `DontCare`).
pub trait ConfiguredOutput {}
impl ConfiguredOutput for PushPull {}
impl ConfiguredOutput for OpenDrain {}
impl ConfiguredOutput for PinPullUp {}
impl ConfiguredOutput for PinPullDown {}

pub struct Port<Direction, PinMode, OutputType, REGISTERS> {
    gpio: GpioId,
    pin: u8,
    direction: Direction,
    pin_mode: PinMode,
    otype: OutputType,
    registers: REGISTERS,
}

pub fn new_output(gpio: &GpioId, pin: u8) -> DisabledOutput {
    Port {
        gpio: *gpio,
        pin,
        direction: Output,
        pin_mode: DontCare,
        otype: DontCare,
        registers: OutputRegisterBlock::new(),
    }
}

impl<OTYPE> Port<Output, DontCare, OTYPE, OutputRegisterBlock> {
    pub fn into_push_pull(self) -> OutputPushPull {
        Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: Output,
            pin_mode: DontCare,
            otype: PushPull,
            registers: OutputRegisterBlock::new()
        }
    }

    pub fn into_open_drain(self) -> OutputPushPull {
        Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: Output,
            pin_mode: DontCare,
            otype: PushPull,
            registers: OutputRegisterBlock::new()
        }
    }
}

impl<PINMOD> Port<Output, PINMOD, PushPull, OutputRegisterBlock> {
    pub fn into_pulled_up(self) -> OutputPushPullPullUp {
        Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: Output,
            pin_mode: PinPullUp,
            otype: PushPull,
            registers: self.registers,
        }
    }

    pub fn into_pulled_down(self) -> OutputPushPullPullDown {
        Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: Output,
            pin_mode: PinPullDown,
            otype: PushPull,
            registers: self.registers,
        }
    }
}

impl<PINMOD> Port<Output, PINMOD, OpenDrain, OutputRegisterBlock> {
    pub fn into_pulled_up(self) -> OutputOpenDrainPullUp {
        Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: Output,
            pin_mode: PinPullUp,
            otype: OpenDrain,
            registers: self.registers,
        }
    }

    pub fn into_pulled_down(self) -> OutputOpenDrainPullDown {
        Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: Output,
            pin_mode: PinPullDown,
            otype: OpenDrain,
            registers: self.registers,
        }
    }
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

impl<PINMOD> Port<Input, PINMOD, DontCare, InputRegisterBlock> {
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
}

impl<PINMOD: ConfiguredInput> Port<Input, PINMOD, DontCare, InputRegisterBlock> {
    pub fn pin_is_high(&self) -> bool {
        // Read the input data register for the configured pin.
        // Actual register-read logic not yet implemented; keep placeholder.
        println!("Reading pin {} of GPIO {:?}", self.pin, self.gpio);
        
        true
    }
}

impl <PINMOD, OTYPE: ConfiguredOutput> Port<Output, PINMOD, OTYPE, OutputRegisterBlock> {
    pub fn set_high(&self) {
        // Write to the output data register for the configured pin.
        // Actual register-write logic not yet implemented; keep placeholder.
        println!("Setting pin {} of GPIO {:?} high", self.pin, self.gpio);
    }

    pub fn set_low(&self) {
        // Write to the output data register for the configured pin.
        // Actual register-write logic not yet implemented; keep placeholder.
        println!("Setting pin {} of GPIO {:?} low", self.pin, self.gpio);
    }
}