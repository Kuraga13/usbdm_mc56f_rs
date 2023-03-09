const DSC_FIRST_CORE_REGISTER: u8 = 0;
const DSC_LAST_CORE_REGISTER: u8 = 37;
const DSC_CORE_REGISTER_COUNT: u8 = DSC_LAST_CORE_REGISTER - DSC_FIRST_CORE_REGISTER + 1;

const DSC_FIRST_ONCE_REGISTER: u8 = 39;
const DSC_LAST_ONCE_REGISTER: u8 = 56;
const DSC_ONCE_REGISTER_COUNT: u8 = DSC_LAST_ONCE_REGISTER - DSC_FIRST_ONCE_REGISTER + 1;

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