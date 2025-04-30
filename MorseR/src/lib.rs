#![no_std]
#![no_main]
#![feature(linkage)]

use core::panic::PanicInfo;

#[cfg(not(any(feature = "boot2", feature = "startup", feature = "transmit")))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "boot2")]
pub mod boot_stage2;

#[cfg(feature = "startup")]
pub mod startup;

#[cfg(feature = "transmit")]
pub mod transmit;