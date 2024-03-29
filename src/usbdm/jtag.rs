#![allow(non_snake_case)]

use super::*;
use crate::errors::{Error, USBDM_ErrorCode};
    
pub const JTAG_COMMAND_MASK         : u8 = 0x7<<5;

pub const JTAG_MISC0                : u8 = 0<<5;
pub const JTAG_MISC1                : u8 = 1<<5;
pub const JTAG_MISC2                : u8 = 2<<5;

//============================================================================================
// The following have no operands
pub const JTAG_END                  : u8 = 0;   // Mark end of sequence
pub const JTAG_NOP                  : u8 = 1;   // No-Operation
pub const JTAG_END_SUB              : u8 = 2;   // Mark end of subroutine (also acts as implicit JTAG_RETURN)
pub const JTAG_TEST_LOGIC_RESET     : u8 = 3;   // Reset TAP

pub const JTAG_MOVE_DR_SCAN         : u8 = 4;   // Move TAP to JTAG_SHIFT_DR (from IDLE or SHIFT-DR/IR)
pub const JTAG_MOVE_IR_SCAN         : u8 = 5;   // Move TAP to JTAG_SHIFT_IR (from IDLE)

pub const JTAG_SET_STAY_SHIFT       : u8 = 6;   // Set Stay in JTAG_SHIFT_DR/IR after shift
pub const JTAG_SET_EXIT_SHIFT_DR    : u8 = 7;   // Set exit to JTAG_SHIFT_DR w/o crossing RUN-TEST-IDLE after shift
pub const JTAG_SET_EXIT_SHIFT_IR    : u8 = 8;   // Set exit to JTAG_SHIFT_IR w/o crossing RUN-TEST-IDLE after shift
pub const JTAG_SET_EXIT_IDLE        : u8 = 9;   // Set exit to RUN-TEST/IDLE after shift
pub const JTAG_SET_IN_FILL_0        : u8 = 10;  // Shift in '0' during JTAG_SHIFT_OUT
pub const JTAG_SET_IN_FILL_1        : u8 = 11;  // Shift in '1' during JTAG_SHIFT_OUT (default)

pub const JTAG_ELSE                 : u8 = 12;  // Else Marker for JTAG_IF..
pub const JTAG_END_IF               : u8 = 13;  // EndIf Marker for JTAG_IF..
pub const JTAG_RETURN               : u8 = 14;  // Return from subroutine - ignores iteration
pub const JTAG_BREAK                : u8 = 15;  // Break JTAG_REPEAT loop
pub const JTAG_CONTINUE             : u8 = 16;  // Continue next JTAG_REPEAT iteration
pub const JTAG_END_REPEAT           : u8 = 17;  // Marks end of JTAG_REPEAT loop

//============================================================================================
// The following have an 8-bit operand as the next byte
                                    // Operand
pub const JTAG_SET_ERROR            : u8 = 18;  // Error#    Set error variable & exit sequence if != 0

pub const JTAG_DEBUG_ON             : u8 = 19;  // Turn on debugging messages (on PC interpreter)
pub const JTAG_DEBUG_OFF            : u8 = 63;  // Turn off debugging messages (on PC interpreter)

//============================================================================================
// The following have no operands
pub const fn JTAG_SUB(x: u8)       -> u8 { 20 + x }
pub const JTAG_SUBA                 : u8 = JTAG_SUB(0);       // Mark start of subroutine A
pub const JTAG_SUBB                 : u8 = JTAG_SUB(1);       // Mark start of subroutine B
pub const JTAG_SUBC                 : u8 = JTAG_SUB(2);       // Mark start of subroutine C
pub const JTAG_SUBD                 : u8 = JTAG_SUB(3);       // Mark start of subroutine D

pub const fn JTAG_CALL_SUB(x: u8)  -> u8 { 24 + x }
pub const JTAG_CALL_SUBA            : u8 = JTAG_CALL_SUB(0);  // Call subroutine A
pub const JTAG_CALL_SUBB            : u8 = JTAG_CALL_SUB(1);  // Call subroutine B
pub const JTAG_CALL_SUBC            : u8 = JTAG_CALL_SUB(2);  // Call subroutine C
pub const JTAG_CALL_SUBD            : u8 = JTAG_CALL_SUB(3);  // Call subroutine D

//============================================================================================
// The following use a value previously set by JTAG_PUSH...
                                    // 5/8/16/32 bit operand from JTAG_PUSH...
pub const JTAG_IF_VARA_EQ           : u8 = 28;  // Value     IF statement testing variable A
pub fn JTAG_IF_VARA_EQ_Q(x: u8)    -> Vec<u8> { add_uu(JTAG_PUSH_Q(x),  JTAG_IF_VARA_EQ) }  // 5-bit value
pub fn JTAG_IF_VARA_EQ_8(x: u8)    -> Vec<u8> { add_vu(JTAG_PUSH_8(x),  JTAG_IF_VARA_EQ) }  // 8-bit value
pub fn JTAG_IF_VARA_EQ_16(x: u16)  -> Vec<u8> { add_vu(JTAG_PUSH_16(x), JTAG_IF_VARA_EQ) }  // 16-bit value
pub fn JTAG_IF_VARA_EQ_32(x: u32)  -> Vec<u8> { add_vu(JTAG_PUSH_32(x), JTAG_IF_VARA_EQ) }  // 32-bit value

pub const JTAG_IF_VARB_EQ           : u8 = 29;  // Value     IF statement testing variable B
pub fn JTAG_IF_VARB_EQ_Q(x: u8)    -> Vec<u8> { add_uu(JTAG_PUSH_Q(x),  JTAG_IF_VARB_EQ) }  // 5-bit value
pub fn JTAG_IF_VARB_EQ_8(x: u8)    -> Vec<u8> { add_vu(JTAG_PUSH_8(x),  JTAG_IF_VARB_EQ) }  // 8-bit value
pub fn JTAG_IF_VARB_EQ_16(x: u16)  -> Vec<u8> { add_vu(JTAG_PUSH_16(x), JTAG_IF_VARB_EQ) }  // 16-bit value
pub fn JTAG_IF_VARB_EQ_32(x: u32)  -> Vec<u8> { add_vu(JTAG_PUSH_32(x), JTAG_IF_VARB_EQ) }  // 32-bit value

pub const JTAG_IF_ITER_NEQ          : u8 = 30;  // Value     IF statement testing iteration number
pub fn JTAG_IF_ITER_NEQ_Q(x: u8)   -> Vec<u8> { add_uu(JTAG_PUSH_Q(x),  JTAG_IF_ITER_NEQ) }  // 5-bit value
pub fn JTAG_IF_ITER_NEQ_8(x: u8)   -> Vec<u8> { add_vu(JTAG_PUSH_8(x),  JTAG_IF_ITER_NEQ) }  // 8-bit value
pub fn JTAG_IF_ITER_NEQ_16(x: u16) -> Vec<u8> { add_vu(JTAG_PUSH_16(x), JTAG_IF_ITER_NEQ) }  // 16-bit value
pub fn JTAG_IF_ITER_NEQ_32(x: u32) -> Vec<u8> { add_vu(JTAG_PUSH_32(x), JTAG_IF_ITER_NEQ) }  // 32-bit value

pub const JTAG_IF_ITER_EQ           : u8 = 31;  // Value     IF statement testing iteration number
pub fn JTAG_IF_ITER_EQ_Q(x: u8)    -> Vec<u8> { add_uu(JTAG_PUSH_Q(x),  JTAG_IF_ITER_EQ) }  // 5-bit value
pub fn JTAG_IF_ITER_EQ_8(x: u8)    -> Vec<u8> { add_vu(JTAG_PUSH_8(x),  JTAG_IF_ITER_EQ) }  // 8-bit value
pub fn JTAG_IF_ITER_EQ_16(x: u16)  -> Vec<u8> { add_vu(JTAG_PUSH_16(x), JTAG_IF_ITER_EQ) }  // 16-bit value
pub fn JTAG_IF_ITER_EQ_32(x: u32)  -> Vec<u8> { add_vu(JTAG_PUSH_32(x), JTAG_IF_ITER_EQ) }  // 32-bit value

//============================================================================================
// The following have no operands
pub const fn JTAG_LOAD_VAR(x: u8)  -> u8 { 32 + x }  // Loads Variable from Temp
pub const JTAG_LOAD_VARA            : u8 = JTAG_LOAD_VAR(0);
pub fn JTAG_LOAD_VARA_Q(x: u8)     -> Vec<u8> { add_uu(JTAG_PUSH_Q(x),  JTAG_LOAD_VARA) }
pub fn JTAG_LOAD_VARA_8(x: u8)     -> Vec<u8> { add_vu(JTAG_PUSH_8(x),  JTAG_LOAD_VARA) }
pub fn JTAG_LOAD_VARA_16(x: u16)   -> Vec<u8> { add_vu(JTAG_PUSH_16(x), JTAG_LOAD_VARA) }
pub fn JTAG_LOAD_VARA_32(x: u32)   -> Vec<u8> { add_vu(JTAG_PUSH_32(x), JTAG_LOAD_VARA) }
pub fn JTAG_LOAD_VARA_DP_8()       -> Vec<u8> { add_uu(JTAG_PUSH_DP_8,  JTAG_LOAD_VARA) }

pub const JTAG_LOAD_VARB            : u8 = JTAG_LOAD_VAR(1);
pub fn JTAG_LOAD_VARB_Q(x: u8)     -> Vec<u8> { add_uu(JTAG_PUSH_Q(x),  JTAG_LOAD_VARB) }
pub fn JTAG_LOAD_VARB_8(x: u8)     -> Vec<u8> { add_vu(JTAG_PUSH_8(x),  JTAG_LOAD_VARB) }
pub fn JTAG_LOAD_VARB_16(x: u16)   -> Vec<u8> { add_vu(JTAG_PUSH_16(x), JTAG_LOAD_VARB) }
pub fn JTAG_LOAD_VARB_32(x: u32)   -> Vec<u8> { add_vu(JTAG_PUSH_32(x), JTAG_LOAD_VARB) }
pub fn JTAG_LOAD_VARB_DP_8()       -> Vec<u8> { add_uu(JTAG_PUSH_DP_8,  JTAG_LOAD_VARB) }

pub const fn JTAG_SAVEDP(x: u8)    -> u8 { 32 + x }
pub const JTAG_SAVE_OUT_DP_VARC     : u8 = JTAG_SAVEDP(2);      // Copy otDataPtr to VARC/D
pub const JTAG_SAVE_OUT_DP_VARD     : u8 = JTAG_SAVEDP(3);

pub const fn JTAG_RESTOREDP(x: u8) -> u8 { 36 + x }
pub const JTAG_RESTORE_DP_VARC      : u8 = JTAG_RESTOREDP(2);  // Restore dataPtr from VARC/D
pub const JTAG_RESTORE_DP_VARD      : u8 = JTAG_RESTOREDP(3);

//============================================================================================
// The following use a value previously set by JTAG_PUSH...
                                    // 5/8/16/32 bit operand from JTAG_PUSH...
pub const JTAG_IF_VARA_NEQ          : u8 = 36;  // Value     IF statement testing variable A
pub fn JTAG_IF_VARA_NEQ_Q(x: u8)   -> Vec<u8> { add_uu(JTAG_PUSH_Q(x),  JTAG_IF_VARA_NEQ) }  // 5-bit value
pub fn JTAG_IF_VARA_NEQ_8(x: u8)   -> Vec<u8> { add_vu(JTAG_PUSH_8(x),  JTAG_IF_VARA_NEQ) }  // 8-bit value
pub fn JTAG_IF_VARA_NEQ_16(x: u16) -> Vec<u8> { add_vu(JTAG_PUSH_16(x), JTAG_IF_VARA_NEQ) }  // 16-bit value
pub fn JTAG_IF_VARA_NEQ_32(x: u32) -> Vec<u8> { add_vu(JTAG_PUSH_32(x), JTAG_IF_VARA_NEQ) }  // 32-bit value

pub const JTAG_IF_VARB_NEQ          : u8 = 37;  // Value     IF statement testing variable B
pub fn JTAG_IF_VARB_NEQ_Q(x: u8)   -> Vec<u8> { add_uu(JTAG_PUSH_Q(x),  JTAG_IF_VARB_NEQ) }  // 5-bit value
pub fn JTAG_IF_VARB_NEQ_8(x: u8)   -> Vec<u8> { add_vu(JTAG_PUSH_8(x),  JTAG_IF_VARB_NEQ) }  // 8-bit value
pub fn JTAG_IF_VARB_NEQ_16(x: u16) -> Vec<u8> { add_vu(JTAG_PUSH_16(x), JTAG_IF_VARB_NEQ) }  // 16-bit value
pub fn JTAG_IF_VARB_NEQ_32(x: u32) -> Vec<u8> { add_vu(JTAG_PUSH_32(x), JTAG_IF_VARB_NEQ) }  // 32-bit value

//============================================================================================
// The following uses a value previously set by JTAG_PUSH...
                                    // 5/8/16/32 bit operand from JTAG_PUSH...
const JTAG_REPEAT               : u8 = 40;  // Value     Repeat a block N times
pub fn JTAG_REPEAT_16(x: u16)      -> Vec<u8> { add_vu(JTAG_PUSH_16(x), JTAG_REPEAT) }  // 16-bit value
pub fn JTAG_REPEAT_32(x: u32)      -> Vec<u8> { add_vu(JTAG_PUSH_32(x), JTAG_REPEAT) }  // 32-bit value
    
//============================================================================================
// The following use an 8-bit operand as next byte in sequence
const JTAG_REPEAT8: u8 = 41;
pub fn JTAG_REPEAT_8(x: u8)        -> Vec<u8> { add_uu(JTAG_REPEAT8, x) }   // 8-bit value

//============================================================================================
// The following push an 8/16/32-bit operand as the next 1/2/4 bytes in sequence (big-endian)
const JTAG_PUSH8                    : u8 = 42;
const JTAG_PUSH16                   : u8 = 43;
const JTAG_PUSH32                   : u8 = 44;

pub fn JTAG_PUSH_8 (x: u8)         -> Vec<u8> { vec![JTAG_PUSH8,  x] }                                                   // Push an 8-bit #
pub fn JTAG_PUSH_16(x: u16)        -> Vec<u8> { vec![JTAG_PUSH16, (x>>8)  as u8, x as u8] }                              // Push a 16-bit #
pub fn JTAG_PUSH_32(x: u32)        -> Vec<u8> { vec![JTAG_PUSH32, (x>>24) as u8, (x>>16) as u8, (x>>8) as u8, x as u8] } // Push a 32-bit #

//============================================================================================
// The following have an 8/16/32-bit operands from DP
pub const JTAG_PUSH_DP_8            : u8 = 45;
pub const JTAG_PUSH_DP_16           : u8 = 46;
pub const JTAG_PUSH_DP_32           : u8 = 47;

//==============================================================================================
// The following take no operands
pub const JTAG_SAVE_SUB             : u8 = 48;       // Save data in subroutine cache

//==============================================================================================
// The following have an 8-bit operand as the next byte, if zero then value is taken from dataPtr
pub const JTAG_SKIP_DP              : u8 = 49;
pub fn JTAG_SKIP_DP_Q(x: u8)       -> Vec<u8> { add_uu(JTAG_PUSH_Q(x),  JTAG_SKIP_DP) }  // #5=N Skip forward N bytes in Dataptr
pub fn JTAG_SKIP_DP_8(x: u8)       -> Vec<u8> { add_vu(JTAG_PUSH_8(x),  JTAG_SKIP_DP) }  // #8=N Skip forward N bytes in Dataptr

pub const JTAG_SHIFT_OUT_DP_VARA    : u8 = 50;                        // Shift out VARA bits, data taken from dataPtr
pub const JTAG_SET_BUSY             : u8 = 51;                        // Set BDM USB interface to send BUSY response

pub const fn JTAG_SHIFT_OUT_VAR(x: u8) -> u8 { 52 + x }               // #8=N    Shift out variable x to TDI
pub const JTAG_SHIFT_OUT_VARA       : u8 = JTAG_SHIFT_OUT_VAR(0);     // #8=N    Shift out variable A to TDI
pub const JTAG_SHIFT_OUT_VARB       : u8 = JTAG_SHIFT_OUT_VAR(1);     // #8=N    Shift out variable B to TDI
pub const JTAG_SHIFT_OUT_VARC       : u8 = JTAG_SHIFT_OUT_VAR(2);     // #8=N    Shift out variable C to TDI
pub const JTAG_SHIFT_OUT_VARD       : u8 = JTAG_SHIFT_OUT_VAR(3);     // #8=N    Shift out variable D to TDI

pub const fn JTAG_SHIFT_IN_OUT_VAR(x: u8) -> u8 { 56 + x }            // #8=N    Set variable x from TDO, with TDI
pub const JTAG_SHIFT_IN_OUT_VARA    : u8 = JTAG_SHIFT_IN_OUT_VAR(0);  // #8=N    Set variable A from TDO, with TDI
pub const JTAG_SHIFT_IN_OUT_VARB    : u8 = JTAG_SHIFT_IN_OUT_VAR(1);  // #8=N    Set variable B from TDO, with TDI
pub const JTAG_SHIFT_IN_OUT_VARC    : u8 = JTAG_SHIFT_IN_OUT_VAR(2);  // #8=N    Set variable C from TDO, with TDI
pub const JTAG_SHIFT_IN_OUT_VARD    : u8 = JTAG_SHIFT_IN_OUT_VAR(3);  // #8=N    Set variable D from TDO, with TDI

pub const JTAG_SHIFT_OUT_DP         : u8 = 60;                        // #8=N    Shift out N bits, data taken from dataPtr
pub const JTAG_SHIFT_IN_DP          : u8 = 61;                        // #8=N    Shift in N bits
pub const JTAG_SHIFT_IN_OUT_DP      : u8 = 62;                        // #8=N    Shift out & in N bits, data taken from dataPtr

//============================================================================================
pub const JTAG_RESERVED_2           : u8 = (2<<5);

//============================================================================================
// The following quick commands take a fixed operand (N=1-31,0=>32) as part of the opcode
                                                               // Operand
pub const fn JTAG_SHIFT_IN_Q(N: u8)     -> u8 { (3<<5) | (N & JTAG_NUM_BITS_MASK) } // #5=N     Shift in N bits (fill with TDI=0/1)
pub const fn JTAG_SHIFT_OUT_Q(N: u8)    -> u8 { (4<<5) | (N & JTAG_NUM_BITS_MASK) } // #5=N     Shift out N bits (data taken in-line)
pub const fn JTAG_SHIFT_IN_OUT_Q(N: u8) -> u8 { (5<<5) | (N & JTAG_NUM_BITS_MASK) } // #5=N     Shift out & in N bits (data taken in-line)
pub const JTAG_NUM_BITS_MASK          : u8 = 0x1F;                                    // Mask for number of bits (N) within above opcodes

//============================================================================================
// The following quick commands take a count (N=2-31,0=>32, 1=>DP) as part of the opcode or from dataptr
pub const fn JTAG_REPEAT_Q(N: u8)    -> u8 { (6<<5) | (N & JTAG_NUM_BITS_MASK) }  // Repeat a block N times
pub const JTAG_REPEAT_DP              : u8 = JTAG_REPEAT_Q(1);                    // A repeat count of 1 means use 8-bit value from outDataPtr

//============================================================================================
// The following quick command take a value (N=0-31) as part of the opcode
pub const fn JTAG_PUSH_Q(N: u8)      -> u8 { (7<<5) | (N & JTAG_NUM_BITS_MASK) }  // Push a 5-bit value

//============================================================================================
// ARM Specific commands
pub const JTAG_ARM_READAP             : u8 = 64; // #addr (16-bit address A[15:8]=AP#, A[7:4]=Bank#, A[3:2]=Reg# Read value from AP register
pub const JTAG_ARM_WRITEAP            : u8 = 65; // Write input data value to AP register
pub const JTAG_ARM_WRITEAP_I          : u8 = 66; // Write immediate value to AP register

//============================================================================================
pub const JTAG_READ_MEM               : u8 = 68; // Set DSC instruction to execute (from DP)
pub const JTAG_WRITE_MEM              : u8 = 69; // Execute DSC instruction previously set

//============================================================================================
// Common JTAG Commands
pub const JTAG_IDCODE_LENGTH          : u8 = 32;
pub const JTAG_IDCODE_COMMAND         : u8 = 0x02;
pub const JTAG_BYPASS_COMMAND         : u8 = !0x00;

//============================================================================================
// Commands to Master JTAG
pub const JTAG_MASTER_COMMAND_LENGTH  : u8 = 8;
pub const JTAG_TLM_SELECT_COMMAND     : u8 = 0x05;

pub const TLM_REGISTER_LENGTH         : u8 = 4;
pub const TLM_MASTER_SELECT_MASK      : u8 = 0x01;
pub const TLM_SLAVE_SELECT_MASK       : u8 = 0x02;

//============================================================================================
// Command to Core JTAG
pub const JTAG_CORE_COMMAND_LENGTH    : u8 = 4;
pub const CORE_ENABLE_ONCE_COMMAND    : u8 = 0x06;
pub const CORE_DEBUG_REQUEST_COMMAND  : u8 = 0x07;

pub const JTAG_READ_MEMORY_HEADER_SIZE: u16 = 8;
pub const JTAG_WRITE_MEMORY_HEADER_SIZE: u16 = 8;

// Calculate number of bytes required to hold N bits
pub const fn BITS_TO_BYTES(n: u8) -> u8 {(n + 7) >> 3}

fn add_uu(x: u8, y: u8) -> Vec<u8> {
    vec![x, y]
} 

fn add_vu(mut x: Vec<u8>, y: u8) -> Vec<u8> {
    x.push(y);
    x
}


//pub trait JtagInterface {

/// Read id code.
//fn read_id_code(&self, commandRegLength :u8, resetTAP: bool) -> Result<(Vec<u8>), Error>;


//}

#[derive(Debug, Clone,PartialEq)]
pub enum OnceStatus {
    
   ExecuteMode,
   StopMode,
   ExternalAccessMode,
   DebugMode,
   UnknownMode,

}

impl From <u8>  for OnceStatus  {
    fn from(target_status : u8) -> OnceStatus {
      match target_status {
          0x01     => OnceStatus::ExecuteMode,          
          0x05     => OnceStatus::StopMode,          
          0x09     => OnceStatus::ExternalAccessMode,         
          0x0D     => OnceStatus::DebugMode,     
          _        => OnceStatus::UnknownMode,      
      }    
    }
  }
  

    


    // Read IDCODE from JTAG TAP
    //
    // @param idCode   - 32-bit IDCODE returned from TAP
    // @param resetTAP - Optionally resets the TAP to RUN-TEST/IDLE before reading IDCODE
    //                   This will enable the MASTER TAP!
    //
    // @note - resetTAP=true will enable the Master TAP & disable the Code TAP
    // @note - Leaves Core TAP in RUN-TEST/IDLE
    //
    pub fn read_id_code(commandRegLength :u8, resetTAP: bool, prg:  &Programmer) -> Result<(Vec<u8>), Error> {
        let mut sequence: Vec<u8> = Vec::new();
        if resetTAP {
            sequence.push(JTAG_TEST_LOGIC_RESET);
        } else {
            sequence.push(JTAG_NOP);
        }
        sequence.push(JTAG_MOVE_IR_SCAN);  // Write IDCODE command to IR
        sequence.push(JTAG_SET_EXIT_SHIFT_DR);
        sequence.push(JTAG_SHIFT_OUT_Q(commandRegLength)); 
        sequence.push(JTAG_IDCODE_COMMAND);
        sequence.push(JTAG_SET_EXIT_IDLE);  // Read IDCODE from DR
        sequence.push(JTAG_SHIFT_IN_Q(32));
        sequence.push(JTAG_END);

        prg.exec_jtag_seq(sequence, 4)
    }

    pub fn read_master_id_code_DSC_JTAG_ID(resetTAP: bool, prg:  &Programmer) -> Result<(Vec<u8>), Error> {
        read_id_code(JTAG_MASTER_COMMAND_LENGTH, resetTAP, prg)
    }

    pub fn read_core_id_code(resetTAP: bool, prg:  &Programmer) -> Result<(Vec<u8>), Error> {
        read_id_code(JTAG_CORE_COMMAND_LENGTH, resetTAP, prg)
    }

    //  Enable the Core TAP using the TLM
    //
    //  @note - Resets the TAPs before enabling Core TAP
    //  @note - It appears that the sequence must end with a EXIT_SHIFT_DR?
    //  @note Leaves Core TAP in RUN-TEST/IDLE to TLM action??
    pub fn enableCoreTAP(prg:  &Programmer) -> Result<(), Error> {
        let mut sequence: Vec<u8> = Vec::new();
        sequence.push(JTAG_TEST_LOGIC_RESET);               // Reset TAP
        sequence.append(&mut JTAG_REPEAT_16(50)); // ~2.26ms
        sequence.push(JTAG_NOP);
        sequence.push(JTAG_END_REPEAT);
        sequence.push(JTAG_MOVE_IR_SCAN);                   // Write TLM command to IR
        sequence.push(JTAG_SET_EXIT_SHIFT_DR);
        sequence.push(JTAG_SHIFT_OUT_Q(JTAG_MASTER_COMMAND_LENGTH));  
        sequence.push(JTAG_TLM_SELECT_COMMAND);
        sequence.push(JTAG_SET_EXIT_IDLE);                  // Select Core TAP
        sequence.push(JTAG_SHIFT_OUT_Q(TLM_REGISTER_LENGTH)); 
        sequence.push(TLM_SLAVE_SELECT_MASK);
        sequence.push(JTAG_END);
        prg.exec_jtag_seq(sequence, 0)?;
        Ok(())
    }

     /// `enableONCE` in JTAG chain & obtain target status
     ///
     /// `return` OnceStatus from JTAG command
     ///
     /// `brief` Assumes Core TAP is active & in RUN-TEST/IDLE
     /// 
     /// `note` Leaves Core TAP in RUN-TEST/IDLE
     /// ### Usage
     /// 
     /// ```
     /// let once_status = enableONCE(&programmer)?;
     /// 
     /// ```
     /// `EOnCE` = `Enhanced On-chip emulation (unit)`
     pub fn enableONCE(prg:  &Programmer) -> Result<(OnceStatus), Error> {
        let mut sequence: Vec<u8> = Vec::new();
        sequence.push(JTAG_MOVE_IR_SCAN);                // Write enable EONCE command to IR
        sequence.push(JTAG_SET_EXIT_IDLE); 
        sequence.push(JTAG_SHIFT_IN_OUT_Q(JTAG_CORE_COMMAND_LENGTH));
        sequence.push(CORE_ENABLE_ONCE_COMMAND);
        sequence.push(JTAG_END);
        let answer = prg.exec_jtag_seq(sequence, JTAG_CORE_COMMAND_LENGTH)?;
        let once_byte = answer[0];
        Ok((OnceStatus::from(once_byte)))
    }

impl Programmer
{
    /// `targetDebugRequest` set Core JTAG-IR to DebugRequest
    ///
    /// `note` Assumes Core TAP is active & in RUN-TEST/IDLE
    /// 
    /// `note` Leaves Core TAP in RUN-TEST/IDLE
    ///
    /// Private helper function use `dsc_target_halt()` instead
    pub fn targetDebugRequest(&self) -> Result<OnceStatus, Error> {
        let mut sequence: Vec<u8> = Vec::new();
        sequence.push(JTAG_MOVE_IR_SCAN);                // Write enable EONCE command to IR
        sequence.push(JTAG_SET_EXIT_IDLE); 
        sequence.push(JTAG_SHIFT_IN_OUT_Q(JTAG_CORE_COMMAND_LENGTH));
        sequence.push(CORE_DEBUG_REQUEST_COMMAND);
        sequence.push(JTAG_END);
        let answer = self.exec_jtag_seq(sequence, JTAG_CORE_COMMAND_LENGTH)?;
        let once_byte = answer[0]; // TODO need right conversion!!! from 4 byte of answer to one once byte. now empric first byte from debug
        Ok(OnceStatus::from(once_byte))
    }
    
    /// `dsc_target_halt` uses `targetDebugRequest` in cycle to set Core JTAG-IR to DebugRequest
    ///
    /// `note` Assumes Core TAP is active & in RUN-TEST/IDLE
    /// 
    /// `note` Leaves Core TAP in RUN-TEST/IDLE
    pub fn dsc_target_halt(&self) -> Result<OnceStatus, Error> {
        let mut once_status: OnceStatus = OnceStatus::UnknownMode;
        for retry in 0..10 
        {
            self.targetDebugRequest()?;
            once_status = enableONCE(&self)?;
            if(once_status == OnceStatus::DebugMode)
                { 
                    break; 
                }
            if(once_status == OnceStatus::UnknownMode) 
                {   
                    self.dsc_target_go()?;
                    let force_once_status = enableONCE(&self)?;
                    dbg!("Force DSC executive! Once status after is: ", force_once_status);
                }
        }
        if (once_status != OnceStatus::DebugMode) {
            return Err((Error::TargetNotConnected("DSC enter debug mode failed!".to_string())))
        }
        Ok(once_status)
    }

    // Start Target execution at current PC
    //
    // @note Assumes Core TAP is active & in RUN-TEST/IDLE
    // @note Leaves Core TAP in RUN-TEST/IDLE
    /// `dsc_target_go` - set DSC to run, (execute mode) 
    /// 
    /// about `dsc_write_once_reg` : from DSP56800 Family Manual : `Enter the debug processing state if the PWD bit is clear in the EOnCE port’s OCR register, and wait
    ///for EOnCE commands. If this bit is not clear, then the processor simply executes two NOPs and continues
    ///program execution`. `Attention`! Each flash routine, in end of task use self dgb_halt assembly instruction, 
    /// if PWD bit not inited DSC not halt after execution (just executes two NOPs), and it will be undef. bev.! 
    pub fn dsc_target_go(&self) -> Result<(), Error> {

        let mut sequence: Vec<u8> = Vec::new();
        sequence.push(JTAG_MOVE_DR_SCAN);  // Write to ONCE (DR-CHAIN)
        sequence.push(JTAG_SET_EXIT_IDLE);
        sequence.push(JTAG_SHIFT_OUT_Q(ONCE_CMD_LENGTH));  // ONCE command
        sequence.push(ONCE_CMD_EXIT|ONCE_CMD_NOREG);
        sequence.push(JTAG_END);  // Terminate sequence
        self.exec_jtag_seq(sequence, 0)?; // Terminate sequence
        Ok(())
    }

    // Read X/P memory via ONCE & target execution
    //
    // @param memorySpace - Memory space & size of memory accesses 1/2/4 bytes
    // @param numBytes    - Number of bytes to read (must be a multiple of elementSize)
    // @param address     - Memory address
    // @param buffer      - Where to obtain the data
    //
    // @note If memory space size is word or long size then address is DSC word address
    // @note If memory space size is byte size then address is DSC byte pointer address
    // @note Size is limited to dscInfo.maxMemoryReadSize
    //
    /// Private helper function use `dsc_read_memory` instead
    fn read_memory_block(&self, mut memory_space: u8, num_bytes: u8, address: u32) -> Result<(Vec<u8>), Error> {
        if (memory_space == memory_space_t::MS_PLONG) {
            // Treat as word access
            memory_space = memory_space_t::MS_PWORD;
        };

        let mut num_bytes_adjusted = num_bytes;
        match (memory_space & memory_space_t::MS_SIZE) {
            memory_space_t::MS_LONG => {
                if ((address & 0x01) == 0) {
                    num_bytes_adjusted /= 4;
                } else {
                    return Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_ILLEGAL_PARAMS))
                };},
            memory_space_t::MS_WORD => { num_bytes_adjusted /= 2; },
            memory_space_t::MS_BYTE => { num_bytes_adjusted /= 1; },
            other => return Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_ILLEGAL_PARAMS)),
        };

        /*
         *    +-----------------------+
         *    |    JTAG_READ_MEM      |
         *    +-----------------------+
         *    |    JTAG_END           |
         *    +-----------------------+
         *    |                       |
         *    +--                   --+
         *    |                       |
         *    +--  Memory Address   --+
         *    |                       |
         *    +--                  ---+
         *    |                       |
         *    +-----------------------+
         *    |  # of memory elements |
         *    +-----------------------+
         *    |   Memory Space        |
         *    +-----------------------+
         */

        let mut sequence: Vec<u8> = Vec::with_capacity(JTAG_READ_MEMORY_HEADER_SIZE.into());
        sequence.push(JTAG_READ_MEM);          // 0
        sequence.push(JTAG_END);               // 1
        sequence.push((address >> 24) as u8);  // 2 Address
        sequence.push((address >> 16) as u8);  // 3
        sequence.push((address >> 8) as u8);   // 4
        sequence.push(address as u8);          // 5
        sequence.push(num_bytes_adjusted);     // 6 Elements
        sequence.push(memory_space);           // 7 Memory space

        let block = self.exec_jtag_seq(sequence, num_bytes)?;

        Ok(block)
    }


    //================================================================================
    // Read X/P memory via ONCE & target execution
    //
    // @param memorySpace - Memory space & size of memory accesses 1/2/4 bytes
    // @param numBytes    - Number of bytes to read (must be a multiple of elementSize)
    // @param address     - Memory address (byte = byte address, word/long = word address)
    // @param buffer      - Where to obtain the data
    //
    // @note If memory space size is word or long size then address is DSC word address
    // @note If memory space size is byte size then address is DSC byte pointer address
    //
    pub fn dsc_read_memory (&self, memory_space: u8, num_bytes: u32, address: u32) -> Result<(Vec<u8>), Error> {
        let element_size: u8 = memory_space & memory_space_t::MS_SIZE;
        let mut bytes_done: u32 = 0;
        let mut current_address: u32 = address;
        let mut output: Vec<u8> = Vec::new();

        let max_read_size: u32 = 0x20;

        while (bytes_done < num_bytes) {
            let mut block_size: u32 = num_bytes - bytes_done;
            
            if (block_size > max_read_size) {
                block_size = max_read_size; }
            
            let mut data = self.read_memory_block(memory_space, block_size as u8, current_address)?;
            output.append(&mut data);
            bytes_done += block_size;
            if element_size == memory_space_t::MS_BYTE {
                current_address += block_size; // Byte currentAddress advanced by count of bytes written
            } else {
                current_address += block_size / 2; // Address advanced by count of words written
            }
        }
        Ok(output)
    }

    //================================================================================
    // Write X/P memory via ONCE & target execution
    //
    // @param memorySpace - Memory space & size of memory accesses 1/2/4 bytes
    // @param numBytes    - Number of bytes to read (must be a multiple of elementSize)
    // @param address     - Memory address
    // @param buffer      - Where to obtain the data
    //
    // @note If memory space size is word or long size then address is DSC word address
    // @note If memory space size is byte size then address is DSC byte pointer address
    // @note Size is limited to dscInfo.maxMemoryWriteSize
    //
    /// Private helper function use `dsc_write_memory` instead
    fn write_memory_block(&self, mut memory_space: u8, mut data: Vec<u8>, address: u32) -> Result<(), Error> {
        if (memory_space == memory_space_t::MS_PLONG) {
            // Treat as word access
            memory_space = memory_space_t::MS_PWORD;
        };

        if data.len() > u8::MAX.into() {return Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_ILLEGAL_PARAMS))}; 
        let num_bytes:u8 = data.len() as u8;
        let mut num_bytes_adjusted = num_bytes;
        match (memory_space & memory_space_t::MS_SIZE) {
            memory_space_t::MS_LONG => {
                if ((address & 0x01) == 0) {
                    num_bytes_adjusted /= 4;
                } else {
                    return Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_ILLEGAL_PARAMS))
                };},
            memory_space_t::MS_WORD => { num_bytes_adjusted /= 2; },
            memory_space_t::MS_BYTE => { num_bytes_adjusted /= 1; },
            other => return Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_ILLEGAL_PARAMS)),
        };

        /*
         *    +-----------------------+
         *    |    JTAG_WRITE_MEM     |
         *    +-----------------------+
         *    |    JTAG_END           |
         *    +-----------------------+
         *    |                       |
         *    +--                   --+
         *    |                       |
         *    +--  Memory Address   --+
         *    |                       |
         *    +--                  ---+
         *    |                       |
         *    +-----------------------+
         *    |  # of memory elements |
         *    +-----------------------+
         *    |   Memory Space        |
         *    +-----------------------+
         *    | ..................... |
         *    = ...... data ......... =
         *    | ..................... |
         *    +-----------------------+
         */

        let mut sequence: Vec<u8> = Vec::with_capacity(JTAG_READ_MEMORY_HEADER_SIZE.into());
        sequence.push(JTAG_WRITE_MEM);          // 0
        sequence.push(JTAG_END);               // 1
        sequence.push((address >> 24) as u8);  // 2 Address
        sequence.push((address >> 16) as u8);  // 3
        sequence.push((address >> 8) as u8);   // 4
        sequence.push(address as u8);          // 5
        sequence.push(num_bytes_adjusted);     // 6 Elements
        sequence.push(memory_space);           // 7 Memory space
        sequence.append(&mut data);

        self.exec_jtag_seq(sequence, 0)?;
        Ok(())
    }

    //================================================================================
    // Write X/P memory via ONCE & target execution
    //
    // @param memorySpace - Memory space & size of memory accesses 1/2/4 bytes
    // @param numBytes    - Number of bytes to read (must be a multiple of elementSize)
    // @param address     - Memory address
    // @param buffer      - Where to obtain the data
    //
    // @note If memory space size is word or long size then address is DSC word address
    // @note If memory space size is byte size then address is DSC byte pointer address
    //
    pub fn dsc_write_memory(&self, mut memory_space: u8, mut data: Vec<u8>, address: u32) -> Result<(), Error> {
        let mut current_address: u32 = address;
        let element_size: u8 = memory_space & memory_space_t::MS_SIZE;

        let max_write_size: usize = 0x20;

        while (data.len() > 0) {
            let mut block_size = data.len();
            
            if (block_size > max_write_size) {
                block_size = max_write_size; };

            self.write_memory_block(memory_space, data.drain(..block_size).collect(), current_address)?;
            if element_size == memory_space_t::MS_BYTE {
                current_address += block_size as u32; // Byte currentAddress advanced by count of bytes written
            } else {
                current_address += block_size as u32 / 2; // Address advanced by count of words written
            }
        }
     
        Ok(())
    }
}


