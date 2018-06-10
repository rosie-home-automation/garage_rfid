use std::thread;
use std::time::Duration;
use sysfs_gpio;

pub struct GpioUtil;

impl GpioUtil {
  pub fn setup_input_pin(pin_num: usize, edge: sysfs_gpio::Edge) -> sysfs_gpio::Pin {
    let pin = sysfs_gpio::Pin::new(pin_num as u64);
    pin.export().expect("Expected to export input pin.");
    thread::sleep(Duration::from_millis(120)); // Delay for sysfs to export the files or you may get permission denied
    pin.set_direction(sysfs_gpio::Direction::In)
      .expect("Expected to set pin direction to an input.");
    pin.set_edge(edge).expect("Expected to set input pin edge.");
    pin
  }

  pub fn setup_output_pin(pin_num: usize) -> sysfs_gpio::Pin {
    let pin = sysfs_gpio::Pin::new(pin_num as u64);
    pin.export().expect("Expected to export input pin.");
    thread::sleep(Duration::from_millis(120)); // Delay for sysfs to export the files or you may get permission denied
    pin.set_direction(sysfs_gpio::Direction::Low)
      .expect("Expected to set pin direction to an output.");
    pin
  }
}
