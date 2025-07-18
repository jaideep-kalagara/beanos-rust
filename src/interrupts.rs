use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::{gdt, hlt_loop, println, vga_buffer};
use lazy_static::lazy_static;

use pic8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
            idt.page_fault.set_handler_fn(page_fault_handler);

            idt[InterruptIndex::Timer.as_u8()].set_handler_fn(timer_interrupt_handler);
            idt[InterruptIndex::Keyboard.as_u8()]
                .set_handler_fn(crate::task::keyboard::keyboard_interrupt_handler);
        }
        idt
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

pub fn init_idt() {
    IDT.load();
}
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    vga_buffer::WRITER
        .lock()
        .change_color(vga_buffer::ColorCode::new(
            vga_buffer::Color::Yellow,
            vga_buffer::Color::Black,
        ));
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    vga_buffer::WRITER
        .lock()
        .change_color(vga_buffer::ColorCode::new(
            vga_buffer::Color::LightGreen,
            vga_buffer::Color::Black,
        ));
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    vga_buffer::WRITER
        .lock()
        .change_color(vga_buffer::ColorCode::new(
            vga_buffer::Color::Yellow,
            vga_buffer::Color::Black,
        ));
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    vga_buffer::WRITER
        .lock()
        .change_color(vga_buffer::ColorCode::new(
            vga_buffer::Color::LightGreen,
            vga_buffer::Color::Black,
        ));
    hlt_loop();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    } // send EOI
}

// breakpoint test
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
