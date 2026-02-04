#![allow(unused)]

use heapless::{IndexMap, index_map::FnvIndexMap};
#[derive(Debug)]

enum GpioId {
    A,
    B,
    C,
    D,
    E,
    F,
}

trait Register {
    fn set_bit(&self, gpio: GpioId, bit_pos: u32, bit_val: u32);
}

struct AHBENR;

impl Register for AHBENR {
    fn set_bit(&self, gpio: GpioId, bit_pos: u32, bit_val: u32) {
        let _address = 0x4800_1418 as *mut u32;
        println!("set bit {} to {} in AHBENR ({:?})", bit_pos, bit_val, gpio);
    }
}

struct Led;

pub struct Input;
pub struct Output;

pub struct Enabled;
pub struct Disabled;

pub struct DontCare;

pub struct Port<MODE, ENABLED> {
    mode: MODE,
    enabled: ENABLED,
}

type DisabledInput = Port<Input, Disabled>;
type EnabledInput = Port<Input, Enabled>;

enum Gpio {
    DisabledInput(DisabledInput),
    EnabledInput(EnabledInput),
}

impl<MOD, EN> Port<MOD, EN> {
    fn into_input(self /*, id: GPIO, pin: u32*/) -> Result<Gpio, ()> {
        Ok(Gpio::DisabledInput(Port {
            mode: Input,
            enabled: Disabled,
        }))
    }
}

#[derive(Debug)]
enum BoardError {
    ResourceTaken,
    ResourceUnsupported,
    Unknown,
}

const NUM_PINS: usize = 16;
const NUM_GPIOX: usize = 6;
struct Board {
    // static-safe flag matrix
    // 896 usize :-0 for the fixed-sized associative array (abandoned)
    //      ports: FnvIndexMap<GpioId, FnvIndexMap<u8, Option<()>, NUM_PINS>, NUM_GPIOX>,
    // 96 usize :-) but be REALLY careful when indexing this
    ports: [[Option<()>; NUM_PINS]; NUM_GPIOX], 
}
impl Board {
    fn take_input(&mut self, gpio: GpioId, pin: u32) -> Result<Gpio, BoardError> {

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

static mut BOARD: Board = Board {
    ports: [[Some(()); NUM_PINS]; NUM_GPIOX] 
};


fn demo_memory_usages() {
    let s = core::mem::size_of::<DisabledInput>();
    println!("DisabledInput {}", s);
    let e = core::mem::size_of::<Gpio>();
    println!("Gpio {}", s);
    let l = core::mem::size_of::<Led>();
    println!("Led {}", l);
    let b = core::mem::size_of::<Board>();
    println!("Board {}", b);
    println!("Option<()> {}", core::mem::size_of::<Option<()>>());
}

fn demo_singleton_usage() {
    #[allow(static_mut_refs)]
    let pa0 = unsafe { BOARD.take_input(GpioId::A, 0).unwrap() };    
}

fn main() {

    demo_memory_usages();
    demo_singleton_usage();
}
