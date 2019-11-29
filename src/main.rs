#![no_std]
#![feature(lang_items, start, link_args)]

extern crate nocorelib;

fn main() {
}

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
    0
}

/// The exception handling personality function for use in the bootstrap.
///
/// We have no exception handling in the kernel, so make it do nothing.
#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

/// Function called on `panic!` invocation.
///
/// Kernel panics.
#[panic_handler] #[no_mangle]
pub extern fn panic_fmt(p: &::core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
