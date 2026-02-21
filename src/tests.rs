#![cfg(test)]

use crate::memory::ATmemory;

#[test]
/// Load 255 to r17
fn tst_ldi() {
    let mut cpu = ATmemory::init();
    let program: Vec<u8> = vec![0x1F, 0xEF];
    cpu.load_flash_from_vec(program).ok();
    cpu.step().ok();
    assert_eq!(cpu.memory()[17], 0xFF)
}

#[test]
/// Adds 16 + 3
fn tst_add() {
    let mut cpu = ATmemory::init();
    let program: Vec<u8> = vec![0x00, 0xE1, 0x13, 0xE0, 0x01, 0x0F];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!(cpu.memory()[16], 19)
}

#[test]
/// Subtract 5 out of 129
fn tst_sub() {
    let mut cpu = ATmemory::init();
    let program: Vec<u8> = vec![0x01, 0xE8, 0x15, 0xE0, 0x01, 0x1B];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!(cpu.memory()[16], 124)
}

#[test]
/// Call a subroutine
fn tst_rcall() {
    let mut cpu = ATmemory::init();
    // Reset:
    //     ldi r16, 0x12
    // rjmp Main
    //
    // increment_reg
    //     inc r16
    // ret
    //
    // Main:
    //     rcall increment_reg
    // rjmp Main
    let program: Vec<u8> = vec![0x02, 0xE1, 0x02, 0xC0, 0x03, 0x95, 0x08, 0x95, 0xFD, 0xDF];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!(
        (cpu.memory()[16], cpu.memory()[0x45E], cpu.pc()),
        (19, 0x05, 0x0005)
    )
}

#[test]
/// Push from Stack
fn tst_push() {
    let mut cpu = ATmemory::init();
    // ldi r16, 24
    // push r16
    // ldi r16, 00
    let program: Vec<u8> = vec![0x08, 0xE1, 0x0F, 0x93, 0x00, 0xE0];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!((cpu.memory()[16], cpu.memory()[0x45E]), (00, 24))
}

#[test]
/// Push and Pop from Stack
fn tst_pop() {
    let mut cpu = ATmemory::init();
    // ldi r16, 24
    // push r16
    // ldi r16, 00
    // pop r16
    let program: Vec<u8> = vec![0x08, 0xE1, 0x0F, 0x93, 0x00, 0xE0, 0x0F, 0x91];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!(
        (cpu.memory()[16], cpu.memory()[0x45E], cpu.sp()),
        (24, 24, 0x45F)
    )
}
