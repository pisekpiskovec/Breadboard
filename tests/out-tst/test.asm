.NOLIST
.include "m16def.inc"
.LIST

; Data Direction
ldi r16, 0xFF
out DDRA, r16

; Sending Data
ldi r16, 0xF0
out PortA, r16
