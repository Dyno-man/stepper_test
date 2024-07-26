use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
    // Physical pins are 11, 13, 15, 16: Using Broadcom numbering instead for crate
    let gpio = Gpio::new()?;
    let mut in1 = gpio.get(17)?.into_output();
    let mut in2 = gpio.get(27)?.into_output();
    let mut in3 = gpio.get(22)?.into_output();
    let mut in4 = gpio.get(23)?.into_output();

    let mut motor_pins = [&mut in1, &mut in2, &mut in3, &mut in4];

    let full_rotation = 4096; // One full rotation for the stepper motor
    let step_sleep = Duration::from_millis(2);

    let direction = true; // Direction of rotation

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

    println!("Rotating a stepper motor on a {}", DeviceInfo::new()?.model());
    for _ in 0..full_rotation {
        for (pin, sequence) in motor_pins.iter_mut().zip(step_sequence[motor_step_counter].iter()) {
            if *sequence == 1 {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
        if direction {
            motor_step_counter = (motor_step_counter - 1) % 8;
        } else {
            motor_step_counter = (motor_step_counter + 1) % 8;
        }
        thread::sleep(step_sleep);
    }

    Ok(())
}
