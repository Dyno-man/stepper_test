use stepper_lib::stepper_rotate;
use std::error::Error;

use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stepper1 = stepper_lib::stepper_rotate::Stepper::new(17,27,22,23, 4096,Duration::from_millis(2), true)?;
    stepper1.rotate()?;

    Ok(())
}
