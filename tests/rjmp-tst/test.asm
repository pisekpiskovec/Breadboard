.device ATmega16

Main:
    ldi r16, 4
    add r16, r16
    sub r16, r16
rjmp Main
