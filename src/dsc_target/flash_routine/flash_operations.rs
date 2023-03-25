use super::*;


/// `get_target_speed` 
/// 
/// load flash routine to target, execute, measure execution speed and calculate target frequence.
/// Need to calculate and init ClkDiv.
/// 
/// assume `target inited & debug state`
/// 
pub fn get_target_speed(family : BaseRoutineFamily, prog :  &mut Programmer) -> Result<u32, Error>
{
    let routine: BaseRoutine = BaseRoutine::get(family)?;
    let timing_header: TimingRoutineHeader = TimingRoutineHeader::get();

    prog.dsc_write_memory(memory_space_t::MS_PWORD, routine.routine, routine.code_load_address)?;
    prog.dsc_write_memory(memory_space_t::MS_PWORD, timing_header.to_vec()?, routine.data_header_address + 0x8000)?;

    prog.dsc_write_pc(routine.code_entry)?;
    prog.dsc_target_go()?;

    thread::sleep(time::Duration::from_millis(1000));

    prog.dsc_target_halt()?;

    let result_header_vec: Vec<u8> = prog.dsc_read_memory(memory_space_t::MS_XWORD, timing_header.len()?, routine.data_header_address)?; 
    let result_header: TimingRoutineHeader = TimingRoutineHeader::from_vec(result_header_vec)?;

    if (result_header.flash_operation != DO_TIMING_LOOP) {
        return Err(Error::InternalError("Flash operation is not DO_TIMING_LOOP".to_string())) }
    
    if (result_header.error_code != 0) {
        return Err(Error::InternalError(parse_flash_err(result_header.error_code))) }   

    if (result_header.timing_count == 0) {
        return Err(Error::InternalError("Unexpected timing_count zero value".to_string())) }

    // Round appropriately (approx 3 digits)
    let target_bus_frequency:u32 = 
        (40.0 * (0.5 + (result_header.timing_count as f64) * (routine.calib_frequency as f64) / ( 40.0 * (routine.calib_factor as f64)))) as u32;
    
    Ok(target_bus_frequency)
}