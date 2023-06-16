#![allow(unused)]

pub mod bdm_commands {
    // Common to all targets
    pub const  CMD_USBDM_GET_COMMAND_RESPONSE    : u8  = 0;   // Status of last/current command
    pub const  CMD_USBDM_SET_TARGET              : u8  = 1; 
    pub const  CMD_USBDM_SET_VDD                 : u8  = 2; 
    pub const  CMD_USBDM_GET_BDM_STATUS          : u8  = 4;   // Status of last/current command
    pub const  CMD_USBDM_GET_CAPABILITIES        : u8  = 5;
    pub const  CMD_USBDM_SET_OPTIONS             : u8  = 6;
    pub const  CMD_USBDM_CONTROL_PINS            : u8  = 8;   // Directly control BDM interface levels
    pub const  CMD_USBDM_GET_VER                 : u8  = 12; 
    pub const  CMD_USBDM_SET_SPEED               : u8  = 16; 
    pub const  CMD_USBDM_JTAG_GOTORESET          : u8  = 38;  // Reset JTAG Tap controller
    pub const  CMD_USBDM_JTAG_GOTOSHIFT          : u8  = 39;  // Move JTAG TAP controller to SHIFT-IR/DR
    pub const  CMD_USBDM_JTAG_WRITE              : u8  = 40;  // Write to JTAG chain
    pub const  CMD_USBDM_JTAG_READ               : u8 =  41;  // Read from JTAG chain
    pub const  CMD_USBDM_SET_VPP                 : u8  = 42;  // Target Vdd Off but previously set level unchanged
    pub const  CMD_USBDM_JTAG_EXECUTE_SEQUENCE   : u8  = 44;  // Execute sequence of JTAG commands
    
}

 
pub mod vpp {
    pub const BDM_TARGET_VPP_OFF      : u8    = 0;     // Target Vpp Off
    pub const BDM_TARGET_VPP_STANDBY  : u8    = 1;     // Target Vpp Standby (Inverter on, Vpp off)
    pub const BDM_TARGET_VPP_ON       : u8    = 2;     // Target Vdd internal 5.0V
    pub const BDM_TARGET_VPP_ERROR    : u8    = 3;     //  Target Vpp ?? WTF?? Why in selection
} 

pub mod memory_space_t {
    // Memory space indicator - includes element size
    // One of the following
    pub const MS_BYTE    : u8  = 1;        // Byte (8-bit) access
    pub const MS_WORD    : u8  = 2;        // Word (16-bit) access
    pub const MS_LONG    : u8  = 4;        // Long (32-bit) access
    // One of the following
    pub const MS_NONE    : u8  = 0<<4;     // Memory space unused/undifferentiated
    pub const MS_PROGRAM : u8  = 1<<4;     // Program memory space (e.g. P: on DSC)
    pub const MS_DATA    : u8  = 2<<4;     // Data memory space (e.g. X: on DSC)
    pub const MS_GLOBAL  : u8  = 3<<4;     // HCS12 Global addresses (Using BDMPPR register)
    // Fast memory access for HCS08/HCS12 (stopped target, regs. are modified
    pub const MS_FAST    : u8  = 1<<7;
    // Masks for above
    pub const MS_SIZE    : u8  = 0x7<<0;   // Size
    pub const MS_SPACE   : u8  = 0x7<<4;   // Memory space
    // For convenience (DSC)
    pub const MS_PWORD   : u8  = MS_WORD + MS_PROGRAM;
    pub const MS_PLONG   : u8  = MS_LONG + MS_PROGRAM;
    pub const MS_XBYTE   : u8  = MS_BYTE + MS_DATA;
    pub const MS_XWORD   : u8  = MS_WORD + MS_DATA;
    pub const MS_XLONG   : u8  = MS_LONG + MS_DATA;
}

pub mod jtag_shift {

    pub const JTAG_SHIFT_DR                      : u8  = 0;     // Enter SHIFT-DR (from TEST-LOGIC-RESET or RUN-TEST/IDLE)
    pub const JTAG_SHIFT_IR                      : u8  = 1;     // Enter SHIFT-IR (from TEST-LOGIC-RESET or RUN-TEST/IDLE)
    pub const JTAG_EXIT_IDLE                     : u8  = 1;     // Exit SHIFT-XX to RUN-TEST/IDLE
    pub const JTAG_EXIT_SHIFT_DR                 : u8  = 2;     // Exit SHIFT-XX & enter SHIFT-DR w/o crossing RUN-TEST/IDLE


}