mod to_s19;
mod parse_s19;
mod sort_and_check;
mod bin_utils;

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedData {
    valid: bool,
    word_length: u8,
    pub data_vec: Vec<DataBlock>,
}

impl Default for ParsedData {
    fn default() -> Self { 
        ParsedData {
            valid        : false,
            word_length  : 0,
            data_vec     : vec![],
        }
    } 
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataBlock {
    address: u32,
    pub data_blob: Vec<u8>,
}

impl Default for DataBlock {
    fn default() -> Self { 
        DataBlock {
            address: 0,
            data_blob: vec![],
        }
    } 
}

#[derive(Debug,Clone,PartialEq)]
pub enum Error {
    DataParserError(String),
 }



