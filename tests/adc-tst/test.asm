.device ATmega16

.def num1L = r16
.def num1H = r17
.def num2L = r18
.def num2H = r19

Main:
        ldi num1L, 0x34
        ldi num1H, 0x12
        ldi num2L, 0xCD
        ldi num2H, 0xAB

        add num1L, num2L
        adc num1H, num2H
rjmp Main
