#![no_std]
#![no_main]

use arduino_hal::{port::mode, port::Pin};
use panic_halt as _;

const FULL_STEP: [[u8; 4]; 4] = [
  [1, 0, 0, 0],
  [0, 1, 0, 0],
  [0, 0, 1, 0],
  [0, 0, 0, 1]
];

struct Stepper {
    pins: [Pin<mode::Output>; 4],
    current_steps: u8,
    step_sequence: [[u8; 4]; 4]
}

fn full_step(_stepper: &mut Stepper, forward: bool) {
    if forward {
        _stepper.current_steps = (_stepper.current_steps + 1) % 4;
    } else {
        _stepper.current_steps = (_stepper.current_steps - 1) % 4;
    }
    for i in 0..4 {
        if _stepper.step_sequence[_stepper.current_steps as usize][i] == 1 {
            _stepper.pins[i].set_high();
        } else {
            _stepper.pins[i].set_low();
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut stepper = Stepper{
        pins: [pins.d8.into_output().downgrade(), pins.d9.into_output().downgrade(), pins.d10.into_output().downgrade(), pins.d11.into_output().downgrade()],
        current_steps: 0,
        step_sequence: FULL_STEP
    };

    loop {
        full_step(&mut stepper, true);
        arduino_hal::delay_ms(500);
    }
}
