//use rusb::{UsbContext};
use std::time::Duration;
use std::sync::{Arc, RwLock};
use crate::errors::{Error, USBDM_ErrorCode};
use crate::enums::{bdm_commands};
use crate::feedback::{FeedBack};
use crate::settings::{BdmSettings};
use packed_struct::prelude::*;

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
    let interface_descriptor = interface.unwrap().descriptors().next().unwrap();


    let mut handle = device.open()?;
    let number  = interface_descriptor.interface_number();
    handle.claim_interface(number)?;
    
    let device_descriptor = device.device_descriptor().unwrap();
    
    let find_endpoint = |direction, transfer_type| {
        interface_descriptor
            .endpoint_descriptors()
            .find(|ep| ep.direction() == direction && ep.transfer_type() == transfer_type)
            .map(|x| x.address())
            .ok_or(Error::Usb(rusb::Error::NotFound))
    };

    Ok(UsbInterface {
        read_ep: find_endpoint(rusb::Direction::In, rusb::TransferType::Bulk).unwrap(),
        write_ep: find_endpoint(rusb::Direction::Out, rusb::TransferType::Bulk).unwrap(),
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

pub fn get_bdm_version(&self) -> Result<Capabilities, Error>{
      
    let request_type = 64; //LIBUSB_REQUEST_TYPE_VENDOR
    let request_type = request_type| &self.read_ep;


    let request      = bdm_commands::CMD_USBDM_GET_VER; // command
    let value        = 100;
    let index        = 0;
    let timeout      = Duration::from_millis(2500);
    
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
    

     Ok(Capabilities {

        bdm_software_version : calculation,
        bdm_hardware_version : version[2],
        icp_software_version : version[3],
        icp_hardware_version : version[4],}
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

       /// `get_full_capabilities` - get all capabilities, if rx_size > 5 we need upgrade Capabilities struct...
       pub fn get_full_capabilities(&mut self) -> Result<(), Error>{
      
        let mut usb_buf = [0; 2];
        usb_buf[0] = 2;  // lenght
        usb_buf[1] = bdm_commands::CMD_USBDM_GET_CAPABILITIES;
        let command = "CMD_USBDM_GET_CAPABILITIES".to_string();
  
        let bit = 0x80;
        let bitter = usb_buf[1] | bit;
        usb_buf[1] = bitter;
  
        self.write(&usb_buf,1500)?;        // write command
        let answer = self.read()?;                   //  read

        self.check_usbm_return_code( &answer)?;                                                
        dbg!(answer);
        
        Ok(())
  
       }




      pub fn set_vdd(&self, power: u8 ) -> Result<(), Error>{
      
        let mut usb_buf  = [0; 4];
        let command = "CMD_USBDM_SET_VDD".to_string();
  
        usb_buf[0] = 4;
        usb_buf[1] = bdm_commands::CMD_USBDM_SET_VDD;
        usb_buf[2] = power;  
        usb_buf[3] = power;  
  
        let bit = 0x80;
        let bitter = usb_buf[1] | bit;
        usb_buf[1] = bitter;
  
        self.write(&usb_buf,1500)?;                                    // write command
        let answer = self.read().expect("Can't read answer");          // read status from bdm
       // self.check_usbm_return_code(command, &answer)?;               // check is status ok
        Ok(())
      }


      pub fn set_vpp(&self, power: u8 ) -> Result<(), Error>{
      
        let mut usb_buf  = [0; 4];
        let command = "CMD_USBDM_SET_VDD".to_string();
  
        usb_buf[0] = 3;
        usb_buf[1] = bdm_commands::CMD_USBDM_SET_VPP;
        usb_buf[2] = power;  
    
  
        let bit = 0x80;
        let bitter = usb_buf[1] | bit;
        usb_buf[1] = bitter;
  
        self.write(&usb_buf,1500)?;                                    // write command
        let answer = self.read().expect("Can't read answer");          // read status from bdm
       // self.check_usbm_return_code(command, &answer)?;               // check is status ok
        Ok(())
      }

}


impl Drop for UsbInterface{

    fn drop(&mut self) {

        self.handle.release_interface(self.interface_n).unwrap();
        println!("UsbInterface dropped");
    }
}

///`Capabilities`
///The idea is to group a huge number of USBDM structures, enumerations and settings into three abstractions.
/// 
/// One is Capabilities - It includes all data about capabilities USBDM, software and hardware versions, buffer sizes
pub struct Capabilities {

    bdm_software_version : u32, // Version of USBDM Firmware
    bdm_hardware_version : u8, // Version of USBDM Hardware
    icp_software_version : u8, // Version of ICP bootloader Firmware
    icp_hardware_version : u8, // Version of Hardware (reported by ICP code)
}


impl Capabilities {




pub fn print_version(&self)
{

    println!("bdm_software_version: {:#02X}",  &self.bdm_software_version);
    println!("bdm_hardware_version: {:#02X}",  &self.bdm_hardware_version);
    println!("icp_software_version: {:#02X}",  &self.icp_software_version);
    println!("icp_hardware_version: {:#02X}",  &self.icp_hardware_version);

}

pub fn check_version(&self) -> Result<(), Error>{

    if &self.bdm_hardware_version != &self.icp_hardware_version{ 
        Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_WRONG_BDM_REVISION))
    }
    else if &self.bdm_software_version < &0x40905 
    {
        Err(Error::USBDM_Errors(USBDM_ErrorCode::BDM_RC_WRONG_BDM_REVISION))
    }
    else
    {
        Ok(())
    }

    }
}




