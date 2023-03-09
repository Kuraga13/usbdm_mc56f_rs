use super::*;

const DSC_FIRST_CORE_REGISTER: u8 = 0;
const DSC_LAST_CORE_REGISTER: u8 = 37;
const DSC_CORE_REGISTER_COUNT: u8 = DSC_LAST_CORE_REGISTER - DSC_FIRST_CORE_REGISTER + 1;

const DSC_FIRST_ONCE_REGISTER: u8 = 39;
const DSC_LAST_ONCE_REGISTER: u8 = 56;
const DSC_ONCE_REGISTER_COUNT: u8 = DSC_LAST_ONCE_REGISTER - DSC_FIRST_ONCE_REGISTER + 1;

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
    Err(Error::Unknown)
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