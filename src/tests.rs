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

#[test]
/// Logical AND test
fn tst_and() {
    let mut cpu = ATmemory::init();
    // ldi r16, 228
    // ldi r17, 29
    // and r16, r17
    let program: Vec<u8> = vec![0x04, 0xEE, 0x1D, 0xE1, 0x01, 0x23];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!(cpu.memory()[16], 4)
}

#[test]
/// Logical EXCLUSIVE OR test
fn tst_xor() {
    let mut cpu = ATmemory::init();
    // ldi r16, 228
    // ldi r17, 29
    // eor r16, r17
    let program: Vec<u8> = vec![0x04, 0xEE, 0x1D, 0xE1, 0x01, 0x27];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!(cpu.memory()[16], 249)
}

#[test]
/// Logical OR test
fn tst_or() {
    let mut cpu = ATmemory::init();
    // ldi r16, 228
    // ldi r17, 29
    // or r16, r17
    let program: Vec<u8> = vec![0x04, 0xEE, 0x1D, 0xE1, 0x01, 0x2B];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!(cpu.memory()[16], 253)
}

#[test]
/// Logical AND with Immediate test
fn tst_andi() {
    let mut cpu = ATmemory::init();
    // ldi r16, 228
    // andi r16, 29
    let program: Vec<u8> = vec![0x04, 0xEE, 0x0D, 0x71];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!(cpu.memory()[16], 4)
}

#[test]
/// Logical OR with Immediate test
fn tst_ori() {
    let mut cpu = ATmemory::init();
    // ldi r16, 228
    // ori r16, 29
    let program: Vec<u8> = vec![0x04, 0xEE, 0x0D, 0x61];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!(cpu.memory()[16], 253)
}

#[test]
/// 16-bit add
fn tst_adc() {
    let mut cpu = ATmemory::init();
    // ldi r16, 0x34
    // ldi r17, 0x12
    // ldi r18, 0xCD
    // ldi r19, 0xAB
    // add r16, r18
    // adc r17, r19
    let program: Vec<u8> = vec![
        0x04, 0xE3, 0x12, 0xE1, 0x2D, 0xEC, 0x3B, 0xEA, 0x02, 0x0F, 0x13, 0x1F,
    ];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!((cpu.memory()[16], cpu.memory()[17]), (0x01, 0xBE))
}

#[test]
/// 16-bit add immediate
fn tst_adiw() {
    let mut cpu = ATmemory::init();
    // ldi r24, 255
    // adiw r25:r24, 1
    let program: Vec<u8> = vec![0x8F, 0xEF, 0x01, 0x96];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!((cpu.memory()[24], cpu.memory()[25]), (0x00, 0x01))
}

#[test]
fn tst_asr() {
    let mut cpu = ATmemory::init();
    // ldi r16, 150
    // asr r16
    let program: Vec<u8> = vec![0x06, 0xE9, 0x05, 0x95];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!((cpu.memory()[16], cpu.sreg() & 0x01), (203, 0))
}

#[test]
fn tst_out() {
    let mut cpu = ATmemory::init();
    // ldi r21, 95
    // out SPL, r21
    let program: Vec<u8> = vec![0x5F, 0xE5, 0x5D, 0xBF];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..(program.len() / 2) {
        cpu.step().ok();
    }
    assert_eq!(cpu.memory()[93], 95)
}
