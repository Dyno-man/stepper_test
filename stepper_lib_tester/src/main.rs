use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stepper1 = stepper_lib::stepper_rotate::Stepper::new(17,27,22,23, 2048,Duration::from_millis(2), true)?;
    let mut stepper2 = stepper_lib::stepper_rotate::Stepper::new(5, 6, 13, 12, 4096, Duration::from_millis(2), true)?; 
    
    stepper1.rotate()?;
    stepper2.rotate()?;

    Ok(())
}
