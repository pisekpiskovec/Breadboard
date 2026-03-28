.device ATmega16

clear:
clr r30
clr r31

setup:
ldi r31, 41
ldi r30, 14

cp r31, r30
brge xoring
rjmp setup

xoring:
eor r30, r31
rjmp clear

