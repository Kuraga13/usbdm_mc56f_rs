use super::*;

impl DataHeader {
    pub fn get_flash_controller(dsc_family : &DscFamily) -> Result<u32, Error> {
        let control_register: u32 =  match dsc_family {
            DscFamily::Mc56f800X => 0xF400,
            DscFamily::Mc56f801X => 0xF400,
            DscFamily::Mc56f802X => 0xF400,
            DscFamily::Mc56f803X => 0xF400,
            _                    => return Err(Error::InternalError("Flash Controller not defined for this DscFamily".to_string())),
        };
        Ok(control_register)
    }

    pub fn get_sector_size(dsc_family : &DscFamily) -> Result<u16, Error> {
        let sector_size: u16 =  match dsc_family {
            DscFamily::Mc56f800X => 256,
            DscFamily::Mc56f801X => 256,
            DscFamily::Mc56f802X => 256,
            DscFamily::Mc56f803X => 256,
            _                    => return Err(Error::InternalError("Sector Size not defined for this DscFamily".to_string())),
        };
        Ok(sector_size)
    }
}