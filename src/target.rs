use crate::errors::Error;
use crate::programmer::jtag::*;
use crate::programmer::jtag::{OnceStatus};
use crate::programmer::{Programmer};
use crate::settings::{TargetVddSelect};
use crate::feedback::{PowerStatus};


    // Memory space indicator - includes element size
    // One of the following
    pub const MS_BYTE    : u8  = 1;        // Byte (8-bit) access
    pub const MS_WORD    : u8  = 2;        // Word (16-bit) access
    pub const MS_LONG    : u8  = 4;        // Long (32-bit) access
    // One of the following
    pub const MS_NONE    : u8  = 0<<4;     // Memory space unused/undifferentiated
    pub const MS_PROGRAM : u8  = 1<<4;     // Program memory space (e.g. P: on DSC)
    pub const MS_DATA    : u8  = 2<<4;     // Data memory space (e.g. X: on DSC)
  //  pub const MS_GLOBAL  : u8  = 3<<4;     // HCS12 Global addresses (Using BDMPPR register)
    // Fast memory access for HCS08/HCS12 (stopped target, regs. are modified
   // pub const MS_FAST    : u8  = 1<<7;
    // Masks for above
   pub const MS_SIZE    : u8  = 0x7<<0;   // Size
    //pub const MS_SPACE   : u8  = 0x7<<4;   // Memory space
    // For convenience (DSC)
    pub const MS_PWORD   : u8  = MS_WORD + MS_PROGRAM;
    pub const MS_PLONG   : u8  = MS_LONG + MS_PROGRAM;
    pub const MS_XBYTE   : u8  = MS_LONG + MS_DATA;
    pub const MS_XWORD   : u8  = MS_WORD + MS_DATA;
    pub const MS_XLONG   : u8  = MS_LONG + MS_DATA;






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

    pub fn new(prg : Programmer) -> Self {
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
fn disconnect(&mut self);

/// Read target
fn read_target(&mut self, power : TargetVddSelect) -> Result<(), Error>;

/// Write target
fn write_target(&self) -> Result<(), Error>;

/// Write target
fn erase_target(&self) -> Result<(), Error>;

}

impl TargetProgramming for Target
{

fn init(&mut self) -> Result<(), Error>
{

  self.programmer.refresh_feedback()?;
  self.programmer.set_bdm_options()?;
  self.programmer.set_target_mc56f()?;

  Ok(())  

}

fn connect(&mut self, power : TargetVddSelect) -> Result<(), Error>
{

  self.power(power);
  self.programmer.set_bdm_options()?;
  self.programmer.set_target_mc56f()?;
  self.programmer.refresh_feedback()?;
 

  let master_id_code = read_master_id_code(true, &self.programmer)?;
  dbg!(master_id_code);
  enableCoreTAP(&self.programmer); // on second not
  let core_id_code = read_core_id_code(false, &self.programmer)?;
  dbg!(core_id_code);

  self.once_status = OnceStatus::UnknownMode;

  while(self.once_status  != OnceStatus::DebugMode)
  {
   dbg!(&self.once_status);
   self.once_status = targetDebugRequest(&self.programmer)?;
  }

  self.once_status = enableONCE(&self.programmer)?;
  dbg!("Final status is: {} ", &self.once_status);
  
  if(self.once_status == OnceStatus::UnknownMode) 
  {

     return Err((Error::TargetNotConnected))

  }

  Ok(())
    
}

fn power(&mut self, user_power_query : TargetVddSelect) -> Result<(), Error>
{
                                                                        
  self.programmer.set_vdd(user_power_query);                      // If we try double-set power, filter in set_vdd just return ok
  self.programmer.check_expected_power(user_power_query)?;              // Check power is setted
  Ok(())

}


fn disconnect(&mut self) 
{
    
 drop(self);
  
}

fn read_target(&mut self, power : TargetVddSelect) -> Result<(), Error>
{
 
 self.programmer.target_power_reset()?;
 self.connect(power);
 dbg!(&self.once_status);
 

 let memory_read = self.programmer.dsc_read_memory(MS_PWORD, 0x200,  0x8000)?;
 
 let mut printed_vec = Vec::new();

 for byte in memory_read.iter()
 {
   let in_string = format!("{:02X}", byte);

   printed_vec.push(in_string);
   if(printed_vec.len() == 0x0F)
   {

    for symbol in printed_vec.iter()
    {
      println!("{}", symbol);
    }

    println!("\n");
    printed_vec.clear();

   }  
 }

 Ok(())


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