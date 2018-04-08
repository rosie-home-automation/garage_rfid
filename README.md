# garage_rfid
A rust implementation of the garage/RFID controller.

## TODO
- [x] Implement RFID Reader
  - [x] Read GPIO input from 2 pins concurrently
- [x] Implement DB
  - [x] SQLite
  - [x] Migrations
- [x] Implement authorizer
  - Called it a bouncer
- [ ] Implement Garage Door
- [ ] Use bcrypt to encrypt credential values
- [ ] Implement API
  - [x] Users#index
  - [ ] Users#show
  - [ ] Users#create
  - [ ] Users#update
  - [ ] Users#destroy
  - [ ] Credentials#index
  - [ ] Credentials#show
  - [ ] Credentials#create
  - [ ] Credentials#update
  - [ ] Credentials#destroy
  - [ ] GarageDoor#show
  - [ ] GarageDoor#toggle
  - [ ] GarageDoor#open
  - [ ] GarageDoor#close
- [ ] Many refactors...
  - [ ] R2D2
    - https://github.com/diesel-rs/r2d2-diesel/blob/master/examples/sqlite.rs
    - https://github.com/gotham-rs/gotham/pull/198/files
  - [ ] Check out tokio instead of mio for RfidReader


## Resources
- https://rustbyexample.com/index.html
- https://github.com/rust-embedded/rust-sysfs-gpio
  - https://github.com/rust-embedded/rust-sysfs-gpio/blob/master/examples/tokio.rs
  - http://rust-embedded.github.io/rust-sysfs-gpio/sysfs_gpio/struct.Pin.html#method.get_stream
- Rust + JSON library https://docs.rs/serde_json/1.0.9/serde_json/#operating-on-untyped-json-values
- http://hermanradtke.com/2017/03/03/future-mpsc-queue-with-tokio.html
  - Ended up ditching tokio and using mio directly.
