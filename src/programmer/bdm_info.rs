use super::*;
use super::jtag::*;
use crate::errors::{Error, USBDM_ErrorCode};
use std::fmt;
use std::time::Duration;

///`BdmInfo`
///The idea is to group a huge number of USBDM structures, enumerations and settings into three abstractions.
/// 
/// One is BdmInfo - It includes all data information about USBDM, software and hardware versions, buffer sizes
#[derive(Debug, PartialEq)]
pub struct BdmInfo {
    pub bdm_software_version      : u32,           // Version of USBDM Firmware
    pub bdm_hardware_version      : u8,            // Version of USBDM Hardware
    pub icp_software_version      : u8,            // Version of ICP bootloader Firmware
    pub icp_hardware_version      : u8,            // Version of Hardware (reported by ICP code)
    pub capabilities              : Capabilities,  // BDM Capabilities
    pub command_buffer_size       : u16,           // Size of BDM Communication buffer
    pub jtag_buffer_size          : u16,           // Size of JTAG buffer (if supported)
    pub dsc_max_memory_read_size  : u16,
    pub dsc_max_memory_write_size : u16,
}

impl Default for BdmInfo {
    fn default() -> Self { 
        BdmInfo {
            bdm_software_version      : 0,
            bdm_hardware_version      : 0,
            icp_software_version      : 0,
            icp_hardware_version      : 0,
            capabilities              : Capabilities::default(),
            command_buffer_size       : 100,
            jtag_buffer_size          : 100,
            dsc_max_memory_read_size  : 0,
            dsc_max_memory_write_size : 0,

        }
    } 
}

#[derive(Debug, PartialEq)]
pub struct Capabilities {
    pub hcs12:       bool,  // Supports HCS12
    pub rs08:        bool,  // 12 V Flash programming supply available (RS08 support)
    pub vddcontrol:  bool,  // Control over target Vdd
    pub vddsense:    bool,  // Sensing of target Vdd
    pub cfv_x:       bool,  // Support for CFV 1,2 & 3
    pub hcs08:       bool,  // Supports HCS08 targets - inverted when queried
    pub cfv1:        bool,  // Supports CFV1 targets  - inverted when queried
    pub jtag:        bool,  // Supports JTAG targets
    pub dsc:         bool,  // Supports DSC targets
    pub arm_jtag:    bool,  // Supports ARM targets via JTAG
    pub rst:         bool,  // Control & sensing of RESET
    pub pst:         bool,  // Supports PST signal sensing
    pub cdc:         bool,  // Supports CDC Serial over USB interface
    pub arm_swd:     bool,  // Supports ARM targets via SWD
    pub s12z:        bool,  // Supports HCS12Z targets via SWD
}

impl Default for Capabilities {
    fn default() -> Self { 
        Capabilities {
            hcs12:       false,
            rs08:        false,
            vddcontrol:  false,
            vddsense:    false,
            cfv_x:       false,
            hcs08:       false,
            cfv1:        false,
            jtag:        false,
            dsc:         false,
            arm_jtag:    false,
            rst:         false,
            pst:         false,
            cdc:         false,
            arm_swd:     false,
            s12z:        false,
        }
    } 
}

impl Programmer {
    pub fn get_bdm_info(&mut self) -> Result<(), Error> {
        self.get_bdm_version()?;
        self.get_bdm_capabilities()?;
        Ok(())
    }

    fn get_bdm_version(&mut self) -> Result<(), Error>{
        let request_type = 64; //LIBUSB_REQUEST_TYPE_VENDOR
        let request_type = request_type| &self.usb_device.read_ep;
        
        let request  = bdm_commands::CMD_USBDM_GET_VER; // command
        let value    = 100;
        let index    = 0;
        let timeout  = Duration::from_millis(2500);
        let mut usb_buf  = [0; 10];
         
        let version = self.usb_device.control_transfer(
            request_type,
            request,
            value,
            index,
            &mut usb_buf,
            timeout)?;                                    
    
        let raw_bdm_software_version = u32::from (version[1]);
        let calculation = ((raw_bdm_software_version&0xF0)<<12) + ((raw_bdm_software_version&0x0F)<<8);
    
        self.bdm_info.bdm_software_version = calculation;
        self.bdm_info.bdm_hardware_version  = version[2];
        self.bdm_info.icp_software_version  = version[3];
        self.bdm_info.icp_hardware_version  = version[4];
        Ok(())
    }

    fn get_bdm_capabilities(&mut self) -> Result<(), Error>{
        let mut usb_buf = [0; 2];
        usb_buf[0] = 2;  // lenght
        usb_buf[1] = bdm_commands::CMD_USBDM_GET_CAPABILITIES;
        let command = "CMD_USBDM_GET_CAPABILITIES".to_string();

        let bit = 0x80;
        let bitter = usb_buf[1] | bit;
        usb_buf[1] = bitter;

        self.usb_device.write(&usb_buf,1500)?;        // write command
        let answer: Vec<u8> = self.usb_device.read()?;                   //  read
    
        self.usb_device.check_usbm_return_code( &answer)?;  

        if answer.len() >= 3 {
            let capabilities: u16 = ((answer[1] as u16) << 8) | answer[2] as u16 ^ ((1<<5) | (1<<6));
            self.bdm_info.capabilities.parse(capabilities);
        }

        if answer.len() >= 5 {
            let mut buffer_size: u16 = ((answer[3] as u16) << 8) + answer[4] as u16;
            let max_packet_size: u16 = 255;
            if buffer_size > max_packet_size {
                buffer_size = max_packet_size;
            }
            let jtag_header_size: u16 = 5;
            self.bdm_info.command_buffer_size = buffer_size;
            self.bdm_info.jtag_buffer_size = buffer_size - jtag_header_size;
        }

        // Calculate permitted read & write length in bytes
        // Allow for JTAG header + USB header (5 bytes) & make multiple of 4
        self.bdm_info.dsc_max_memory_read_size  = (self.bdm_info.jtag_buffer_size - JTAG_READ_MEMORY_HEADER_SIZE  - 5) & !3;
        self.bdm_info.dsc_max_memory_write_size = (self.bdm_info.jtag_buffer_size - JTAG_WRITE_MEMORY_HEADER_SIZE - 5) & !3;
                                            
        Ok(())
    } 
}

impl fmt::Display for BdmInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl BdmInfo {
    pub fn print_version(&self) {
        println!("bdm_software_version: {:#02X}",  &self.bdm_software_version);
        println!("bdm_hardware_version: {:#02X}",  &self.bdm_hardware_version);
        println!("icp_software_version: {:#02X}",  &self.icp_software_version);
        println!("icp_hardware_version: {:#02X}",  &self.icp_hardware_version);
    }

    pub fn check_version(&self) -> Result<(), Error> {
        if &self.bdm_hardware_version != &self.icp_hardware_version { 
            Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_WRONG_BDM_REVISION))
        } else if &self.bdm_software_version < &0x40905 {
            Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_WRONG_BDM_REVISION))
        } else {
            Ok(())
        }
    }
}



impl Capabilities {
    pub fn parse(&mut self, capabilities: u16 ){
        if (capabilities & (1<<0)) != 0 { self.hcs12 = true; }
        if (capabilities & (1<<1)) != 0 { self.rs08 = true; }
        if (capabilities & (1<<2)) != 0 { self.vddcontrol = true; }
        if (capabilities & (1<<3)) != 0 { self.vddsense = true; }
        if (capabilities & (1<<4)) != 0 { self.cfv_x = true; }
        if (capabilities & (1<<5)) != 0 { self.hcs08 = true; }
        if (capabilities & (1<<6)) != 0 { self.cfv1 = true; }
        if (capabilities & (1<<7)) != 0 { self.jtag = true; }
        if (capabilities & (1<<8)) != 0 { self.dsc = true; }
        if (capabilities & (1<<9)) != 0 { self.arm_jtag = true; }
        if (capabilities & (1<<10)) != 0 { self.rst = true; }
        if (capabilities & (1<<11)) != 0 { self.pst = true; }
        if (capabilities & (1<<12)) != 0 { self.cdc = true; }
        if (capabilities & (1<<13)) != 0 { self.arm_swd = true; }
        if (capabilities & (1<<14)) != 0 { self.s12z = true; }
    }
}

