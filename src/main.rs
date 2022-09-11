#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
use core::arch::asm;
extern crate avr_std_stub;

#[no_mangle]

fn main() {
    const DDRB: *mut u8 = 0x24 as *mut u8;
    const PORTB: *mut u8 = 0x25 as *mut u8;
    loop {
        unsafe {
            core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | 0b00001111);
            core::ptr::write_volatile(PORTB, 0b00001010);
            for _ in 0..1000000 {
                asm!("NOP");
            }
            core::ptr::write_volatile(PORTB, 0b00000101);
            for _ in 0..1000000 {
                asm!("NOP");
            }
        }
    }
}
