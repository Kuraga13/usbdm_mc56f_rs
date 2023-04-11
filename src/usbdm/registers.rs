use super::*;

const DSC_FIRST_CORE_REGISTER: u8 = 0;
const DSC_LAST_CORE_REGISTER: u8 = 37;
const DSC_CORE_REGISTER_COUNT: u8 = DSC_LAST_CORE_REGISTER - DSC_FIRST_CORE_REGISTER + 1;

const DSC_FIRST_ONCE_REGISTER: u8 = 39;
const DSC_LAST_ONCE_REGISTER: u8 = 56;
const DSC_ONCE_REGISTER_COUNT: u8 = DSC_LAST_ONCE_REGISTER - DSC_FIRST_ONCE_REGISTER + 1;

// EONCE Command register details
//-------------------------------------------------------------------
pub const ONCE_CMD_LENGTH: u8 = 8;

// The following bit masks may be combined
pub const ONCE_CMD_READ  : u8 = 1<<7;
pub const ONCE_CMD_WRITE : u8 = 0<<7;
pub const ONCE_CMD_GO    : u8 = 1<<6;
pub const ONCE_CMD_EXIT  : u8 = 1<<5;

// Register field - some commonly used regs here
pub const OPDBR_ADDRESS  : u8 = 0x04;
pub const OTX_ADDRESS    : u8 = 0x07;  // tx to target OTX/ORX register
pub const OTX1_ADDRESS   : u8 = 0x09;
pub const ORX_ADDRESS    : u8 = 0x0B;  // rx from target OTX/ORX register
pub const ORX1_ADDRESS   : u8 = 0x0D;
pub const ONCE_CMD_NOREG : u8 = 0x1F;  // used for no register

// EONCE_OCR register details
//--------------------------------------------------------------------
pub const OCR_ERLO            : u8 = 1<<7;
pub const OCR_PWU             : u8 = 1<<5;
pub const OCR_DEVEN           : u8 = 1<<4;
pub const OCR_LTE             : u8 = 1<<3;
pub const OCR_ISC_0           : u8 = 0x00;
pub const OCR_ISC_1           : u8 = 0x01;
pub const OCR_ISC_2           : u8 = 0x02;
pub const OCR_ISC_3           : u8 = 0x03;
pub const OCR_ISC_4           : u8 = 0x04;
pub const OCR_ISC_SINGLE_STEP : u8 = 0x05;
pub const OCR_ISC_6           : u8 = 0x06;
pub const OCR_ISC_7           : u8 = 0x07;

//Aliases for Cached routines
const JTAG_SUB_EXECUTE    : u8 = JTAG_SUBA;       // execute a series of target instructions (firmware implemented)
const JTAG_SUB_MEM_READ   : u8 = JTAG_SUBB;       // read a block from target memory
const JTAG_SUB_MEM_WRITE  : u8 = JTAG_SUBC;       // write a block to target memory
const JTAG_CALL_EXECUTE   : u8 = JTAG_CALL_SUBA;
const JTAG_CALL_MEM_READ  : u8 = JTAG_CALL_SUBB;
const JTAG_CALL_MEM_WRITE : u8 = JTAG_CALL_SUBC;

// regNo Parameter for DSC_ReadReg() with DSC target
// DSC Core registers
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum DscRegisters {
    // Core registers
    DscRegX0      =  0,  // 0
    DscRegY0      =  1,
    DscRegY1      =  2,
    DscRegA0      =  3,
    DscRegA1      =  4,
    DscRegA2      =  5,
    DscRegB0      =  6,
    DscRegB1      =  7,
    DscRegB2      =  8,
    DscRegC0      =  9,
    DscRegC1      = 10,  // 10
    DscRegC2      = 11,
    DscRegD0      = 12,
    DscRegD1      = 13,
    DscRegD2      = 14,
    DscRegOmr     = 15,
    DscRegSr      = 16,
    DscRegLa      = 17,
    DscRegLa2     = 18,  // read only
    DscRegLc      = 19,
    DscRegLc2     = 20,  // 20 read only
    DscRegHws0    = 21,
    DscRegHws1    = 22,
    DscRegSp      = 23,
    DscRegN3      = 24,
    DscRegM01     = 25,
    DscRegN       = 26,
    DscRegR0      = 27,
    DscRegR1      = 28,
    DscRegR2      = 29,
    DscRegR3      = 30,  // 30
    DscRegR4      = 31,
    DscRegR5      = 32,
    DscRegShm01   = 33,
    DscRegShn     = 34,
    DscRegShr0    = 35,
    DscRegShr1    = 36,
    DscRegPc      = 37,
    // JTAG registers
    DscRegIdcode  = 38,  // JTAG Core IDCODE
    // ONCE registers
    DscRegOcr     = 39,  // 39 ONCE Control register
    DscRegOscntr  = 40,  // ONCE Instruction Step Counter
    DscRegOsr     = 41,  // ONCE Status register
    DscRegOpdbr   = 42,  // ONCE Program Data Bus Register
    DscRegObase   = 43,  // ONCE Peripheral Base Address regitsre
    DscRegOtxrxsr = 44,  // ONCE Tx & Rx Status & Control register
    DscRegOtx     = 45,  // ONCE Transmit register (32-bit)
    DscRegOtx1    = 46,  // ONCE Transmit register (16-bit)
    DscRegOrx     = 47,  // ONCE Receive register (32-bit)
    DscRegOrx1    = 48,  // ONCE Receive register (16-bit)
    DscRegOtbcr   = 49,  // ONCE Trace buffer control register
    DscRegOtbpr   = 50,  // ONCE Trace Buffer Pointer register
    DscRegOtb     = 51,  // Trace Buffer Register Stages
    DscRegOb0cr   = 52,  // Breakpoint Unit 0 Control register
    DscRegOb0ar1  = 53,  // Breakpoint Unit 0 Address register 1
    DscRegOb0ar2  = 54,  // Breakpoint Unit 0 Address register 2
    DscRegOb0msk  = 55,  // Breakpoint Unit 0 Mask register
    DscRegOb0Cntr = 56,  // Breakpoint Unit 0 Counter
}

pub struct TargetRegisterDetails {
    name: &'static str,
    width: u8,
}
 
const TARGET_REGISTER_DETAILS: [TargetRegisterDetails; DSC_CORE_REGISTER_COUNT as usize]  = [
    TargetRegisterDetails {name: "x0",    width: 16},   // X0
    TargetRegisterDetails {name: "y0",    width: 16},   // Y0
    TargetRegisterDetails {name: "y1",    width: 16},   // Y1
    TargetRegisterDetails {name: "a0",    width: 16},   // A0
    TargetRegisterDetails {name: "a1",    width: 16},   // A1
    TargetRegisterDetails {name: "a2",    width:  4},   // A2
    TargetRegisterDetails {name: "b0",    width: 16},   // B0
    TargetRegisterDetails {name: "b1",    width: 16},   // B1
    TargetRegisterDetails {name: "b2",    width:  4},   // B2
    TargetRegisterDetails {name: "c0",    width: 16},   // C0
    TargetRegisterDetails {name: "c1",    width: 16},   // C1
    TargetRegisterDetails {name: "c2",    width:  4},   // C2
    TargetRegisterDetails {name: "d0",    width: 16},   // D0
    TargetRegisterDetails {name: "d1",    width: 16},   // D1
    TargetRegisterDetails {name: "d2",    width:  4},   // D2
    TargetRegisterDetails {name: "omr",   width: 16},   // OMR
    TargetRegisterDetails {name: "sr",    width: 16},   // SR
    TargetRegisterDetails {name: "la",    width: 24},   // LA
    TargetRegisterDetails {name: "la2",   width: 24},   // LA2
    TargetRegisterDetails {name: "lc",    width: 16},   // LC
    TargetRegisterDetails {name: "lc2",   width: 16},   // LC2
    TargetRegisterDetails {name: "hws0",  width: 24},   // HWS0
    TargetRegisterDetails {name: "hws1",  width: 24},   // HWS1
    TargetRegisterDetails {name: "sp",    width: 24},   // SP
    TargetRegisterDetails {name: "n3",    width: 16},   // N3
    TargetRegisterDetails {name: "m01",   width: 16},   // M01
    TargetRegisterDetails {name: "n",     width: 24},   // N
    TargetRegisterDetails {name: "r0",    width: 24},   // R0
    TargetRegisterDetails {name: "r1",    width: 24},   // R1
    TargetRegisterDetails {name: "r2",    width: 24},   // R2
    TargetRegisterDetails {name: "r3",    width: 24},   // R3
    TargetRegisterDetails {name: "r4",    width: 24},   // R4
    TargetRegisterDetails {name: "r5",    width: 24},   // R5
    TargetRegisterDetails {name: "shm01", width: 16},   // SHM01
    TargetRegisterDetails {name: "shn",   width: 24},   // SHN
    TargetRegisterDetails {name: "shr0",  width: 24},   // SHR0
    TargetRegisterDetails {name: "shr1",  width: 24},   // SHR1
    TargetRegisterDetails {name: "pc",    width: 21},   // PC
];

pub struct EonceRegisterDetails {
    address: u8,
    length: u8,
    name: &'static str,
}

pub const EONCE_REGISTER_DETAILS: [EonceRegisterDetails; DSC_ONCE_REGISTER_COUNT as usize]  = [
    EonceRegisterDetails {address: 0x01, length:  8, name: "ocr"    },
    EonceRegisterDetails {address: 0x02, length: 24, name: "oscntr" },
    EonceRegisterDetails {address: 0x03, length: 16, name: "osr"    },
    EonceRegisterDetails {address: 0x04, length: 16, name: "opdbr"  }, /* 16, 32, or 48 */
    EonceRegisterDetails {address: 0x05, length:  8, name: "obase"  },
    EonceRegisterDetails {address: 0x06, length:  8, name: "otxrxsr"},
    EonceRegisterDetails {address: 0x07, length: 32, name: "otx"    },
    EonceRegisterDetails {address: 0x09, length: 16, name: "otx1"   },
    EonceRegisterDetails {address: 0x0b, length: 32, name: "orx"    },
    EonceRegisterDetails {address: 0x0d, length: 16, name: "orx1"   },
    EonceRegisterDetails {address: 0x0e, length: 16, name: "otbcr"  },
    EonceRegisterDetails {address: 0x0f, length:  8, name: "otbpr"  },
    EonceRegisterDetails {address: 0x10, length: 21, name: "otb"    },
    EonceRegisterDetails {address: 0x11, length: 24, name: "ob0cr"  },
    EonceRegisterDetails {address: 0x12, length: 24, name: "ob0ar1" },
    EonceRegisterDetails {address: 0x13, length: 32, name: "ob0ar2" },
    EonceRegisterDetails {address: 0x14, length: 32, name: "ob0msk" },
    EonceRegisterDetails {address: 0x15, length: 16, name: "ob0cntr"},
];

// Obtain size of given register in bits
//
// @param regNo - The register number
//
// @return - size of the register in bits
//
pub fn get_register_size(reg: DscRegisters) -> Result<u8, Error> {
   
    if ((reg as u8 >= DSC_FIRST_CORE_REGISTER) && (reg as u8 <= DSC_LAST_CORE_REGISTER)) {
        let reg_index: u8 = reg as u8 - DSC_FIRST_CORE_REGISTER;
        return Ok(TARGET_REGISTER_DETAILS[reg_index as usize].width)
    } 
    else if ((reg as u8 >= DSC_FIRST_ONCE_REGISTER) && (reg as u8 <= DSC_LAST_ONCE_REGISTER)) {
        let reg_index: u8 = reg as u8 - DSC_FIRST_ONCE_REGISTER;
        return Ok(EONCE_REGISTER_DETAILS[reg_index as usize].length)
    }
    else if (reg as u8 == DscRegisters::DscRegIdcode as u8) {
        return Ok(32);
    }
    Err(Error::InternalError("Unexpected error in get_register_size".to_string()))
}

pub struct DscProgSeq {
    instruction_count: u8,
    sequence: &'static[u8],
}

// Register reads
// Each sequence writes the given register to otx/otx1
pub const TARGET_READ_REG_SEQUENCE: [DscProgSeq; DSC_CORE_REGISTER_COUNT as usize] = [
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE7,0x7F,0xD4,0x7C,0xFF,0xFF]},   // X0     move.w  X0,X:>>otx1
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE7,0x7F,0xD5,0x7C,0xFF,0xFF]},   // Y0     move.w  Y0,X:>>otx1
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE7,0x7F,0xD7,0x7C,0xFF,0xFF]},   // Y1     move.w  Y1,X:>>otx1
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE7,0x7F,0xD6,0xFC,0xFF,0xFF]},   // A0     move.w  A0,X:>>otx1
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE7,0x7F,0xD0,0x7C,0xFF,0xFF]},   // A1     move.w  A1,X:>>otx1
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE7,0x7F,0xD4,0xFC,0xFF,0xFF]},   // A2     move.w  A2,X:>>otx1
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE7,0x7F,0xD7,0xFC,0xFF,0xFF]},   // B0     move.w  B0,X:>>otx1
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE7,0x7F,0xD1,0x7C,0xFF,0xFF]},   // B1     move.w  B1,X:>>otx1
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE7,0x7F,0xD5,0xFC,0xFF,0xFF]},   // B2     move.w  B2,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[1,0x7C,0x20,                         // C0     tfr     C,A
                                                  3,0xE7,0x7F,0xD6,0xFC,0xFF,0xFF]},   //        move.w  A0,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[1,0x7C,0x20,                         // C1     tfr     C,A
                                                  3,0xE7,0x7F,0xD0,0x7C,0xFF,0xFF]},   //        move.w  A1,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[1,0x7C,0x20,                         // C2     tfr     C,A
                                                  3,0xE7,0x7F,0xD4,0xFC,0xFF,0xFF]},   //        move.w  A2,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[1,0x7C,0x30,                         // D0     tfr     D,A
                                                  3,0xE7,0x7F,0xD6,0xFC,0xFF,0xFF]},   //        move.w  A0,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[1,0x7C,0x30,                         // D1     tfr     D,A
                                                  3,0xE7,0x7F,0xD0,0x7C,0xFF,0xFF]},   //        move.w  A1,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[1,0x7C,0x30,                         // D2     tfr     D,A
                                                  3,0xE7,0x7F,0xD4,0xFC,0xFF,0xFF]},   //        move.w  A2,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[1,0x86,0x9C,                         // OMR    move.w  OMR,A0
                                                  3,0xE7,0x7F,0xD6,0xFC,0xFF,0xFF]},   //        move.w  A0,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[1,0x86,0x9D,                         // SR     move.w  SR,A0
                                                  3,0xE7,0x7F,0xD6,0xFC,0xFF,0xFF]},   //        move.w  A0,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[1,0x8C,0x1F,                         // LA     moveu.w LA,R4
                                                  3,0xE3,0x7F,0xDC,0x7D,0xFF,0xFF]},   //        move.l  R4,X:>>otx
    DscProgSeq {instruction_count: 2, sequence: &[3,0xE4,0x1C,0xDB,0xAD,0xFF,0xBA,     // LA2    move.l  #$ffbadbad,R4
                                                  3,0xE3,0x7F,0xDC,0x7D,0xFF,0xFF]},   //        move.l  R4,X:>>otx
    DscProgSeq {instruction_count: 2, sequence: &[1,0x8C,0x1E,                         // LC     moveu.w LC,R4
                                                  3,0xE7,0x7F,0xDC,0x7C,0xFF,0xFF]},   //        move.w  R4,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[2,0x87,0x4C,0xAB,0xAD,               // LC2    moveu.w  #$abad,R4
                                                  3,0xE7,0x7F,0xDC,0x7C,0xFF,0xFF]},   //        move.w  R4,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[3,0xE4,0x1C,0xDB,0xAD,0xFF,0xBA,     // HWS0   move.l  #$ffbadbad,R4
                                                  3,0xE3,0x7F,0xDC,0x7D,0xFF,0xFF]},   //        move.l  R4,X:>>otx
    DscProgSeq {instruction_count: 2, sequence: &[3,0xE4,0x1C,0xDB,0xAD,0xFF,0xBA,     // HWS1   move.l  #$ffbadbad,R4
                                                  3,0xE3,0x7F,0xDC,0x7D,0xFF,0xFF]},   //        move.l  R4,X:>>otx
    DscProgSeq {instruction_count: 2, sequence: &[1,0x81,0xBC,                         // SP     tfra    SP,R4
                                                  3,0xE3,0x7F,0xDC,0x7D,0xFF,0xFF]},   //        move.l  R4,X:>>otx
    DscProgSeq {instruction_count: 2, sequence: &[1,0x86,0x99,                         // N3     move.w  N3,A0
                                                  3,0xE7,0x7F,0xD6,0xFC,0xFF,0xFF]},   //        move.w  A0,X:>>otx1
    DscProgSeq {instruction_count: 2, sequence: &[1,0x86,0x9A,                         // M01    move.w  M01,A0
                                                  3,0xE7,0x7F,0xD6,0xFC,0xFF,0xFF]},   //        move.w  A0,X:>>otx1
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE3,0x7F,0xDE,0x7D,0xFF,0xFF]},   // N      move.l  N,X:>>otx
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE3,0x7F,0xD8,0x7D,0xFF,0xFF]},   // R0     move.l  R0,X:>>otx
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE3,0x7F,0xD9,0x7D,0xFF,0xFF]},   // R1     move.l  R1,X:>>otx
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE3,0x7F,0xDA,0x7D,0xFF,0xFF]},   // R2     move.l  R2,X:>>otx
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE3,0x7F,0xDB,0x7D,0xFF,0xFF]},   // R3     move.l  R3,X:>>otx
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE3,0x7F,0xDC,0x7D,0xFF,0xFF]},   // R4     move.l  R4,X:>>otx
    DscProgSeq {instruction_count: 1, sequence: &[3,0xE3,0x7F,0xDD,0x7D,0xFF,0xFF]},   // R5     move.l  R5,X:>>otx
    DscProgSeq {instruction_count: 4, sequence: &[1,0xE7,0x06,                         // SHM01  swap    shadows
                                                  1,0x86,0x9A,                         //        move.w  M01,A0
                                                  3,0xE7,0x7F,0xD6,0xFC,0xFF,0xFF,     //        move.w  A0,X:>>otx1
                                                  1,0xE7,0x06]},                       //        swap    shadows
    DscProgSeq {instruction_count: 5, sequence: &[1,0xE7,0x06,                         // SHN    swap    shadows
                                                  3,0xE3,0x7F,0xDE,0x7D,0xFF,0xFF,     //        move.l  N,X:>>otx
                                                  1,0xE7,0x00,                         //        nop
                                                  1,0xE7,0x00,                         //        nop
                                                  1,0xE7,0x06]},                       //        swap    shadows
    DscProgSeq {instruction_count: 5, sequence: &[1,0xE7,0x06,                         // SHR0   swap    shadows
                                                  3,0xE3,0x7F,0xD8,0x7D,0xFF,0xFF,     //        move.l  R0,X:>>otx
                                                  1,0xE7,0x00,                         //        nop
                                                  1,0xE7,0x00,                         //        nop
                                                  1,0xE7,0x06]},                       //        swap    shadows
    DscProgSeq {instruction_count: 5, sequence: &[1,0xE7,0x06,                         // SHR1   swap    shadows
                                                  3,0xE3,0x7F,0xD9,0x7D,0xFF,0xFF,     //        move.l  R1,X:>>otx
                                                  1,0xE7,0x00,                         //        nop
                                                  1,0xE7,0x00,                         //        nop
                                                  1,0xE7,0x06]},                       //        swap    shadows
    DscProgSeq {instruction_count: 2, sequence: &[1,0xE7,0x16,                         // PC     move.l  PC,R4  // must be R4!
                                                  3,0xE3,0x7F,0xDC,0x7D,0xFF,0xFF]},   //        move.l  R4,X:>>otx
];

// Register Writes
// These write a value in R4 into the given register
pub const TARGET_WRITE_REG_SEQUENCE: [DscProgSeq; DSC_CORE_REGISTER_COUNT as usize] = [
    DscProgSeq {instruction_count: 1, sequence: &[1,0x84,0x0C,]},     // X0    move.w  R4,X0
    DscProgSeq {instruction_count: 1, sequence: &[1,0x85,0x0C,]},     // X1    move.w  R4,Y0
    DscProgSeq {instruction_count: 1, sequence: &[1,0x87,0x0C,]},     // X2    move.w  R4,Y1
    DscProgSeq {instruction_count: 1, sequence: &[1,0x86,0x8C,]},     // A0    move.w  R4,A0
    DscProgSeq {instruction_count: 1, sequence: &[1,0x80,0x8C,]},     // A1    move.w  R4,A1
    DscProgSeq {instruction_count: 1, sequence: &[1,0x84,0x8C,]},     // A2    move.w  R4,A2
    DscProgSeq {instruction_count: 1, sequence: &[1,0x87,0x8C,]},     // B0    move.w  R4,B0
    DscProgSeq {instruction_count: 1, sequence: &[1,0x81,0x8C,]},     // B1    move.w  R4,B1
    DscProgSeq {instruction_count: 1, sequence: &[1,0x85,0x8C,]},     // B2    move.w  R4,B2
    DscProgSeq {instruction_count: 3, sequence: &[1,0x7C,0x20,        // C0    tfr     C,A
                                                  1,0x86,0x8C,        //       move.w  R4,A0
                                                  1,0x7D,0x00,]},     //       tfr     A,C
    DscProgSeq {instruction_count: 3, sequence: &[1,0x7C,0x20,        // C1    tfr     C,A
                                                  1,0x80,0x8C,        //       move.w  R4,A1
                                                  1,0x7D,0x00,]},     //       tfr     A,C
    DscProgSeq {instruction_count: 3, sequence: &[1,0x7C,0x20,        // C2    tfr     C,A
                                                  1,0x84,0x8C,        //       move.w  R4,A2
                                                  1,0x7D,0x00,]},     //       tfr     A,C
    DscProgSeq {instruction_count: 3, sequence: &[1,0x7C,0x30,        // D0    tfr     D,A
                                                  1,0x86,0x8C,        //       move.w  R4,A0
                                                  1,0x7D,0x80,]},     //       tfr     A,D
    DscProgSeq {instruction_count: 3, sequence: &[1,0x7C,0x30,        // D1    tfr     D,A
                                                  1,0x80,0x8C,        //       move.w  R4,A1
                                                  1,0x7D,0x80,]},     //       tfr     A,D
    DscProgSeq {instruction_count: 3, sequence: &[1,0x7C,0x30,        // D2    tfr     D,A
                                                  1,0x84,0x8C,        //       move.w  R4,A2
                                                  1,0x7D,0x80,]},     //       tfr     A,D
    DscProgSeq {instruction_count: 1, sequence: &[1,0x8C,0x8C,]},     // OMR   moveu.w R4,OMR
    DscProgSeq {instruction_count: 3, sequence: &[1,0x8D,0x8C,        // SR    moveu.w R4,SR
                                                  1,0xE7,0x00,        //       nop
                                                  1,0xE7,0x00,]},     //       nop
    DscProgSeq {instruction_count: 1, sequence: &[1,0x8F,0x8C,]},     // LA    moveu.w R4,LA
    DscProgSeq {instruction_count: 1, sequence: &[1,0xE7,0x00,]},     // LA2   nop
    DscProgSeq {instruction_count: 1, sequence: &[1,0x8E,0x8C,]},     // LC    moveu.w R4,LC
    DscProgSeq {instruction_count: 1, sequence: &[1,0xE7,0x00,]},     // LC2   nop
    DscProgSeq {instruction_count: 1, sequence: &[1,0xE7,0x00,]},     // HWS0  nop
    DscProgSeq {instruction_count: 1, sequence: &[1,0xE7,0x00,]},     // HWS1  nop
    DscProgSeq {instruction_count: 1, sequence: &[1,0x81,0xAB,]},     // SP    tfra    R4,SP
    DscProgSeq {instruction_count: 1, sequence: &[1,0x89,0x8C,]},     // N3    moveu.w R4,N3
    DscProgSeq {instruction_count: 1, sequence: &[1,0x8A,0x8C,]},     // M01   moveu.w R4,M01
    DscProgSeq {instruction_count: 1, sequence: &[1,0x81,0xAA,]},     // N     tfra    R4,N
    DscProgSeq {instruction_count: 1, sequence: &[1,0x81,0xA0,]},     // R0    tfra    R4,R0
    DscProgSeq {instruction_count: 1, sequence: &[1,0x81,0xA1,]},     // R1    tfra    R4,R1
    DscProgSeq {instruction_count: 1, sequence: &[1,0x81,0xA2,]},     // R2    tfra    R4,R2
    DscProgSeq {instruction_count: 1, sequence: &[1,0x81,0xA3,]},     // R3    tfra    R4,R3
    DscProgSeq {instruction_count: 1, sequence: &[1,0xE7,0x00,]},     // R4    nop
    DscProgSeq {instruction_count: 1, sequence: &[1,0x81,0xA9,]},     // R5    tfra    R4,R5
    DscProgSeq {instruction_count: 5, sequence: &[1,0xE7,0x06,        // SHM01 swap    shadows
                                                  1,0x8A,0x8C,        //       moveu.w R4,M01
                                                  1,0xE7,0x00,        //       nop
                                                  1,0xE7,0x00,        //       nop
                                                  1,0xE7,0x06,]},     //       swap    shadows
    DscProgSeq {instruction_count: 5, sequence: &[1,0xE7,0x06,        // SHN   swap    shadows
                                                  1,0x81,0xAA,        //       tfra    R4,N
                                                  1,0xE7,0x00,        //       nop
                                                  1,0xE7,0x00,        //       nop
                                                  1,0xE7,0x06,]},     //       swap    shadows
    DscProgSeq {instruction_count: 5, sequence: &[1,0xE7,0x06,        // SHR0  swap    shadows
                                                  1,0x81,0xA0,        //       tfra    R4,R0
                                                  1,0xE7,0x00,        //       nop
                                                  1,0xE7,0x00,        //       nop
                                                  1,0xE7,0x06,]},     //       swap    shadows
    DscProgSeq {instruction_count: 5, sequence: &[1,0xE7,0x06,        // SHR1  swap    shadows
                                                  1,0x81,0xA1,        //       tfra    R4,R1
                                                  1,0xE7,0x00,        //       nop
                                                  1,0xE7,0x00,        //       nop
                                                  1,0xE7,0x06,]},     //       swap    shadows
    DscProgSeq {instruction_count: 1, sequence: &[1,0xE7,0x17,]},     // PC    move.l  R4,PC
];

fn read_core_reg_sequence(reg: DscRegisters) -> Result<Vec<u8>, Error> {
    if (reg as u8) < DSC_FIRST_CORE_REGISTER || (reg as u8) > DSC_LAST_CORE_REGISTER {
        return Err(Error::InternalError("Unexpected input value in read_core_reg_sequence".to_string()))
    }
    let reg_index: u8 = reg as u8 - DSC_FIRST_CORE_REGISTER;
    let reg_size: u8 = get_register_size(reg)?;
    let instruction_count: u8 = TARGET_READ_REG_SEQUENCE[reg_index as usize].instruction_count;
    let sequence: &[u8] = TARGET_READ_REG_SEQUENCE[reg_index as usize].sequence;
    let mut output: Vec<u8> = vec![];

    output.push(instruction_count); // # of instructions
    output.extend_from_slice(sequence); // target instructions to write the given register to otx/otx1
    
    if reg_size > 16 {
        output.push(ONCE_CMD_READ|OTX_ADDRESS);
    } else {
        output.push(ONCE_CMD_READ|OTX1_ADDRESS);
    }
    output.push(reg_size);
    
    Ok(output)
}

fn write_core_reg_sequence(reg: DscRegisters, value: u32) -> Result<Vec<u8>, Error> {
    if (reg as u8) < DSC_FIRST_CORE_REGISTER || (reg as u8) > DSC_LAST_CORE_REGISTER {
        return Err(Error::InternalError("Unexpected input value in write_core_reg_sequence".to_string()))
    }
    let reg_index: u8 = reg as u8 - DSC_FIRST_CORE_REGISTER;
    let instruction_count: u8 = TARGET_WRITE_REG_SEQUENCE[reg_index as usize].instruction_count;
    let sequence: &[u8] = TARGET_WRITE_REG_SEQUENCE[reg_index as usize].sequence;
    let mut value: u32 = value;
    let mut output: Vec<u8> = vec![];

    if (value &  0xFF800000) != 0 {
        value |= 0xFF000000;
    }

    output.push(instruction_count + 1); // # of instructions (+1 for MOVE #dd,R4)

    // Target code to load 32-bit value into R4
    output.push(3);      // 3 words long
    output.push(0xE4);   // opcode = move #<value>,R4
    output.push(0x1C);
     
    // V - value to load into register
    output.push((value >> 8) as u8);  // Immediate value
    output.push( value as u8);
    output.push((value >> 24) as u8);
    output.push((value >> 16) as u8);

    // I - Target instruction(s) to transfer value to Reg
    output.extend_from_slice(sequence);

    Ok(output)
}


impl Programmer
{
    // Read Core register via OnCE & target execution
    //
    // @param regNo     - Register number
    // @param regValue  - Value for register
    //
    // @note Assumes Core TAP is active & in RUN-TEST/IDLE
    // @note Leaves Core TAP in RUN-TEST/IDLE, EONCE register selected
    //
    pub fn dsc_read_core_reg(&self, reg: DscRegisters) -> Result<u32, Error> {
        if (reg as u8) < DSC_FIRST_CORE_REGISTER || (reg as u8) > DSC_LAST_CORE_REGISTER {
            return Err(Error::InternalError("Unexpected input value in read_core_reg".to_string())) 
        }
        let mut sequence: Vec<u8> = vec![];

        // Execute target instruction to transfer register to memory-mapped EONCE reg OTX
        // Read OTX/OTX1
        // Main
        sequence.push(JTAG_CALL_EXECUTE);                                 // Execute target instruction: move Reg -> OTX/OTX1
        // Read EONCE reg OTX/OTX1
        sequence.push(JTAG_MOVE_DR_SCAN);                                  // Move to SCAN-DR (EONCE)
        sequence.push(JTAG_SET_EXIT_SHIFT_DR);
        sequence.push(JTAG_SHIFT_OUT_DP); sequence.push(ONCE_CMD_LENGTH); // Command for Read Register - either OTX/OTX1
        sequence.push(JTAG_SET_EXIT_IDLE);
        sequence.push(JTAG_SHIFT_IN_DP); sequence.push(0);                // Data size to read
        sequence.push(JTAG_END);
    
        sequence.append(&mut read_core_reg_sequence(reg)?);

        let answer_length = BITS_TO_BYTES(get_register_size(reg)?);
        let answer_vec = self.exec_jtag_seq(sequence, answer_length)?;

        if answer_vec.len() > 4 {return Err(Error::InternalError("Answer too long in read_core_reg".to_string()))}
        if answer_vec.len() == 0 {return Err(Error::InternalError("No answer in read_core_reg".to_string()))}
    
        let mut answer: u32 = 0;
        for i in 0..answer_vec.len(){ 
            answer = (answer << 8) | answer_vec[i] as u32; 
        }
    
        Ok(answer)
    }

    /// `dsc_write_core_reg` write Core register via ONCE & target execution
    ///
    /// `regNo`     - Register number
    /// 
    /// `regValue`  - Value for register
    ///
    /// `note` Assumes Core TAP is active & in RUN-TEST/IDLE
    /// 
    /// `note` Leaves Core TAP in RUN-TEST/IDLE, EONCE register selected
    ///
    pub fn dsc_write_core_reg(&self, reg: DscRegisters, value: u32) -> Result<(), Error> {
        if (reg as u8) < DSC_FIRST_CORE_REGISTER || (reg as u8) > DSC_LAST_CORE_REGISTER {
            return Err(Error::InternalError("Unexpected input value in write_core_reg".to_string())) 
        }
        
        // Execute target instructions to load register
        let mut sequence: Vec<u8> = vec![];
        sequence.push(JTAG_CALL_EXECUTE);  // Execute instructions routine
        sequence.push(JTAG_END);

        sequence.append(&mut write_core_reg_sequence(reg, value)?);
 
        self.exec_jtag_seq(sequence, 0)?;

        Ok(())
    }

    pub fn dsc_read_pc(&self) -> Result<u32, Error>{
        self.dsc_read_core_reg(DscRegisters::DscRegPc)
    }

    pub fn dsc_write_pc(&self, value: u32) -> Result<(), Error>  {
        self.dsc_write_core_reg(DscRegisters::DscRegPc, value)
    }


    // Read ONCE register
    //
    // @param regNo    - Register number
    // @param regValue - Value from register
    //
    // @note Assumes Core TAP is active & in RUN-TEST/IDLE
    // @note Leaves Core TAP in RUN-TEST/IDLE
    //
    pub fn dsc_read_once_reg(&self, reg: DscRegisters) -> Result<u32, Error> {
        if (reg as u8) < DSC_FIRST_ONCE_REGISTER || (reg as u8) > DSC_LAST_ONCE_REGISTER {
            return Err(Error::InternalError("Unexpected input value in dsc_read_once_reg".to_string())) 
        }
        
        let reg_index: u8 = reg as u8 - DSC_FIRST_ONCE_REGISTER;
        let command: u8 = EONCE_REGISTER_DETAILS[reg_index as usize].address | ONCE_CMD_READ;
        let length: u8 = EONCE_REGISTER_DETAILS[reg_index as usize].length;
        let answer_length: u8 = BITS_TO_BYTES(length);

        let mut sequence: Vec<u8> = vec![];
        sequence.push(JTAG_MOVE_DR_SCAN);  // Access ONCE (DR-CHAIN)
        sequence.push(JTAG_SET_EXIT_SHIFT_DR);
        sequence.push(JTAG_SHIFT_OUT_Q(ONCE_CMD_LENGTH)); sequence.push(command);  // ONCE Command to Read register + RegNo
        sequence.push(JTAG_SET_EXIT_IDLE);
        sequence.push(JTAG_SHIFT_IN_Q(length));  // Shift-in data value
        sequence.push(JTAG_END);

        let answer_vec = self.exec_jtag_seq(sequence, answer_length)?;

        if answer_vec.len() > 4 {return Err(Error::InternalError("Answer too long in read_eonce_reg".to_string()))}
        if answer_vec.len() == 0 {return Err(Error::InternalError("No answer in read_eonce_reg".to_string()))}
    
        let mut answer: u32 = 0;
        for i in 0..answer_vec.len(){ 
            answer = (answer << 8) | answer_vec[i] as u32; 
        }
    
        Ok(answer)
    }

    // Write ONCE register
    //
    // @param regNo     - Register number
    // @param modifiers - Modifier for command byte to ONCE register
    // @param regValue  - Value for register
    //
    // @note Assumes Core TAP is active & in RUN-TEST/IDLE
    // @note Leaves Core TAP in RUN-TEST/IDLE
    //
    pub fn dsc_write_once_reg(&self, reg: DscRegisters, value: u32) -> Result<(), Error> {
        if (reg as u8) < DSC_FIRST_ONCE_REGISTER || (reg as u8) > DSC_LAST_ONCE_REGISTER {
            return Err(Error::InternalError("Unexpected input value in dsc_write_once_reg".to_string())) 
        }

        let reg_index: u8 = reg as u8 - DSC_FIRST_ONCE_REGISTER;
        let command: u8 = EONCE_REGISTER_DETAILS[reg_index as usize].address | ONCE_CMD_WRITE;
        let length: u8 = EONCE_REGISTER_DETAILS[reg_index as usize].length;

        let mut sequence: Vec<u8> = vec![];
        sequence.push(JTAG_MOVE_DR_SCAN);            // Write to ONCE (DR-CHAIN)
        sequence.push(JTAG_SET_EXIT_SHIFT_DR);
        sequence.push(JTAG_SHIFT_OUT_Q(ONCE_CMD_LENGTH)); sequence.push(command); // ONCE command - Write register+RegNo+modifier
        sequence.push(JTAG_SET_EXIT_IDLE);
        sequence.push(JTAG_SHIFT_OUT_Q(length));     // Shift-out data value
        sequence.push (value as u8);  // Immediate value
        sequence.push((value >>  8) as u8);
        sequence.push((value >> 16) as u8);
        sequence.push((value >> 24) as u8);
        sequence.push(JTAG_END);

        self.exec_jtag_seq(sequence, 0)?;

        Ok(())
 }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_register_size_test() {
        assert_eq!(get_register_size(DscRegisters::DscRegX0     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegY0     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegY1     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegA0     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegA1     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegA2     ).unwrap(),  4);
        assert_eq!(get_register_size(DscRegisters::DscRegB0     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegB1     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegB2     ).unwrap(),  4);
        assert_eq!(get_register_size(DscRegisters::DscRegC0     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegC1     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegC2     ).unwrap(),  4);
        assert_eq!(get_register_size(DscRegisters::DscRegD0     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegD1     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegD2     ).unwrap(),  4);
        assert_eq!(get_register_size(DscRegisters::DscRegOmr    ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegSr     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegLa     ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegLa2    ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegLc     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegLc2    ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegHws0   ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegHws1   ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegSp     ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegN3     ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegM01    ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegN      ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegR0     ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegR1     ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegR2     ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegR3     ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegR4     ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegR5     ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegShm01  ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegShn    ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegShr0   ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegShr1   ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegPc     ).unwrap(), 21);
        assert_eq!(get_register_size(DscRegisters::DscRegOcr    ).unwrap(),  8);
        assert_eq!(get_register_size(DscRegisters::DscRegOscntr ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegOsr    ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegOpdbr  ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegObase  ).unwrap(),  8);
        assert_eq!(get_register_size(DscRegisters::DscRegOtxrxsr).unwrap(),  8);
        assert_eq!(get_register_size(DscRegisters::DscRegOtx    ).unwrap(), 32);
        assert_eq!(get_register_size(DscRegisters::DscRegOtx1   ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegOrx    ).unwrap(), 32);
        assert_eq!(get_register_size(DscRegisters::DscRegOrx1   ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegOtbcr  ).unwrap(), 16);
        assert_eq!(get_register_size(DscRegisters::DscRegOtbpr  ).unwrap(),  8);
        assert_eq!(get_register_size(DscRegisters::DscRegOtb    ).unwrap(), 21);
        assert_eq!(get_register_size(DscRegisters::DscRegOb0cr  ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegOb0ar1 ).unwrap(), 24);
        assert_eq!(get_register_size(DscRegisters::DscRegOb0ar2 ).unwrap(), 32);
        assert_eq!(get_register_size(DscRegisters::DscRegOb0msk ).unwrap(), 32);
        assert_eq!(get_register_size(DscRegisters::DscRegOb0Cntr).unwrap(), 16);

    }
}