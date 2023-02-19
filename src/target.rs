use crate::jtag::jtag::{JtagInterface};
use crate::errors::Error;

pub struct Target {
    jtag: JtagInterface,
    }

impl Target{

    pub fn init(&mut self, jtag: JtagInterface) {
        self.jtag = jtag;
    }

    pub fn dsc_connect(&self) -> Result<(), Error> {

        let master_id_code = self.jtag.read_master_id_code(true);
        self.jtag.enableCoreTAP(); // on second not
        let core_id_code = self.jtag.read_core_id_code(true);
        Ok(())
    }
}