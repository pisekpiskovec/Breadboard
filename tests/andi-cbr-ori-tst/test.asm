.device ATmega16

ldi r16, 228
andi r16, 29

nop
nop

ldi r16, 228
cbr r16, 0b11000000

nop
nop

ldi r16, 228
ori r16, 29
