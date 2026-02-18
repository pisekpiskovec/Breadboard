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
    for (idx, _) in program.iter().enumerate() {
        println!("{}", idx);
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
    for (idx, _) in program.iter().enumerate() {
        println!("{}", idx);
        cpu.step().ok();
    }
    assert_eq!(cpu.registers()[16], 124)
}
