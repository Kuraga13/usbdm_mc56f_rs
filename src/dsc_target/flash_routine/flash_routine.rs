use super::*;

use crate::usbdm::registers::*;

#[derive(Debug)]
pub struct FlashRoutine {
    dsc_family: DscFamily,
    routine: BaseRoutine,
    timing_header: TimingHeader,
    data_header: DataHeader,
    max_write_size: u32,
}

impl FlashRoutine {
    pub fn init(dsc_family : DscFamily, ram_size : u32) -> Result<Self, Error> {
        let routine: BaseRoutine = BaseRoutine::get(dsc_family.clone())?;
        let timing_header: TimingHeader = TimingHeader::get();
        let mut data_header: DataHeader = DataHeader::get(&dsc_family)?;
        data_header.data_address = routine.data_header_address + data_header.len()? / 2;
        let max_write_size: u32 = ((ram_size - data_header.data_address) * 2) & !0x0F;
        Ok (
            Self {
                dsc_family,
                routine,
                timing_header,
                data_header,
                max_write_size
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

        if (result_header.error_code != 0) {
            return Err(Error::InternalError(parse_flash_err(result_header.error_code))) }   

        if (result_header.flash_operation != DO_TIMING_LOOP) {
            return Err(Error::InternalError("Flash operation is not DO_TIMING_LOOP".to_string())) }
    
        if (result_header.timing_count == 0) {
            return Err(Error::InternalError("Unexpected timing_count zero value".to_string())) }

        // Round appropriately (approx 3 digits)
        let target_bus_frequency:u32 = 
            (40.0 * (0.5 + (result_header.timing_count as f64) * (self.routine.calib_frequency as f64) / ( 40.0 * (self.routine.calib_factor as f64)))) as u32;
    
        Ok(target_bus_frequency)
    }

    pub fn dsc_write_prog_mem(&mut self, prog: &mut Programmer, mut data: Vec<u8>, address: u32) -> Result<(), Error> {
        let mut current_address: u32 = address;

        while (data.len() > 0) {
            let mut block_size = data.len() as u32;
            
            if (block_size > self.max_write_size) {
                block_size = self.max_write_size; };

            self.routine_write_block(prog, data.drain(..block_size as usize).collect(), current_address)?;
                current_address += block_size as u32 / 2; // Address advanced by count of words written
        }
        
        Ok(())      
    }

    fn routine_write_block(&mut self, prog: &mut Programmer, data: Vec<u8>, address: u32) -> Result<(), Error> {

        self.data_header.flash_operation = OP_PROGRAM;
        self.data_header.address = address;
        self.data_header.data_size = (data.len() / 2) as u16;

        prog.dsc_write_memory(self.routine.address_memspace, self.routine.routine.clone(), self.routine.address)?;
        prog.dsc_write_memory(self.routine.data_header_address_memspace, self.data_header.to_vec()?, self.routine.data_header_address)?;
        prog.dsc_write_memory(self.routine.data_header_address_memspace, data, self.data_header.data_address)?;

        prog.dsc_write_pc(self.routine.code_entry)?;
        prog.dsc_target_go()?;

        let mut timeout = 30;
        loop {
            thread::sleep(time::Duration::from_millis(20));
            let once_status = enableONCE(&prog)?;
            if once_status == OnceStatus::DebugMode {break}
            timeout -= 1;
            if timeout <= 0 { println!("Routine halt failed!!! Timeout used"); break}
        }

        prog.dsc_target_halt()?;

        let header_vec: Vec<u8> = prog.dsc_read_memory(self.routine.data_header_address_memspace, self.data_header.len()?, self.routine.data_header_address)?; 
        let header: DataHeader = DataHeader::from_vec(header_vec)?;
        
        if (header.flash_operation & 0x8000) == 0 {
            return Err(Error::InternalError("No complition flag in write_block".to_string())) }

        Ok(())
    }

    fn routine_blank_check_range(&mut self, prog: &mut Programmer, start_address: u32, end_address: u32) -> Result<bool, Error> {

        self.data_header.flash_operation = OP_BLANK_CHECK;
        self.data_header.address = start_address;
        self.data_header.data_size = (end_address - start_address + 1) as u16;

        prog.dsc_write_memory(self.routine.address_memspace, self.routine.routine.clone(), self.routine.address)?;
        prog.dsc_write_memory(self.routine.data_header_address_memspace, self.data_header.to_vec()?, self.routine.data_header_address)?;

        prog.dsc_write_pc(self.routine.code_entry)?;
        prog.dsc_target_go()?;

        let mut timeout = 30;
        loop {
            thread::sleep(time::Duration::from_millis(20));
            let once_status = enableONCE(&prog)?;
            if once_status == OnceStatus::DebugMode {break}
            timeout -= 1;
            if timeout <= 0 { println!("Routine halt failed!!! Timeout used"); break}
        }

        prog.dsc_target_halt()?;

        let header_vec: Vec<u8> = prog.dsc_read_memory(self.routine.data_header_address_memspace, self.data_header.len()?, self.routine.data_header_address)?; 
        let header: DataHeader = DataHeader::from_vec(header_vec)?;
        
        if (header.flash_operation & 0x8000) == 0 {
            return Err(Error::InternalError("No complition flag in blank_check".to_string())) }

        if header.error_code == 0 {return Ok(true)}
        if header.error_code == 6 {return Ok(false)}
        Err(Error::InternalError("Blank Check Error".to_string()))
    }

    pub fn dsc_erase_routine(&mut self, prog: &mut Programmer) -> Result<(), Error> {

        self.data_header.flash_operation = OP_ERASE_BLOCK;

        prog.dsc_write_memory(self.routine.address_memspace, self.routine.routine.clone(), self.routine.address)?;
        prog.dsc_write_memory(self.routine.data_header_address_memspace, self.data_header.to_vec()?, self.routine.data_header_address)?;

        prog.dsc_write_pc(self.routine.code_entry)?;
        prog.dsc_target_go()?;

        thread::sleep(time::Duration::from_millis(100));
 
        let once_status = enableONCE(&prog)?;
        prog.dsc_target_halt()?;

        let header_vec: Vec<u8> = prog.dsc_read_memory(self.routine.data_header_address_memspace, self.data_header.len()?, self.routine.data_header_address)?; 
        let header: DataHeader = DataHeader::from_vec(header_vec)?;
        
        if (header.flash_operation & 0x8000) == 0 {
            return Err(Error::InternalError("No complition flag in erase_block".to_string())) }

        Ok(())
    }
}