extern crate libc;
use libc::{c_int, c_void};

type RawBlink1 = c_void;

#[link(name = "Blink1")]
extern {
    fn blink1_enumerate() -> c_int;
    fn blink1_open() -> *RawBlink1;
    fn blink1_close(device: *RawBlink1);
    fn blink1_setRGB(device: *RawBlink1, red: u8, green: u8, blue: u8) -> c_int;
}

#[deriving(Rand)]
struct Rgb(u8, u8, u8);

struct Blink1 {
    dev: *RawBlink1
}

impl Blink1 {
    pub fn open() -> Option<Blink1> {
        unsafe {
            blink1_open().to_option().map(|p| {
                Blink1 { dev: p }
            })
        }
    }

    pub fn set_rgb(&self, rgb: Rgb) {
        unsafe {
            let Rgb(r, g, b) = rgb;
            blink1_setRGB(self.dev, r, g, b);
        }
    }
}

impl Drop for Blink1 {
    fn drop(&mut self) {
        unsafe {
            blink1_close(self.dev);
        }
    }
}
pub fn main() {
    use std::io::timer::sleep;
    use std::rand;

    let x = unsafe { blink1_enumerate() };
    println!("num of devices {}", x);

    let blink1 = Blink1::open().unwrap();
    let green = Rgb(255, 0, 0);
    blink1.set_rgb(green);
    sleep(500);
    let crazy: Rgb = rand::random();
    blink1.set_rgb(crazy);
}
