use std::{
    path::Path,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
};
use crate::app::{App};
use crate::errors::Error;
use super::data_parser::{to_bdm_s19_325, ParsedData, s19_to_bin};
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
         let mut s19 = s19_to_bin(buffer_vec)?;
         return Ok(())
      
        }
        FileFormat::UnknownFormat => 
        {

         return Err(Error::FileFormatErr)

        }
    };
}


pub fn save_buffer_to_file(path : String,  start_addr : u32, size : usize, app: &mut App, format : FileFormat) -> Result<(), Error>
{

    // TODO:
    // check buffer size of target MemoryMap
    // check start address + size of target MemoryMap
    // and then save bin file

     let mut save_file = match   File::create(path) {
        Ok(save_file) => save_file,
        Err(e) => return Err(Error::FileReadErr),
    };

    // get buffer   
    let mut data_to_file = app.buffer.download_in_one();

    let formatted = match format {
        FileFormat::Bin => { data_to_file},
        FileFormat::S19 => { to_bdm_s19_325(data_to_file).unwrap() },
        FileFormat::UnknownFormat => { data_to_file},
        _ => {return Err(Error::FileReadErr)}
    }; 
    

    // write it to file
    save_file.write(formatted.as_slice())?;
 

 

    Ok(())

}