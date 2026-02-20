.device ATmega16

Reset:
        ldi r16, 0x12
rjmp Main

increment_reg:
        inc r16

Main:
        rcall increment_reg
rjmp Main
