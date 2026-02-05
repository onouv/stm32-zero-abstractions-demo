use crate::{gpio::*, port::*};

pub struct Led;

#[derive(Debug)]
pub enum BoardError {
    ResourceTaken,
    ResourceUnsupported,
    Unknown,
}

const NUM_PINS: usize = 16;
const NUM_GPIOX: usize = 6;
pub struct Board {
    // static-safe flag matrix
    // 896 usize :-0 for the fixed-sized associative array (abandoned)
    //      ports: FnvIndexMap<GpioId, FnvIndexMap<u8, Option<()>, NUM_PINS>, NUM_GPIOX>,
    // 96 usize :-) but be REALLY careful when indexing this
    ports: [[Option<()>; NUM_PINS]; NUM_GPIOX], 
}
impl Board {
    pub fn take_input(&mut self, gpio: GpioId, pin: u32) -> Result<Gpio, BoardError> {

        // find index into flag matrix
        let (g,p): (usize, usize) = match (gpio, pin) {
            (GpioId::A, 0) => (0,0),
            (GpioId::E, 8) => (5,8),
            _ => {
                return Err(BoardError::ResourceUnsupported);
            }
        };

        // check the flag matrix and mark as taken, if not already taken
        unsafe {
            let mut port = self.ports[p][g];
            if port.is_none() {
                return Err(BoardError::ResourceTaken);
            }
            port.take();
        }

        // TODO: At this point, the input port should have a set of the correct GPIOx registers it will need
        Ok(Gpio::DisabledInput(DisabledInput { mode: Input, enabled: Disabled }))    }
}

pub static mut BOARD: Board = Board {
    ports: [[Some(()); NUM_PINS]; NUM_GPIOX] 
};

