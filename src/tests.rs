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
/// Load adds 2 to 254 in r16
fn tst_add() {
    let mut cpu = ATmemory::init();
    let program: Vec<u8> = vec![0x0E, 0x0F, 0x12, 0xE0, 0x01, 0x0F];
    cpu.load_flash_from_vec(program.clone()).ok();
    for _ in program.iter().enumerate().take(2) {
        cpu.step().ok();
    }
    assert_eq!(cpu.registers()[16], 0x00)
}
