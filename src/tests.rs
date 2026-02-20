#![cfg(test)]

use crate::memory::ATmemory;

#[test]
/// Load 255 to r17
fn tst_ldi() {
    let mut cpu = ATmemory::init();
    let program: Vec<u8> = vec![0x1F, 0xEF];
    cpu.load_flash_from_vec(program).ok();
    cpu.step().ok();
    assert_eq!(cpu.registers()[17], 0xFF)
}

#[test]
/// Adds 16 + 3
fn tst_add() {
    let mut cpu = ATmemory::init();
    let program: Vec<u8> = vec![0x00, 0xE1, 0x13, 0xE0, 0x01, 0x0F];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in program.iter().enumerate() {
        cpu.step().ok();
    }
    assert_eq!(cpu.registers()[16], 19)
}

#[test]
/// Subtract 5 out of 129
fn tst_sub() {
    let mut cpu = ATmemory::init();
    let program: Vec<u8> = vec![0x01, 0xE8, 0x15, 0xE0, 0x01, 0x1B];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in program.iter().enumerate() {
        cpu.step().ok();
    }
    assert_eq!(cpu.registers()[16], 124)
}

#[test]
/// Call a subroutine
fn tst_rcall() {
    let mut cpu = ATmemory::init();
    let program: Vec<u8> = vec![
        0x02, 0xE1, 0x01, 0xC0, 0x03, 0x95, 0xFE, 0xDF, 0xFE, 0xCF,
    ];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in 0..4 {
        cpu.step().ok();
    }
    assert_eq!(
        (cpu.registers()[16], cpu.sram()[0x3FF], cpu.pc()),
        (19, 0x06, 0x0006)
    )
}
