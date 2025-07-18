#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(beanos_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]



use core::{panic::PanicInfo};
use beanos_rust::println;
#[allow(unused_imports)]
use beanos_rust::vga_buffer;



#[unsafe(no_mangle)]
// entry point
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    beanos_rust::init();


    #[cfg(test)]
    test_main();


    println!("Did this run?");
    beanos_rust::hlt_loop();
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::WRITER.lock().change_color(vga_buffer::ColorCode::new(vga_buffer::Color::Red, vga_buffer::Color::Black));
    println!("{}", info);
    vga_buffer::WRITER.lock().change_color(vga_buffer::ColorCode::new(vga_buffer::Color::LightGreen, vga_buffer::Color::Black));
    beanos_rust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    beanos_rust::test_panic_handler(info)
}