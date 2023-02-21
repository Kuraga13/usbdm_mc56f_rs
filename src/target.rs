use crate::errors::Error;
use crate::jtag::*;
use crate::programmer::{Programmer};
use crate::settings::{TargetVddSelect};


pub struct Target {
    
   pub programmer  : Programmer,
   pub once_status : OnceStatus,

    
}

impl Drop for Target{

        fn drop(&mut self) {
            drop(&mut self.programmer);
            println!("Target dropped");
        }
}

impl Target{

    pub fn init(prg : Programmer) -> Self {
        Self{

            programmer  : prg,
            once_status : OnceStatus::UnknownMode,
          
      }
   }
}

pub trait TargetProgramming
{

/// Init
fn init(&mut self) -> Result<(), Error>;

/// Connect
fn connect(&mut self, power : TargetVddSelect ) -> Result<(), Error>;

/// Power. Toggled
fn power(&mut self, power : TargetVddSelect ) -> Result<(), Error>;

/// Disconnect
fn disconnect(&self);

/// Read target
fn read_target(&self) -> Result<(), Error>;

/// Write target
fn write_target(&self) -> Result<(), Error>;

/// Write target
fn erase_target(&self) -> Result<(), Error>;

}

impl TargetProgramming for Target
{

fn init(&mut self) -> Result<(), Error>
{

  if let Err(_e) = self.programmer.refresh_feedback() { Err(_e) }
  else
  {
  self.programmer.set_bdm_options()?;
  self.programmer.set_target_mc56f()?;
  Ok(())  
  }

}

fn connect(&mut self, power : TargetVddSelect) -> Result<(), Error>
{

  self.power(power);
  let master_id_code = read_master_id_code(true, &self.programmer).expect("Dsc target connect error");
  dbg!(master_id_code);
  enableCoreTAP(&self.programmer); // on second not
  let core_id_code = read_core_id_code(true, &self.programmer);
  dbg!(core_id_code);
  self.once_status = enableONCE(&self.programmer)?;
  dbg!(&self.once_status);
  
  match self.once_status
  {
   OnceStatus::UnknownMode => 
   {
     return Err((Error::TargetNotConnected))
   }
   _=>{}

  }

  Ok(())
    
}

fn power(&mut self, power : TargetVddSelect) -> Result<(), Error>
{

 let mut is_powered = self.programmer.check_power().expect("Err on check power!");
 dbg!(is_powered);
 match power
 {
    TargetVddSelect::VddOff =>
    {   
        if(is_powered)
        {
         self.programmer.set_vdd(TargetVddSelect::VddOff);
         is_powered = self.programmer.check_power().expect("Err on check power!");
         if(!is_powered)
         {
            Ok(())
         }
         else
         {
            Err((Error::PowerStateError))
         }
        }
        else
        {
        Ok(())
        }
    }
    _ => 
    {
        if(!is_powered)
        {
        self.programmer.set_vdd(power);
        is_powered = self.programmer.check_power().expect("Err on check power!");
         if(is_powered)
         {
            Ok(())
         }
         else
         {
            Err((Error::PowerStateError))
         }
        }
        else
        {
        Ok(())
        }
     }
   }
}

    

fn disconnect(&self) 
{
    
 drop(self);
  
}

fn read_target(&self) -> Result<(), Error>
{
    
    unimplemented!()
}

fn write_target(&self) -> Result<(), Error>
{
    
    unimplemented!()
    
}

fn erase_target(&self) -> Result<(), Error>
{
    
    unimplemented!()
    
}




}