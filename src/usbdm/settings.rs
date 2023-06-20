#![allow(unused)]

use std::fmt;
use packed_struct::prelude::*;

/// Options passed to the BDM
#[derive(Debug, PartialEq, Clone)]
pub struct BdmSettings {

    pub target_type:                 TargetType,       // Target type - have to be init twice ))) On SetExtOptions + set target command
    pub target_voltage:              TargetVddSelect,  // Target Vdd (off, 3.3V or 5V) - here you only select target vdd, to power ON need send set vdd command
    pub cycle_vdd_on_reset:          bool,             // Cycle target Power when resetting
    pub cycle_vdd_on_connect:        bool,             // Cycle target Power if connection problems)
    pub leave_target_powered:        bool,             // Leave target power on exit
    pub auto_reconnect:              AutoConnect,      // Automatically re-connect to target (for speed change)
    pub guess_speed:                 bool,             // Guess speed for target w/o ACKN
    pub bdm_clock_source:            BdmClockSource,   // BDM clock source in target
    pub use_reset_signal:            bool,             // Whether to use RESET signal on BDM interface
    pub mask_interrupts:             bool,             // Whether to mask interrupts when  stepping
    pub interface_frequency:         u64,              // JTAG/CFVx/DSC only (kHz), 0 => selected by connection type
    pub use_pst_signals:             bool,             // CFVx, PST Signal monitors
    pub power_off_duration:          u64,              // How long to remove power (ms)
    pub power_on_recovery_interval:  u64,              // How long to wait after power enabled (ms)
    pub reset_duration:              u64,              // How long to assert reset (ms)
    pub reset_release_interval:      u64,              // How long to wait after reset release to release other signals (ms)
    pub reset_recovery_interval:     u64,              // How long to wait after reset sequence completes (ms)

}

///  Target microcontroller types
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
 pub enum TargetType {
    Hc12Hcs12       = 0,       // HC12 or HCS12 target
    Hcs08           = 1,       // HCS08 target
    Rs08            = 2,       // RS08 target
    Cfv1            = 3,       // Coldfire Version 1 target
    Cfvx            = 4,       // Coldfire Version 2,3,4 target
    Jtag            = 5,       // JTAG target - TAP is set to \b RUN-TEST/IDLE
    EzFlash         = 6,       // EzPort Flash interface (SPI?)
    MC56F80xx       = 7,       // JTAG target with MC56F80xx optimised subroutines
    ArmJtag         = 8,       // ARM target using JTAG
    ArmSwd          = 9,       // ARM target using SWD
    Arm             = 10,      // ARM target using either SWD (preferred) or JTAG as supported
    S12Z            = 11,      // S12Z target
    Illegal         = 0xFE,    // Used to indicate error in selecting target
    None            = 0xFF,    // Turn off interface (no target)
 }

/// Internal Target Voltage supply selection
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetVddSelect {
    VddOff       = 0x0,     // Target Vdd Off
    Vdd3V3       = 0x01,     // Target Vdd internal 3.3V
    Vdd5V        = 0x02,     // Target Vdd internal 5.0V
    VddEnable    = 0x10,  // Target Vdd internal at last set level
    VddDisable   = 0x11,  // Target Vdd Off but previously set level unchanged
}



impl std::fmt::Display for TargetVddSelect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TargetVddSelect::VddOff => "Off",
                TargetVddSelect::Vdd3V3 => "3.3v",
                TargetVddSelect::Vdd5V => "5.0v",
                TargetVddSelect::VddEnable => "",
                TargetVddSelect::VddDisable => "",
            }
        )
    }
}




impl From<TargetVddSelect> for u8 {
  fn from(power_selected : TargetVddSelect) -> u8 {
    match power_selected {
        TargetVddSelect::VddOff     => 0,           // Target Vdd Off
        TargetVddSelect::Vdd3V3     => 1,           // Target Vdd internal 3.3V
        TargetVddSelect::Vdd5V      => 2,           // Target Vdd internal 5.0V
        TargetVddSelect::VddEnable  => 0x10,        // Target Vdd internal at last set level
        TargetVddSelect::VddDisable => 0x11,        // Target Vdd Off but previously set level unchanged
    }    
  }
}



/// Auto-reconnect options
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum AutoConnect {
    Never   = 0,  // Only connect explicitly
    Status  = 1,  // Reconnect on USBDM_ReadStatusReg()
    Always  = 2,  // Reconnect before every command
}

/// Target BDM Clock selection
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum BdmClockSource {
    Default    =  0xFF,  // Use default clock selection (don't modify target's reset default)
    AltClk     =  0,     // Force ALT clock (CLKSW = 0)
    NormalClk  =  1,     // Force Normal clock (CLKSW = 1)
}


impl Default for BdmSettings {
    fn default() -> Self { 
        BdmSettings {
            target_type:                 TargetType::None,
            target_voltage:              TargetVddSelect::VddOff,
            cycle_vdd_on_reset:          false,
            cycle_vdd_on_connect:        false,
            leave_target_powered:        false,
            auto_reconnect:              AutoConnect::Always,
            guess_speed:                 false,
            bdm_clock_source:            BdmClockSource::Default,
            use_reset_signal:            false,
            mask_interrupts:             false,
            interface_frequency:         2000,
            use_pst_signals:             false,
            power_off_duration:          500,
            power_on_recovery_interval:  200,
            reset_duration:              250,
            reset_release_interval:      100,
            reset_recovery_interval:     200,
        } 
    }
}

impl BdmSettings {
    pub fn new() -> Self {
        Default::default()
    }
}

impl fmt::Display for BdmSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bdm_settings_default() {
        let a: BdmSettings = Default::default();
        let b = BdmSettings::new();
        assert_eq!(a, b);
    }

    #[test]
    fn test_bad_add() {
        let a: String = format!("{:#?}", BdmSettings::new());
        let b: String = format!("{}", BdmSettings::new());
        assert_eq!(a, b);
    }

}

