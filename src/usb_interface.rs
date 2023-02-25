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
    self.handle.write_bulk(self.write_ep, data, timeout)?;
    Ok(())
}


/// `read` - read_bulk from usbdm. param - programmer
pub fn read(&self, rx_size: usize) -> Result<Vec<u8>, Error> {

    let mut buff: Vec<u8> = vec![0; rx_size];
    self.handle.read_bulk(self.read_ep, buff.as_mut_slice(), Duration::from_millis(500))?;
    let mut answer = buff.to_vec();
    let check_status = self.check_usbm_return_code(&answer)?;
    Ok(answer)

}

fn check_usbm_return_code(&self, return_byte : &Vec<u8>) -> Result<(), Error>{
    
    let mut return_code = return_byte[0];
    return_code &= !0xC0;
    let return_from_bdm = USBDM_ErrorCode::from(return_code);
     
    if return_from_bdm != USBDM_ErrorCode::BDM_RC_OK {     

       println!("Error: BDM RC status from BDM : {}!", return_from_bdm);
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


   self.handle.read_control(request_type, request, value, index, usb_buff, timeout)?;
   let control_answer = usb_buff.to_vec();
   
   Ok(control_answer) 
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
        let answer = self.read(3)?;                   //  read status from bdm and save buffer to answer -
                                                     
        let feedback_slice = [answer[1],answer[2]];      // two bytes for status feedback (in answer [1] use only 2 bits... for VPP bits)
        println!("FeedBack is: {:02X?}", feedback_slice);
        let unpack = FeedBack::unpack(&feedback_slice)?;
    
        Ok(unpack)
  
      }

       






}


impl Drop for UsbInterface{

    fn drop(&mut self) {

        self.handle.release_interface(self.interface_n);
        println!("UsbInterface dropped");
    }
}



