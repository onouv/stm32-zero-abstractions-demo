use super::mcu::InputRegisterBlock;

use super::gpio::{Gpio, GpioId};

struct Input;
struct Output;

struct PushPull;
struct OpenDrain;
struct Disabled;
struct PinPullUp;
struct PinPullDown;
struct PinFloating;

enum PinMode {
    PullUp,
    PullDown,
    Floating,
}

pub struct Port<DIRECTION, PIN_MODE, OUTPUT_TYPE, REGISTERS> {
    gpio: GpioId,
    pin: u8,
    direction: DIRECTION,
    pin_mode: PIN_MODE,
    otype: OUTPUT_TYPE,
    registers: REGISTERS,
}

impl<DIR, PIN, OTYPE, REG> Port<DIR, PIN, OTYPE, REG> {
    fn into_input(self, id: GpioId, pin: u8, pin_mode: PinMode) -> Result<Gpio, ()> {
        match pin_mode {
            PinMode::PullUp => Ok(Gpio::InputPullUp(Port {
                gpio: id,
                pin,
                direction: Input,
                pin_mode: PinPullUp,
                otype: Disabled,
                registers: InputRegisterBlock
            }))),
            PinMode::PullDown => Ok(Gpio::InputPullDown(Port {
                gpio: id,
                pin,
                direction: Input,
                pin_mode: PinPullDown,
                otype: Disabled,
                registers: InputRegisterBlock
            })),
            PinMode::Floating => Err(())
        }
    }
}

pub type InputPullUp = Port<Input, PinPullUp, Disabled, InputRegisterBlock>;
pub type InputPullDown = Port<Input, PinPullDown, Disabled, InputRegisterBlock>;
