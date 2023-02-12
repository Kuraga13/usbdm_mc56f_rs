use crate::usb_interface::{UsbInterface, find_usbdm_as, Capabilities};
use rusb::{UsbContext};
use crate::errors::{Error};
use crate::feedback::{FeedBack};
use crate::enums::{bdm_commands,vdd,vpp};



pub struct Programmer {


    usb_device   : UsbInterface,
    capabilities : Capabilities,
    feedback     : FeedBack,
   
    //jtag_buffer_size : u32,
    
    
    
    //state_from_bdm : BdmStatus,
    
    
    
    }

impl Drop for Programmer{

    fn drop(&mut self) {
        self.set_vdd_off();
        drop(&mut self.usb_device);
        println!("Programmer dropped");
    }
}

impl Programmer
{


pub fn new(mut device : UsbInterface) -> Self {


        Self{
    
            
            capabilities    : device.get_bdm_version().expect("Error on get bdm ver"),
            feedback        : device.get_bdm_status().expect("Error on feedback"),
            usb_device      : device,
        }
    
    }

pub fn set_vdd_off(&self) -> Result<(), Error>
{
            
  self.usb_device.set_vdd(vdd::BDM_TARGET_VDD_OFF)?;
  Ok(())
            
            
}
pub fn set_vdd_3_3v(&self) -> Result<(), Error>
{
        
  self.usb_device.set_vdd(vdd::BDM_TARGET_VDD_3V3)?;
  Ok(())
        
        
}

pub fn set_vdd_5v(&self) -> Result<(), Error>
{
    
    self.usb_device.set_vdd(vdd::BDM_TARGET_VDD_5V)?;
    Ok(())
    
    
}


fn set_vdd(&self, power: u8 ) -> Result<(), Error>
{

    self.usb_device.set_vdd(power)?;
    Ok(())


}

fn set_vpp(&self, power: u8 ) -> Result<(), Error>
{

    self.usb_device.set_vpp(power)?;
    Ok(())


}

fn refresh_feedback(&mut self)
{
    self.feedback = self.usb_device.get_bdm_status().unwrap();
    println!("{}", self.feedback);
}

fn print_usbdm_programmer(&self) -> Result<(), Error>

{
   
    &self.capabilities.print_version();
    &self.feedback.print_feedback();
    
    Ok(())
}

}



