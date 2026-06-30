# Instructions

For currently supported instructions, see table below.

| Instruction | Full Name                                                | Notes                                | Implemented |
| ----------- | -------------------------------------------------------- | ------------------------------------ | ----------- |
| ADC         | Add with Carry                                           |                                      | Y           |
| ADD         | Add without Carry                                        |                                      | Y           |
| ADIW        | Add Immediate to Word                                    |                                      | Y           |
| AND         | Logical AND                                              |                                      | Y           |
| ANDI        | Logical AND with Immediate                               |                                      | Y           |
| ASR         | Arithmetic Shift Right                                   |                                      | Y           |
| BCLR        | Bit Clear in SREG                                        |                                      | Y           |
| BLD         | Bit Load from the T Flag in SREG to a Bit in Register    |                                      | N           |
| BRBC        | Branch if Bit in SREG is Cleared                         |                                      | Y           |
| BRBS        | Branch if Bit in SREG is Set                             |                                      | Y           |
| BRCC        | Branch if Carry Cleared                                  | This is handled by BRBC instruction. | Y           |
| BRCS        | Branch if Carry Set                                      | This is handled by BRBS instruction. | Y           |
| BREAK       | Break                                                    |                                      | N           |
| BREQ        | Branch if Equal                                          | This is handled by BRBS instruction. | Y           |
| BRGE        | Branch if Greater or Equal (Signed)                      | This is handled by BRBS instruction. | Y           |
| BRHC        | Branch if Half Carry Flag is Cleared                     | This is handled by BRBC instruction. | Y           |
| BRHS        | Branch if Half Carry Flag is Set                         | This is handled by BRBS instruction. | Y           |
| BRID        | Branch if Global Interrupt is Disabled                   | This is handled by BRBC instruction. | Y           |
| BRIE        | Branch if Global Interrupt is Enabled                    | This is handled by BRBS instruction. | Y           |
| BRLO        | Branch if Lower (Unsigned)                               | This is handled by BRBS instruction. | Y           |
| BRLT        | Branch if Less Than (Signed)                             | This is handled by BRBS instruction. | Y           |
| BRMI        | Branch if Minus                                          | This is handled by BRBS instruction. | Y           |
| BRNE        | Branch if Not Equal                                      | This is handled by BRBC instruction. | Y           |
| BRPL        | Branch if Plus                                           | This is handled by BRBC instruction. | Y           |
| BRSH        | Branch if Same or Higher (Unsigned)                      | This is handled by BRBC instruction. | Y           |
| BRTC        | Branch if the T Flag is Cleared                          | This is handled by BRBC instruction. | Y           |
| BRTS        | Branch if the T Flag is Set                              | This is handled by BRBS instruction. | Y           |
| BRVC        | Branch if Overflow Cleared                               | This is handled by BRBC instruction. | Y           |
| BRVS        | Branch if Overflow Set                                   | This is handled by BRBS instruction. | Y           |
| BSET        | Bit Set in SREG                                          |                                      | Y           |
| BST         | Bit Store from Bit in Register to T Flag in SREG         |                                      | N           |
| CALL        | Long Call to a Subroutine                                |                                      | Y           |
| CBI         | Clear Bit in I/O Register                                | I/O is currently not supported.      | Y           |
| CBR         | Clear Bits in Register                                   | This is handled by ANDI instruction. | Y           |
| CLC         | Clear Carry Flag                                         | This is handled by BCLR instruction. | Y           |
| CLH         | Clear Half Carry Flag                                    | This is handled by BCLR instruction. | Y           |
| CLI         | Clear Global Interrupt Flag                              | This is handled by BCLR instruction. | Y           |
| CLN         | Clear Negative Flag                                      | This is handled by BCLR instruction. | Y           |
| CLR         | Clear Register                                           | This is handled by EOR instruction.  | Y           |
| CLS         | Clear Signed Flag                                        | This is handled by BCLR instruction. | Y           |
| CLT         | Clear T Flag                                             | This is handled by BCLR instruction. | Y           |
| CLV         | Clear Overflow Flag                                      | This is handled by BCLR instruction. | Y           |
| CLZ         | Clear Zero Flag                                          | This is handled by BCLR instruction. | Y           |
| COM         | One's Complement                                         |                                      | N           |
| CP          | Compare                                                  |                                      | Y           |
| CPC         | Compare with Carry                                       |                                      | N           |
| CPI         | Compare with Immediate                                   |                                      | N           |
| CPSE        | Compare Skip if Equal                                    |                                      | N           |
| DEC         | Decrement                                                |                                      | Y           |
| EOR         | Exclusive OR                                             |                                      | Y           |
| FMUL        | Fractional Multiply Unsigned                             |                                      | N           |
| FMULS       | Fractional Multiply Signed                               |                                      | N           |
| FMULSU      | Fractional Multiply Signed with Unsigned                 |                                      | N           |
| ICALL       | Indirect Call to Subroutine                              |                                      | N           |
| IJMP        | Indirect Jump                                            |                                      | N           |
| IN          | Load an I/O Location to Register                         |                                      | N           |
| INC         | Increment                                                |                                      | Y           |
| JMP         | Jump                                                     |                                      | Y           |
| LD          | Load Indirect from Data Space to Register using Index X  |                                      | N           |
| LDD         | Load Indirect from Data Space to Register using Index Y  |                                      | N           |
| LDD         | Load Indirect from Data Space to Register using Index Z  |                                      | N           |
| LDI         | Load Immediate                                           |                                      | Y           |
| LDS         | Load Direct from Data Space                              |                                      | N           |
| LPM         | Load Program Memory                                      |                                      | N           |
| LSL         | Logical Shift Left                                       |                                      | N           |
| LSR         | Logical Shift Right                                      |                                      | N           |
| MOV         | Copy Register                                            |                                      | Y           |
| MOVW        | Copy Register Word                                       |                                      | N           |
| MUL         | Multiply Unsigned                                        |                                      | N           |
| MULS        | Multiply Signed                                          |                                      | N           |
| MULSU       | Multiply Signed with Unsigned                            |                                      | N           |
| NEG         | Two's Complement                                         |                                      | N           |
| NOP         | No Operation                                             |                                      | Y           |
| OR          | Logical OR                                               |                                      | Y           |
| ORI         | Logical OR with Immediate                                |                                      | Y           |
| OUT         | Store Register to I/O Location                           | I/O is currently not supported.      | Y           |
| POP         | Pop Register from Stack                                  |                                      | Y           |
| PUSH        | Push Register on Stack                                   |                                      | Y           |
| RCALL       | Relative Call to Subroutine                              |                                      | Y           |
| RET         | Return from Subroutine                                   |                                      | Y           |
| RETI        | Return from Interrupt                                    |                                      | Y           |
| RJMP        | Relative Jump                                            |                                      | Y           |
| ROL         | Rotate Left trough Carry                                 |                                      | N           |
| ROR         | Rotate Right trough Carry                                |                                      | N           |
| SBC         | Subtract with Carry                                      |                                      | N           |
| SBCI        | Subtract Immediate with Carry                            |                                      | N           |
| SBI         | Set Bit in I/O Register                                  |                                      | N           |
| SBIC        | Skip if Bit in I/O Register is Cleared                   |                                      | N           |
| SBIS        | Skip if Bit in I/O Register is Set                       |                                      | N           |
| SBIW        | Subtract Immediate from Word                             |                                      | N           |
| SBR         | Set Bits in Register                                     |                                      | N           |
| SBRC        | Skip if Bit in Register is Cleared                       |                                      | N           |
| SBRS        | Skip if Bit in Register is Set                           |                                      | N           |
| SEC         | Set Carry Flag                                           | This is handled by BSET instruction. | Y           |
| SEH         | Set Half Carry Flag                                      | This is handled by BSET instruction. | Y           |
| SEI         | Set Global Interrupt Flag                                | This is handled by BSET instruction. | Y           |
| SEN         | Set Negative Flag                                        | This is handled by BSET instruction. | Y           |
| SER         | Set all Bits in Register                                 |                                      | N           |
| SES         | Set Signed Flag                                          | This is handled by BSET instruction. | Y           |
| SET         | Set T Flag                                               | This is handled by BSET instruction. | Y           |
| SEV         | Set Overflow Flag                                        | This is handled by BSET instruction. | Y           |
| SEZ         | Set Zero Flag                                            | This is handled by BSET instruction. | Y           |
| SLEEP       | Sleep                                                    |                                      | N           |
| SPM         | Store Program Memory                                     |                                      | N           |
| ST          | Store Indirect From Register to Data Space using Index X |                                      | N           |
| STD         | Store Indirect From Register to Data Space using Index Y |                                      | N           |
| STD         | Store Indirect From Register to Data Space using Index Z |                                      | N           |
| STS         | Store Direct to Data Space                               |                                      | N           |
| SUB         | Subtract without Carry                                   |                                      | Y           |
| SUBI        | Subtract Immediate                                       |                                      | N           |
| SWAP        | Swap Nibbles                                             |                                      | N           |
| TST         | Test for Zero or Minus                                   |                                      | N           |
| WDR         | Watchdog Reset                                           |                                      | N           |

Other instruction will result in `NOP` and won't be executed, making program stuck in place.
