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

#[repr(C, packed)]
#[derive(Debug)]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u32; 4],
    pub mmap_length: u32,
    pub mmap_addr: u32,
}

struct MultibootMmapEntry {
    size: u32,
    addr: u32,
    len: u32,
    typ: u32
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(multiboot_info_ptr: u32, multiboot_magic_ptr: u32) -> ! {
    let mb_info = unsafe { &*(multiboot_info_ptr as *const MultibootInfo) };

    let mut mmap_addr = mb_info.mmap_addr;
    let mmap_length = mb_info.mmap_length;
    let mmap_end = mmap_addr + mmap_length;

    println!("Magic Number: {:#x}", multiboot_magic_ptr);

    println!(
        "mmap_length: {}\nmmap_addr: {:#x}", mmap_length, mmap_addr
    );

    while mmap_addr < mmap_end {
        let entry = unsafe { &*(mmap_addr as *const MultibootMmapEntry) };

        println!(
            "size: {} addr: {:#x} len: {} typ: {:#x}",
            entry.size,
            entry.addr,
            entry.len,
            entry.typ
        );

        mmap_addr += entry.size + 4; // add size of `size` field
    }

    loop {}
}


// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

