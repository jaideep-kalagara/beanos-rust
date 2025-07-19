#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(beanos_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use beanos_rust::allocator;
use beanos_rust::memory;
use beanos_rust::memory::get_usable_memory;
use beanos_rust::println;
use beanos_rust::task::Task;
use beanos_rust::task::executor::Executor;
use beanos_rust::task::keyboard;
#[allow(unused_imports)]
use beanos_rust::vga_buffer;
use bootloader::BootInfo;
use bootloader::entry_point;
use core::panic::PanicInfo;
use raw_cpuid::CpuId;
use x86_64::VirtAddr;
entry_point!(kernel_main);

#[unsafe(no_mangle)]
// entry point
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    beanos_rust::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    println!("----------------------------");
    println!("BEANOS KERNEL");
    println!("----------------------------");
    println!();
    let cpuid = CpuId::new();
    println!(
        "Using CPU: {}",
        cpuid.get_processor_brand_string().unwrap().as_str()
    );
    println!("Memory: {} MiB", get_usable_memory(boot_info) / 1024 / 1024);
    println!("----------------------------");
    println!();

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::WRITER
        .lock()
        .change_color(vga_buffer::ColorCode::new(
            vga_buffer::Color::Red,
            vga_buffer::Color::Black,
        ));
    println!("{}", info);
    vga_buffer::WRITER
        .lock()
        .change_color(vga_buffer::ColorCode::new(
            vga_buffer::Color::LightGreen,
            vga_buffer::Color::Black,
        ));
    beanos_rust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    beanos_rust::test_panic_handler(info)
}
