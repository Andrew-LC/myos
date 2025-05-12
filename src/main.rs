#![allow(bad_asm_style)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(unsafe_op_in_unsafe_fn)]
#[warn(unused_imports)]


use core::arch::global_asm;
use core::panic::PanicInfo;

mod vga;
mod allocator;

extern crate alloc;

use alloc::vec::Vec;


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

#[repr(C, packed)]
#[derive(Debug)]
struct MultibootMmapEntry {
    size: u32,
    addr: u64,
    len: u64,
    typ: u32
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(mb_info: *const MultibootInfo, multiboot_magic_ptr: u32) -> ! {
    let mem_lower = unsafe { (*mb_info).mem_lower };
    let mem_upper = unsafe { (*mb_info).mem_upper };
    let mut mmap_addr = unsafe { (*mb_info).mmap_addr };
    let mmap_length = unsafe { (*mb_info).mmap_length };
    let mmap_end = mmap_addr + mmap_length;

    println!("Magic Number: {:#x}", multiboot_magic_ptr);
    println!(
        "mem_lower: {}kb\nmem_upper: {}kb\nmmap_length: {}\nmmap_addr: {:#x}\nmmap_end: {}",
        mem_lower, mem_upper, mmap_length, mmap_addr, mmap_end
    );

    while mmap_addr < mmap_end {
	let entry = unsafe { &*(mmap_addr as *const MultibootMmapEntry) };

	// Copy each field to a local variable to avoid unaligned reference
	let size = core::ptr::addr_of!(entry.size);
	let addr = core::ptr::addr_of!(entry.addr);
	let len  = core::ptr::addr_of!(entry.len);
	let typ  = core::ptr::addr_of!(entry.typ);

	println!(
            "size: {} addr: {:#x} len: {} typ: {}",
            unsafe { size.read_unaligned() },
	    unsafe { addr.read_unaligned() },
	    unsafe { len.read_unaligned() / 1024 },
	    unsafe { typ.read_unaligned() }
	);

	// Move to next entry (size doesn't include size field itself)
	mmap_addr += entry.size + 4;
    }

    loop {}
}



// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("We panicked!");
    loop {}
}

