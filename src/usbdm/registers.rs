const DSC_FIRST_CORE_REGISTER: u8 = 0;
const DSC_LAST_CORE_REGISTER: u8 = 37;
const DSC_CORE_REGISTER_COUNT: u8 = DSC_LAST_CORE_REGISTER - DSC_FIRST_CORE_REGISTER + 1;

const DSC_FIRST_ONCE_REGISTER: u8 = 39;
const DSC_LAST_ONCE_REGISTER: u8 = 56;
const DSC_ONCE_REGISTER_COUNT: u8 = DSC_LAST_ONCE_REGISTER - DSC_FIRST_ONCE_REGISTER + 1;

// regNo Parameter for DSC_ReadReg() with DSC target
// DSC Core registers
enum DscRegisters {
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
    DscRegsHm01   = 33,
    DscRegsHn     = 34,
    DscRegsHr0    = 35,
    DscRegsHr1    = 36,
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