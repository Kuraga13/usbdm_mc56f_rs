#![allow(unused)]

//use rusb::{UsbContext};
use std::time::Duration;
use crate::errors::{Error, USBDM_ErrorCode};
use crate::enums::{bdm_commands};
use crate::feedback::{FeedBack};
use packed_struct::prelude::*;
use std::fmt;

const USBDM_VID: u16 = 0x16D0;



pub fn match_vid<T: rusb::UsbContext>(device:&rusb::Device<T>) -> bool 
{
    if let Ok(descriptor) = device.device_descriptor() 
    {
       descriptor.vendor_id() == USBDM_VID    
  
    } 
    else 
    {
        false
    }
}
pub async fn find_usbdm_as() -> Result<rusb::Device<rusb::GlobalContext>, Error>

{
    rusb::DeviceList::new()
    .unwrap()
    .iter()
    .filter(match_vid)
    .next()
    .ok_or(Error::Usb(rusb::Error::NotFound))
    
}

pub  fn find_usbdm() -> Result<rusb::Device<rusb::GlobalContext>, Error>

{
    rusb::DeviceList::new()
    .unwrap()
    .iter()
    .filter(match_vid)
    .next()
    .ok_or(Error::Usb(rusb::Error::NotFound))
    
}



#[derive(Debug)]

pub struct UsbInterface
{
   // handle: Arc<RwLock<rusb::DeviceHandle<rusb::GlobalContext>>>,
    handle: rusb::DeviceHandle<rusb::GlobalContext>,
    pub read_ep: u8,
    write_ep: u8,
    model: String,
    serial_number: String,
    interface_n  : u8,

}




impl  UsbInterface
{


pub fn new(device: rusb::Device<rusb::GlobalContext>) -> Result<Self, Error> {
    
    let config = device.active_config_descriptor()?;
    let interface = config.interfaces().next();
    let interface_descriptor = interface.ok_or(Error::Usb(rusb::Error::NotFound))?.descriptors().next().ok_or(Error::Usb(rusb::Error::NotFound))?;


    let mut handle = device.open()?;
    let number  = interface_descriptor.interface_number();
    handle.claim_interface(number)?;
    
    let device_descriptor = device.device_descriptor()?;
    
    let find_endpoint = |direction, transfer_type| {
        interface_descriptor
            .endpoint_descriptors()
            .find(|ep| ep.direction() == direction && ep.transfer_type() == transfer_type)
            .map(|x| x.address())
            .ok_or(Error::Usb(rusb::Error::NotFound))
    };

    Ok(UsbInterface {
        read_ep: find_endpoint(rusb::Direction::In, rusb::TransferType::Bulk)?,
        write_ep: find_endpoint(rusb::Direction::Out, rusb::TransferType::Bulk)?,
        model: handle.read_product_string_ascii(&device_descriptor)?,
        serial_number: handle.read_serial_number_string_ascii(&device_descriptor)?,
        //handle: Arc::new(RwLock::new(handle)),
        handle: handle,
        interface_n : number,

       
    })
}


/// print USBDM model, serial number, EP
pub fn print_usb_interface(&self)  {

    println!("Model: {}",&self.model);
    println!("Serial Number: {}",&self.serial_number);
    println!("Read EP: {} ",&self.read_ep);
    println!("Write EP: {}",&self.write_ep);
}

/// `write` - write_bulk to usbdm. param - programmer, data u8 slice, timeout.
pub fn write(&self, data: &[u8], timeout_value: u64) -> Result<(), Error> {
    let timeout = Duration::from_millis(timeout_value);
    //self.handle.read().unwrap().write_bulk(self.write_ep, data, timeout)?;
    self.handle.write_bulk(self.write_ep, data, timeout)?;
    Ok(())
}


/// `read` - read_bulk from usbdm. param - programmer
pub fn read(&self) -> Result<Vec<u8>, Error> {

    const RECEIVE_SIZE: usize = 32;
    let mut buff = [0; RECEIVE_SIZE];
    //self.handle.read().unwrap().read_bulk(self.read_ep, &mut buff, Duration::from_millis(2500))?;
    self.handle.read_bulk(self.read_ep, &mut buff, Duration::from_millis(2500))?;
    let answer = buff.to_vec();
    let check_status = self.check_usbm_return_code(&answer)?;
    Ok(answer)

}

pub fn check_usbm_return_code(&self, return_byte : &Vec<u8>) -> Result<(), Error>{
    
    let mut return_code = return_byte[0];
    return_code &= !0xC0;
    let return_from_bdm = USBDM_ErrorCode::from(return_code);
     
    if return_from_bdm != USBDM_ErrorCode::BDM_RC_OK {     

       eprintln!("Error: BDM RC status from BDM : {}!", return_from_bdm);
       return Err(Error::USBDM_Errors(return_from_bdm))

      }

       else {
      
       println!("OK Status from BDM RC : {}!", return_from_bdm);
       Ok (())

      }
  }

pub fn read_slice(&self) -> Result<[u8;32], Error> {

    const RECEIVE_SIZE: usize = 32;
    let mut buff = [0; RECEIVE_SIZE];
    //self.handle.read().unwrap().read_bulk(self.read_ep, &mut buff, Duration::from_millis(2500))?;
    self.handle.read_bulk(self.read_ep, &mut buff, Duration::from_millis(2500))?;
    

    
    Ok(buff)

}

pub fn control_transfer(
    &self,
    request_type: u8,
    request: u8,
    value: u16,
    index: u16,
    usb_buff : &mut [u8],
    timeout: Duration
) -> Result<Vec<u8>, Error> {


  // self.handle.read().unwrap().read_control(request_type, request, value, index, usb_buff, timeout)?;
   self.handle.read_control(request_type, request, value, index, usb_buff, timeout)?;
   let control_answer = usb_buff.to_vec();
   
   Ok(control_answer) 
}

pub fn get_bdm_version(&self) -> Result<BdmInfo, Error>{
      
    let request_type = 64; //LIBUSB_REQUEST_TYPE_VENDOR
    let request_type = request_type| &self.read_ep;


    let request      = bdm_commands::CMD_USBDM_GET_VER; // command
    let value       = 100;
    let index       = 0;
    let timeout= Duration::from_millis(2500);
    
    let mut usb_buf  = [0; 10];
 

    let version = self.control_transfer(
      request_type,
      request,
      value,
      index,
      &mut usb_buf,
      timeout)?;                                    

    let raw_bdm_software_version = u32::from (version[1]);
    let calculation = ((raw_bdm_software_version&0xF0)<<12) + ((raw_bdm_software_version&0x0F)<<8);
    

     Ok(BdmInfo {

        bdm_software_version : calculation,
        bdm_hardware_version : version[2],
        icp_software_version : version[3],
        icp_hardware_version : version[4],
        ..Default::default() }
      )
    
    }

   /// `get_bdm_status` - get status byte from USBDM by rusb read_bulk 
   ///  Return packed-struct (bits-field) `FeebBack` with currently status of USBDM 
    pub fn get_bdm_status(&mut self) -> Result<FeedBack, Error>{
      
        let mut usb_buf = [0; 2];
        usb_buf[0] = 2;  // lenght
        usb_buf[1] = bdm_commands::CMD_USBDM_GET_BDM_STATUS;
        let command = "CMD_USBDM_GET_BDM_STATUS".to_string();
  
        let bit = 0x80;
        let bitter = usb_buf[1] | bit;
        usb_buf[1] = bitter;
  
        self.write(&usb_buf,1500)?;                  // write command
        let answer = self.read()?;                   //  read status from bdm and save buffer to answer -
                                                     
        let feedback_slice = [answer[3],answer[2]];      // two bytes for status feedback (in answer [3] use only 2 bits... for VPP bits)
        println!("FeedBack is: {:02X?}", feedback_slice);
        //let unpack = FeedBack::unpack(&[0x02, 0xff]).unwrap();   // for test TODO - write test's in FeedBack and paste where
        let unpack = FeedBack::unpack(&feedback_slice).unwrap();
    
        Ok(unpack)
  
      }

       






}


impl Drop for UsbInterface{

    fn drop(&mut self) {

        self.handle.release_interface(self.interface_n).unwrap();
        println!("UsbInterface dropped");
    }
}

///`BdmInfo`
///The idea is to group a huge number of USBDM structures, enumerations and settings into three abstractions.
/// 
/// One is BdmInfo - It includes all data information about USBDM, software and hardware versions, buffer sizes
#[derive(Debug, PartialEq)]
pub struct BdmInfo {
    pub bdm_software_version:   u32,           // Version of USBDM Firmware
    pub bdm_hardware_version:   u8,            // Version of USBDM Hardware
    pub icp_software_version:   u8,            // Version of ICP bootloader Firmware
    pub icp_hardware_version:   u8,            // Version of Hardware (reported by ICP code)
    pub capabilities:           Capabilities,  // BDM Capabilities
    pub command_buffer_size:    u16,           // Size of BDM Communication buffer
    pub jtag_buffer_size:       u16,           // Size of JTAG buffer (if supported)
}

impl Default for BdmInfo {
    fn default() -> Self { 
        BdmInfo {
            bdm_software_version:   0,
            bdm_hardware_version:   0,
            icp_software_version:   0,
            icp_hardware_version:   0,
            capabilities:           Capabilities::default(),
            command_buffer_size:    0,
            jtag_buffer_size:       0,
        }
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

