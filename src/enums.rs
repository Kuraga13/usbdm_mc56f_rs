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
   pub const  CMD_USBDM_SET_VPP                 : u8  = 42;  // Target Vdd Off but previously set level unchanged
   pub const  CMD_USBDM_JTAG_EXECUTE_SEQUENCE   : u8  = 44;  // Execute sequence of JTAG commands

 }

 
pub mod vpp {
   pub const BDM_TARGET_VPP_OFF      : u8    = 0;     // Target Vpp Off
   pub const BDM_TARGET_VPP_STANDBY  : u8    = 1;     // Target Vpp Standby (Inverter on, Vpp off)
   pub const BDM_TARGET_VPP_ON       : u8    = 2;     // Target Vdd internal 5.0V
   pub const BDM_TARGET_VPP_ERROR    : u8    = 3;     //  Target Vpp ?? WTF?? Why in selection
} 
