# Instructions

For currently supported instructions, see table below.

| Instruction | Full Name                      | Notes                                |
| ----------- | ------------------------------ | ------------------------------------ |
| ADC         | Add with Carry                 |                                      |
| ADD         | Add without Carry              |                                      |
| ADIW        | Add Immediate to Word          |                                      |
| AND         | Logical AND                    |                                      |
| ANDI        | Logical AND with Immediate     |                                      |
| ASR         | Arithmetic Shift Right         |                                      |
| CALL        | Long Call to a Subroutine      |                                      |
| CBI         | Clear Bit in I/O Register      | I/O is currently not supported.      |
| CBR         | Clear Bits in Register         | This is handled by ANDI instruction. |
| CLC         | Clear Carry Flag               |                                      |
| CLH         | Clear Half Carry Flag          |                                      |
| CLI         | Clear Global Interrupt Flag    |                                      |
| CLN         | Clear Negative Flag            |                                      |
| CLR         | Clear Register                 | This is handled by EOR instruction.  |
| CLS         | Clear Signed Flag              |                                      |
| CLT         | Clear T Flag                   |                                      |
| CLV         | Clear Overflow Flag            |                                      |
| CLZ         | Clear Zero Flag                |                                      |
| DEC         | Decrement                      |                                      |
| EOR         | Exclusive OR                   |                                      |
| INC         | Increment                      |                                      |
| JMP         | Jump                           |                                      |
| LDI         | Load Immediate                 |                                      |
| MOV         | Copy Register                  |                                      |
| NOP         | No Operation                   |                                      |
| OR          | Logical OR                     |                                      |
| ORI         | Logical OR with Immediate      |                                      |
| OUT         | Store Register to I/O Location | I/O is currently not supported.      |
| POP         | Pop Register from Stack        |                                      |
| PUSH        | Push Register on Stack         |                                      |
| RCALL       | Relative Call to Subroutine    |                                      |
| RET         | Return from Subroutine         |                                      |
| RETI        | Return from Interrupt          |                                      |
| RJMP        | Relative Jump                  |                                      |
| SEC         | Set Carry Flag                 |                                      |
| SUB         | Subtract without Carry         |                                      |

Other instruction will result in `NOP` and won't be executed, making program stuck in place.
