#![allow(unused)]

//use rusb::{UsbContext};
use std::time::Duration;
use crate::errors::{Error, USBDM_ErrorCode};
use crate::usbdm::constants::{bdm_commands};
use crate::usbdm::feedback::{FeedBack};
use packed_struct::prelude::*;
use std::fmt;

const USB_TIMEOUT: u64 = 500;
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

pub fn find_usbdm() -> Result<rusb::Device<rusb::GlobalContext>, Error>
{
    rusb::DeviceList::new()?
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
    pub model: String,
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
            interface_n : number,})
    }

    /// print USBDM model, serial number, EP
    pub fn print_usb_interface(&self) {
        println!("Model: {}",&self.model);
        println!("Serial Number: {}",&self.serial_number);
        println!("Read EP: {} ",&self.read_ep);
        println!("Write EP: {}",&self.write_ep);
    }

    /// `write` - write_bulk to usbdm. param - data u8 slice.
    pub fn write(&self, data: &[u8]) -> Result<(), Error> {
        self.handle.write_bulk(self.write_ep, data, Duration::from_millis(USB_TIMEOUT))?;
        Ok(())
    }

    /// `read` - read_bulk from usbdm. param - rx_size
    pub fn read(&self, rx_size: usize) -> Result<Vec<u8>, Error> {
        let mut buff: Vec<u8> = vec![0; rx_size];
        self.handle.read_bulk(self.read_ep, buff.as_mut_slice(), Duration::from_millis(USB_TIMEOUT))?;
        let mut answer = buff.to_vec();
        self.check_usbm_return_code(&answer)?;
        Ok(answer)
    }

    fn check_usbm_return_code(&self, return_byte : &Vec<u8>) -> Result<(), Error> {
        let mut return_code = return_byte[0];
        return_code &= !0xC0;
        let return_from_bdm = USBDM_ErrorCode::from(return_code);
        if return_from_bdm != USBDM_ErrorCode::BDM_RC_OK {     
            return Err(Error::USBDM_Errors(return_from_bdm))
        } else {
            Ok (())
        }
    }

    pub fn control_transfer(&self, request_type: u8, request: u8, value: u16, index: u16, rx_size: usize) -> Result<Vec<u8>, Error> {
        let mut buff: Vec<u8> = vec![0; rx_size];
        self.handle.read_control(request_type, request, value, index, buff.as_mut_slice(), Duration::from_millis(USB_TIMEOUT))?;
        let control_answer = buff.to_vec();
        Ok(control_answer) 
    }
}


impl Drop for UsbInterface
{
    fn drop(&mut self) {
        self.handle.release_interface(self.interface_n);
        println!("UsbInterface dropped");
    }
}



