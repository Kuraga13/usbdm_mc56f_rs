use super::*;

#[derive(Debug)]
pub struct FlashRoutine {
    dsc_family: DscFamily,
    routine: BaseRoutine,
    timing_header: TimingHeader,
}

impl FlashRoutine {
    pub fn init(dsc_family : DscFamily) -> Result<Self, Error> {
        let routine: BaseRoutine = BaseRoutine::get(dsc_family.clone())?;
        let timing_header: TimingHeader = TimingHeader::get();
        
        Ok (
            Self {
                dsc_family,
                routine,
                timing_header,
            }
        )
    }

    /// `get_target_speed` 
    /// 
    /// load flash routine to target, execute, measure execution speed and calculate target frequence.
    /// Need to calculate and init ClkDiv.
    /// 
    /// assume `target inited & debug state`
    /// 
    pub fn get_target_speed(&mut self, prog: &mut Programmer) -> Result<u32, Error>
    {
        prog.dsc_write_memory(self.routine.address_memspace, self.routine.routine.clone(), self.routine.address)?;
        prog.dsc_write_memory(self.routine.data_header_address_memspace, self.timing_header.to_vec()?, self.routine.data_header_address)?;

        prog.dsc_write_pc(self.routine.code_entry)?;
        prog.dsc_target_go()?;

        thread::sleep(time::Duration::from_millis(1000));

        prog.dsc_target_halt()?;

        let result_header_vec: Vec<u8> = prog.dsc_read_memory(self.routine.data_header_address_memspace, self.timing_header.len()?, self.routine.data_header_address)?; 
        let result_header: TimingHeader = TimingHeader::from_vec(result_header_vec)?;

        if (result_header.flash_operation != DO_TIMING_LOOP) {
            return Err(Error::InternalError("Flash operation is not DO_TIMING_LOOP".to_string())) }
    
        if (result_header.error_code != 0) {
            return Err(Error::InternalError(parse_flash_err(result_header.error_code))) }   

        if (result_header.timing_count == 0) {
            return Err(Error::InternalError("Unexpected timing_count zero value".to_string())) }

        // Round appropriately (approx 3 digits)
        let target_bus_frequency:u32 = 
            (40.0 * (0.5 + (result_header.timing_count as f64) * (self.routine.calib_frequency as f64) / ( 40.0 * (self.routine.calib_factor as f64)))) as u32;
    
        Ok(target_bus_frequency)
    }
}