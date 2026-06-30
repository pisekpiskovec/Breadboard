# Instructions

For currently supported instructions, see table below.

| Instruction | Full Name                              | Notes                                |
| ----------- | -------------------------------------- | ------------------------------------ |
| ADC         | Add with Carry                         |                                      |
| ADD         | Add without Carry                      |                                      |
| ADIW        | Add Immediate to Word                  |                                      |
| AND         | Logical AND                            |                                      |
| ANDI        | Logical AND with Immediate             |                                      |
| ASR         | Arithmetic Shift Right                 |                                      |
| BCLR        | Bit Clear in SREG                      |                                      |
| BRBC        | Branch if Bit in SREG is Cleared       |                                      |
| BRBS        | Branch if Bit in SREG is Set           |                                      |
| BRCC        | Branch if Carry Cleared                | This is handled by BRBC instruction. |
| BRCS        | Branch if Carry Set                    | This is handled by BRBS instruction. |
| BREQ        | Branch if Equal                        | This is handled by BRBS instruction. |
| BRGE        | Branch if Greater or Equal (Signed)    | This is handled by BRBS instruction. |
| BRHC        | Branch if Half Carry Flag is Cleared   | This is handled by BRBC instruction. |
| BRHS        | Branch if Half Carry Flag is Set       | This is handled by BRBS instruction. |
| BRID        | Branch if Global Interrupt is Disabled | This is handled by BRBC instruction. |
| BRIE        | Branch if Global Interrupt is Enabled  | This is handled by BRBS instruction. |
| BRLO        | Branch if Lower (Unsigned)             | This is handled by BRBS instruction. |
| BRLT        | Branch if Less Than (Signed)           | This is handled by BRBS instruction. |
| BRMI        | Branch if Minus                        | This is handled by BRBS instruction. |
| BRNE        | Branch if Not Equal                    | This is handled by BRBC instruction. |
| BRPL        | Branch if Plus                         | This is handled by BRBC instruction. |
| BRSH        | Branch if Same or Higher (Unsigned)    | This is handled by BRBC instruction. |
| BRTC        | Branch if the T Flag is Cleared        | This is handled by BRBC instruction. |
| BRTS        | Branch if the T Flag is Set            | This is handled by BRBS instruction. |
| BRVC        | Branch if Overflow Cleared             | This is handled by BRBC instruction. |
| BRVS        | Branch if Overflow Set                 | This is handled by BRBS instruction. |
| BSET        | Bit Set in SREG                        |                                      |
| CALL        | Long Call to a Subroutine              |                                      |
| CBI         | Clear Bit in I/O Register              | I/O is currently not supported.      |
| CBR         | Clear Bits in Register                 | This is handled by ANDI instruction. |
| CLC         | Clear Carry Flag                       | This is handled by BCLR instruction. |
| CLH         | Clear Half Carry Flag                  | This is handled by BCLR instruction. |
| CLI         | Clear Global Interrupt Flag            | This is handled by BCLR instruction. |
| CLN         | Clear Negative Flag                    | This is handled by BCLR instruction. |
| CLR         | Clear Register                         | This is handled by EOR instruction.  |
| CLS         | Clear Signed Flag                      | This is handled by BCLR instruction. |
| CLT         | Clear T Flag                           | This is handled by BCLR instruction. |
| CLV         | Clear Overflow Flag                    | This is handled by BCLR instruction. |
| CLZ         | Clear Zero Flag                        | This is handled by BCLR instruction. |
| CP          | Compare                                |                                      |
| DEC         | Decrement                              |                                      |
| EOR         | Exclusive OR                           |                                      |
| INC         | Increment                              |                                      |
| JMP         | Jump                                   |                                      |
| LDI         | Load Immediate                         |                                      |
| MOV         | Copy Register                          |                                      |
| NOP         | No Operation                           |                                      |
| OR          | Logical OR                             |                                      |
| ORI         | Logical OR with Immediate              |                                      |
| OUT         | Store Register to I/O Location         | I/O is currently not supported.      |
| POP         | Pop Register from Stack                |                                      |
| PUSH        | Push Register on Stack                 |                                      |
| RCALL       | Relative Call to Subroutine            |                                      |
| RET         | Return from Subroutine                 |                                      |
| RETI        | Return from Interrupt                  |                                      |
| RJMP        | Relative Jump                          |                                      |
| SEC         | Set Carry Flag                         | This is handled by BSET instruction. |
| SEH         | Set Half Carry Flag                    | This is handled by BSET instruction. |
| SEI         | Set Global Interrupt Flag              | This is handled by BSET instruction. |
| SEN         | Set Negative Flag                      | This is handled by BSET instruction. |
| SES         | Set Signed Flag                        | This is handled by BSET instruction. |
| SET         | Set T Flag                             | This is handled by BSET instruction. |
| SEV         | Set Overflow Flag                      | This is handled by BSET instruction. |
| SEZ         | Set Zero Flag                          | This is handled by BSET instruction. |
| SUB         | Subtract without Carry                 |                                      |

Other instruction will result in `NOP` and won't be executed, making program stuck in place.
