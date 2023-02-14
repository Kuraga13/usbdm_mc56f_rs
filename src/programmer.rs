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

pub fn refresh_feedback(&mut self)
{
    self.feedback = self.usb_device.get_bdm_status().unwrap();
    self.feedback.print_feedback();
    //println!("{}", self.feedback);
}

fn print_usbdm_programmer(&self) -> Result<(), Error>

{
   
    &self.capabilities.print_version();
    &self.feedback.print_feedback();
    
    Ok(())
}

pub fn set_target_mc56f(&mut self) -> Result<(), Error>{

    let mut usb_buf  = [0; 3];
    let mc56_target  =  7;  // byte to set command
    let command = "CMD_USBDM_SET_TARGET".to_string();

    usb_buf[0] = 3;            // lenght of command
    usb_buf[1] = bdm_commands::CMD_USBDM_SET_TARGET;
    usb_buf[2] = mc56_target;  
 
    let bit = 0x80;           
    let bitter = usb_buf[1] | bit;
    usb_buf[1] = bitter;

    self.usb_device.write(&usb_buf,1500)?;                                    // write command
    let answer = self.usb_device.read().expect("Can't read answer");          // read status from bdm
    let status = self.usb_device.check_usbm_return_code( &answer)?;    // check is status ok
    Ok(status)
  }
  
}



