#![allow(unused)]

use iced_native::layout::{self, Layout};
use iced_native::renderer;
use iced_native::widget::{self, Widget};
use iced_native::{Color, Element, Length, Point, Rectangle, Size};

use std::collections::HashMap;
use crate::errors::{Error};

type AddressKey = u32;
type ByteValue  = u8;
type AsciiValue = char;
type HexMap = HashMap<AddressKey, OneByte>;

#[derive(PartialEq, Eq, Hash)]
pub struct OneByte
{

byte : ByteValue,
ascii : AsciiValue,

}

impl Default for OneByte
{

     fn default() -> Self {

        Self{
            byte    : 0xFF,
            ascii   : 0xFF as char,
        }
    }
}


impl OneByte 
{
    pub fn push(one_byte : u8) -> Self {

        Self{
            byte    : one_byte,
            ascii   : one_byte as char,
        }
    }
}



pub struct HexBuffer
{

    map  : HexMap ,
    size : u32,

}

impl Default for HexBuffer
{

     fn default() -> Self {

        let v: Vec<u8> = vec![0xFF; 0xFFFF]; // default HexBuffer size 0xFFFF, filled 0xFF
        let mut map_default: HexMap = HashMap::new();
        let mut address_index: AddressKey = 0;
        for byte_ in v.iter()
        {
            map_default.insert(address_index, OneByte::push(*byte_));
            address_index += 0x1;
        } 
        
        Self{
            map    : map_default,
            size   : 0xFFFF,
        }
    }
}

impl HexBuffer
{

pub fn init() -> Self
{

Self { map: (HashMap::new()), size: (0) }

}
pub fn fill_buffer(&mut self, buffer : &Vec<u8>, start_address : AddressKey)
{

    let mut address_index = start_address;
    for byte_ in buffer.iter() {

    let value = OneByte::push(*byte_);
    self.map.insert(address_index, value);
    address_index += 0x1;

    }
}


pub fn get_byte_in_address(&mut self, address : AddressKey) -> Result<&OneByte, Error>
{
    println!("Look values at: {:#04X}", address);

    match self.map.get(&address)
    {
        Some(one_byte) => 
        {
        println!("u8 byte on address: {:#02X}", one_byte.byte);
        println!("ascii on address: {}", one_byte.ascii);
        Ok(one_byte)
        }
        _ => 
        {
        println!("AddressKey not found!");
        Err(Error::PowerStateError)
        }
    }
}

}





type SegmentStartAddress = u32;
type SegmentSize = u32;

pub struct MemoryMap
{

flash_start : SegmentStartAddress,
flash_size  : SegmentSize,
    
}
