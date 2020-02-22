#![no_std]
#![no_main]

use core::panic::PanicInfo;
use riscv_rt::entry;

use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_hal::Peripherals;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let clocks = k210_hal::clock::Clocks::new();

    let serial = p.UARTHS.configure((p.pins.pin5, p.pins.pin4), 115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();
    let mut stdout = Stdout(&mut tx);

    let mut x = 0;
    loop {
        let _res = writeln!(stdout, "Hello World: {}", x);
        x = x + 1;
    }

}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
