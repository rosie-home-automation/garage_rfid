# garage_rfid
A rust implementation of the garage/RFID controller.

# TODO
- [ ] Read GPIO input from 2 pins concurrently
  - The println! works [here](https://github.com/rosie-home-automation/garage_rfid/blob/82a414b2e5e95d4ee877991a7ae640eefe089b67/src/main.rs#L54), but the rx doesn't work [here](https://github.com/rosie-home-automation/garage_rfid/blob/82a414b2e5e95d4ee877991a7ae640eefe089b67/src/main.rs#L82)
- [ ] Implement API

## Resources
- https://rustbyexample.com/index.html
- https://github.com/rust-embedded/rust-sysfs-gpio
  - https://github.com/rust-embedded/rust-sysfs-gpio/blob/master/examples/tokio.rs
  - http://rust-embedded.github.io/rust-sysfs-gpio/sysfs_gpio/struct.Pin.html#method.get_stream
- http://hermanradtke.com/2017/03/03/future-mpsc-queue-with-tokio.html
