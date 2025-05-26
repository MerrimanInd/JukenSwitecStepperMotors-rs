#![no_std]

use embedded_hal::digital::{OutputPin, PinState};
use embedded_hal::delay::DelayNs;

// When zeroing the library will drive the motor home for some set duration.
const ZEROING_STEP_MICROSEC: u32 = 800;

// These are the states of each of the 4 pins in each of the 6 positions of the motor.
// Since Rust's bools are one byte and the PinState enum is even larger this is the most memory efficient way to store these state, at the cost of a little bit shifting later on.
const STATE_MAP: [u8; 6] = [0x9, 0x1, 0x7, 0x6, 0xE, 0x8];
// State  3 2 1 0   Value
// 0      1 0 0 1   0x9
// 1      0 0 0 1   0x1
// 2      0 1 1 1   0x7
// 3      0 1 1 0   0x6
// 4      1 1 1 0   0xE
// 5      1 0 0 0   0x8

// This would be the more intuitive way to store the state map but would eat up an unnecessary chunk of memory
// const STATE_MAP_2:[[PinState; 4]; 6] = [
//     [PinState::High, PinState::Low, PinState::Low, PinState::High],
//     [PinState::Low, PinState::Low, PinState::Low, PinState::High],
//     [PinState::Low, PinState::High, PinState::High, PinState::High],
//     [PinState::Low, PinState::High, PinState::High, PinState::Low],
//     [PinState::High, PinState::High, PinState::High, PinState::Low],
//     [PinState::High, PinState::Low, PinState::Low, PinState::Low],
// ];

const STATE_COUNT: usize = 6;

const DEFAULT_ACCEL_TABLE: [[u16; 2]; 5] = [[20, 3000], [50, 1500], [100, 1000], [150, 800], [300, 600]];

const DEFAULT_ACCEL_TABLE_SIZE: u8 = DEFAULT_ACCEL_TABLE.len() as u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    IoError(embedded_hal::digital::ErrorKind),
}

impl From<embedded_hal::digital::ErrorKind> for Error {
    fn from(value: embedded_hal::digital::ErrorKind) -> Self {
        Error::IoError(value)
    }
}

pub enum Direction {
    Stopped,
    Forward,
    Reverse,
}


pub struct Motor<T: OutputPin> {
    steps: u16,
    pins: [T; 4],
    current_state: usize,

    direction: Direction,
    velocity: u8,
    stopped: bool,
    current_step: u16,
    target_step: u16,
}

impl<T: OutputPin> Motor<T> {
    pub fn new(steps: u16, pin_1: T, pin_2: T, pin_3: T, pin_4: T) -> Self {
        Self {
            steps,
            pins: [pin_1, pin_2, pin_3, pin_4],
            current_state: 0,

            direction: Direction::Stopped,
            velocity: 0,
            stopped: true,
            current_step: 0,
            target_step: 0,
        }
    }

    fn write_io(&mut self) -> Result<(), Error> {
        let mut mask = STATE_MAP[self.current_state];
        for i in 0..4 {
            let pin_state = match mask & 0x1 {
              0 => {PinState::Low},
              _ => {PinState::High},
              // Ignoring all other cases since this is indexing a const vec defined above and cannot fail.
            };
            self.pins[i].set_state(pin_state).unwrap();
            mask = mask >> 1;
        };
        Ok(())
    }

    pub fn step_up(&mut self) {
        if self.current_step < self.steps {
            self.current_step += 1;
            self.current_state = (self.current_state + 1) % STATE_COUNT;
            self.write_io();
        }
    }

    pub fn step_down(&mut self) {
        if self.current_step > 0 {
            self.current_step -= 1;
            self.current_state = (self.current_state + 5) % STATE_COUNT;
            self.write_io();
        }
    }

    pub fn zero(&mut self, mut delay: impl DelayNs) {
        self.current_step = self.steps - 1;
        for _ in 0..self.steps {
            self.step_down();
            delay.delay_us(ZEROING_STEP_MICROSEC);
        }
        self.current_step = 0;
        self.target_step = 0;
        self.velocity = 0;
        self.direction = Direction::Stopped;
    }

    pub fn advance(&mut self, current_time: usize) {
        // detect stopped state
        if (self.current_step == self.target_step && self.velocity == 0) {
            self.stopped = true;
            self.direction = Direction::Stopped;
            let time_0 = current_time;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
