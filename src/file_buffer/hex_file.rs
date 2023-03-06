use std::{
    path::Path,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
};
use crate::app::{App};
use crate::errors::Error;
use super::data_parser::{to_bdm_s19_325, ParsedData};
use std::ffi::{OsStr, OsString};

#[derive(Debug, Clone, PartialEq)]
pub enum FileFormat {
    
    S19,
    Bin,
    UnknownFormat

}
// parse extestion now NOT IMPLEMENTED... in plan
impl From <&OsStr>  for FileFormat  {
    fn from(ext : &OsStr) -> FileFormat {
         match ext.to_str() {
         Some("bin")   => FileFormat::Bin, 
         Some("Bin")   => FileFormat::Bin,  
         Some("BIN")   => FileFormat::Bin,          
         Some("s19")   => FileFormat::S19,
         Some("S19")   => FileFormat::S19,                  
         Some(_)       => FileFormat::UnknownFormat,
         None          => FileFormat::UnknownFormat,
     }  
   }    
}
  



pub fn load_buffer_from_file(path : String, start_addr : u32, size : usize, app : &mut App ) -> Result<(), Error>
{
    
    let binding = path.clone();
    let ext = 
    match Path::new(&binding).extension() {
        None                        => return Err(Error::FileReadErr),
        Some(ext_str)       => ext_str};

    let format = FileFormat::from(ext);
   
    let mut file_hex = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(Error::from(e)),
    };

    let mut buffer_vec: Vec<u8> = Vec::new();
 
    match format {

        FileFormat::Bin => 
        {

            file_hex.seek(SeekFrom::Start(u64::from(start_addr)))?;
            file_hex.read_to_end(&mut buffer_vec);
            app.buffer.upload(buffer_vec);
            return Ok(())

        }
        FileFormat::S19 => 
        {
            file_hex.read_to_end(&mut buffer_vec);
            let parsed_data = ParsedData::parse_s19(buffer_vec)?;
            app.buffer.upload(parsed_data.to_bin()?);
            return Ok(())
      
        }
        FileFormat::UnknownFormat => 
        {

            return Err(Error::FileFormatErr)

        }
    };
}


pub fn save_buffer_to_file(path : String,  start_addr : u32, size : usize, app: &mut App) -> Result<(), Error>
{

    // TODO:
    // check buffer size of target MemoryMap
    // check start address + size of target MemoryMap
    // and then save bin file

    let binding = path.clone();
    let ext = 
    match Path::new(&binding).extension() {
        None                        => return Err(Error::FileReadErr),
        Some(ext_str)       => ext_str};

    let format = FileFormat::from(ext);
   

     let mut save_file = match   File::create(path) {
        Ok(save_file) => save_file,
        Err(e) => return Err(Error::FileReadErr),
    };

    // get buffer   
    let mut data_to_file = app.buffer.download_in_one();

    match format {

        FileFormat::Bin => 
        {

          save_file.write(data_to_file.as_slice())?;
          return Ok(())

        }
        FileFormat::S19 => 
        {

          data_to_file = to_bdm_s19_325(data_to_file)?;
          save_file.write(data_to_file.as_slice())?;
          return Ok(())
      
        }
        FileFormat::UnknownFormat => 
        {

         return Err(Error::FileFormatErr)

        }
    };

}