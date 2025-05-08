#![allow(bad_asm_style)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(unsafe_op_in_unsafe_fn)]
#[warn(unused_imports)]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod vga;

// Include boot.s which defines _start as inline assembly in main. This allows us to do more fine
// grained setup than if we used a naked _start function in rust. Theoretically we could use a
// naked function + some inline asm, but this seems much more straight forward.
global_asm!(include_str!("boot.s"));


#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    println!("hello {}\n", 32);
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

