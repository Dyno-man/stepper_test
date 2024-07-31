use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

pub mod stepper_rotate {
    use super::*;

    pub struct Stepper {
        in1: OutputPin,
        in2: OutputPin,
        in3: OutputPin,
        in4: OutputPin,
        full_rotation_steps: usize,
        step_sleep: Duration,
        direction: bool,
    }

    impl Stepper {
        pub fn new(in1: u8, in2: u8, in3: u8, in4: u8, full_rotation_steps: usize, step_sleep: Duration, direction: bool) -> Result<Stepper, Box<dyn Error>> {
            let gpio = Gpio::new()?;
            let in1_pin = gpio.get(in1)?.into_output();
            let in2_pin = gpio.get(in2)?.into_output();
            let in3_pin = gpio.get(in3)?.into_output();
            let in4_pin = gpio.get(in4)?.into_output();

            Ok(Stepper {
                in1: in1_pin,
                in2: in2_pin,
                in3: in3_pin,
                in4: in4_pin,
                full_rotation_steps,
                step_sleep,
                direction,
            })
        }

        pub fn rotate(&mut self) -> Result<(), Box<dyn Error>> {
            let mut motor_pins = [&mut self.in1, &mut self.in2, &mut self.in3, &mut self.in4];

            let step_sequence = [
                [1, 0, 0, 1],
                [1, 0, 0, 0],
                [1, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 1, 1],
                [0, 0, 0, 1],
            ]; // Stepper sequence

            let mut motor_step_counter = 0;

            for _ in 0..self.full_rotation_steps {
                for (pin, sequence) in motor_pins.iter_mut().zip(step_sequence[motor_step_counter].iter()) {
                    if *sequence == 1 {
                        pin.set_high();
                    } else {
                        pin.set_low();
                    }
                }
                if self.direction {
                    motor_step_counter = (motor_step_counter - 1) % 8;
                } else {
                    motor_step_counter = (motor_step_counter + 1) % 8;
                }
                thread::sleep(self.step_sleep);
            }

            Ok(())
        }
    }
}