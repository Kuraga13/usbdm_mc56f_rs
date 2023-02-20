
use crate::errors::Error;
use crate::jtag::*;
use crate::programmer::{Programmer};
pub struct Target {
    
   pub programmer : Programmer,
    
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

            programmer : prg,

        }

    }

    pub fn dsc_connect(&self) -> Result<(), Error> {

        let master_id_code = read_master_id_code(true, &self.programmer);
        dbg!(master_id_code);
        enableCoreTAP(&self.programmer); // on second not
        let core_id_code = read_core_id_code(true, &self.programmer);
        dbg!(core_id_code);
        Ok(())
    }
}