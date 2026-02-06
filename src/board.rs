use crate::gpio::{self, DisabledInput, DisabledOutput, GpioId};

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
    pub fn take_input(&mut self, gpio: &GpioId, pin: u8) -> Result<DisabledInput, BoardError> {
        let (g, p): (usize, usize) = get_port_indices(gpio, pin)?;
        // check the flag matrix and mark as taken, if not already taken
        unsafe {
            let mut port = self.ports[p][g];
            if port.is_none() {
                return Err(BoardError::ResourceTaken);
            }
            port.take();
        }

        Ok(gpio::new_input(gpio, pin))
    }

    pub fn take_output(&mut self, gpio: &GpioId, pin: u8) -> Result<DisabledOutput, BoardError> {
        let (g, p): (usize, usize) = get_port_indices(gpio, pin)?;

        unsafe {
            let mut port = self.ports[g][p];
            if port.is_none() {
                return Err(BoardError::ResourceTaken);
            }
            port.take();
            println!("ports[g][p] = [{:?}][{}] is taken: {}", gpio, pin, port.is_none());
        }

        Ok(gpio::new_output(gpio, pin))
    }
}

pub static mut BOARD: Board = Board {
    ports: [[Some(()); NUM_PINS]; NUM_GPIOX],
};

fn get_port_indices(gpio: &GpioId, pin: u8) -> Result<(usize, usize), BoardError> {
    match (gpio, pin) {
        // Note the indices for gpio are 0-based,
        // while the pin numbers are 1-based.
        // This is intentional to simplify indexing into the flag matrix.
        (GpioId::A, 0) => Ok((0, 0)),
        (GpioId::A, 1) => Ok((0, 1)),
        (GpioId::A, 2) => Ok((0, 2)),
        (GpioId::E, 0) => Ok((4, 0)),
        (GpioId::E, 1) => Ok((4, 1)),
        (GpioId::E, 2) => Ok((4, 2)),
        (GpioId::E, 3) => Ok((4, 3)),
        _ => Err(BoardError::ResourceUnsupported),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds_returns_error() {
        unsafe {

            #[allow(static_mut_refs)]
            let pe8 = BOARD.take_output(&GpioId::E, 8);
            assert!(pe8.is_err(), "PE8 should be out of bounds and return an error");
        }
    }
}
