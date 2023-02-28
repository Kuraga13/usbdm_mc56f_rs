use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
};
use crate::app::{App};
use crate::errors::Error;

pub fn load_buffer_from_file(path : String, start_addr : u32, size : usize, app : &mut App ) -> Result<(), Error>
{

     let mut file_hex = match  File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(Error::FileReadErr),
    };
 
   /*  match format {
        Format::Bin(options) => loader.load_bin_data(&mut file, options),
        Format::S19 => loader.load_elf_data(&mut file),
    }?;
    */
    
    file_hex.seek(SeekFrom::Start(u64::from(start_addr)))?;
    let mut bin_vec: Vec<u8> = Vec::new();
    file_hex.read_to_end(&mut bin_vec);
    app.buffer.upload(bin_vec);

    Ok(())

}

pub fn save_buffer_to_file(path : String,  start_addr : u32, size : usize, app: &mut App ) -> Result<(), Error>
{
    /*  match format {
        Format::Bin(options) => loader.load_bin_data(&mut file, options),
        Format::S19 => loader.load_elf_data(&mut file),
    }?; // for future
    */
    

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

    // write it to file
    save_file.write(data_to_file.as_slice())?;
 

 

    Ok(())

}