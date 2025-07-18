#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(beanos_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]



use core::{panic::PanicInfo};
use beanos_rust::println;
#[allow(unused_imports)]
use beanos_rust::vga_buffer;
use bootloader::BootInfo;
use bootloader::entry_point;
use beanos_rust::memory;
use x86_64::{VirtAddr, structures::paging::Page};
entry_point!(kernel_main);   

#[unsafe(no_mangle)]
// entry point
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    beanos_rust::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;
    
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};


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