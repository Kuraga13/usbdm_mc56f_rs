use iced::theme::{self, Theme};
use iced::theme::Rule;
use iced::alignment;
use iced::widget::{text_input, column, container, image, row, text, vertical_rule, horizontal_rule, Column, Container, Row, Space, Text};
use crate::app::{Message };
use iced::{ Length, Color, Renderer, Alignment, Element};
use iced_aw::Grid;


const COLUMNS: usize = 3;
pub struct HexBuffer {
 pub   bytes: Vec<u8>,
 pub   start_addr: u16,
 pub   end_addr: u16,
 pub   lenght_: u16,
 pub   hex_index: usize,

 
}
impl HexBuffer
{

    pub fn new() -> Self {
        HexBuffer {
            bytes: vec![0xFF; 0xFFFF],
            start_addr: 0x0000,
            end_addr: 0x2000,
            lenght_ : 0x0F,
            hex_index : 0x0F, 
            //buffer_builded : self.
        }
        
    }

    pub fn set_bytes(&mut self, bytes: Vec<u8>, start_addr: u16, end_addr: u16) {
        self.bytes = bytes;
        self.start_addr = start_addr;
        self.end_addr = end_addr;
    }
}


pub fn build_buffer<'a>(buff : &HexBuffer) -> Element<'a, Message, iced::Renderer<Theme>> {


    let mut grid = Grid::with_columns(COLUMNS)
    .push(Text::new("").style(theme::Text::Color(Color::from([0.05882, 0.72157, 0.10196]))))
    .push(Text::new("01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F").style(theme::Text::Color(Color::from([0.05882, 0.72157, 0.10196]))))
    .push(Text::new("ASCII").style(theme::Text::Color(Color::from([0.05882, 0.72157, 0.10196]))));
   // Add elements to the grid

    let mut rows_amount = (buff.end_addr - buff.start_addr) / buff.lenght_;

    //let hex_index = usize::from(rows_amount);
    let hex_index = 10;
    dbg!(hex_index);


    for i in 0..hex_index {
    grid.insert(Text::new(format!("Row {}, Test", (1))));
    grid.insert(Text::new(format!("Row {}, Test", (2))));
    grid.insert(Text::new(format!("Row {}, Test", (3))));
    }
 
    let  final_buff = Column::new()
    .spacing(15)
    .max_width(600)
    .padding(10)
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .push(grid)
    .into();
    

    final_buff
}





pub struct HexBufferView
{

pub   buffer_builded     : Element<'static, Message, iced::Renderer<Theme>>, 
      hex_buffer         :    HexBuffer,

}

impl HexBufferView
{

    pub fn new() -> Self {

        let buffer = HexBuffer::new();

        HexBufferView {
            buffer_builded : build_buffer(&buffer),
            hex_buffer : buffer,
        }   
    }

    pub fn view<'a>(&self) -> Element<'a, Message, iced::Renderer<Theme>> {


    
        
        //self.buffer_builded

        Container::new(build_buffer(&self.hex_buffer))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(1)
        .into()
    
      }


      
  pub fn address_row_line(&self, address : u16) -> Vec<String> {

    let address_slice = &self.hex_buffer.start_addr.to_be_bytes();
  
    let adress1: String = format!("{:02X}", address_slice[0]);
    let adress2: String = format!("{:02X}", address_slice[1]);

    let mut adress_to_row = Vec::new();


    adress_to_row.push(adress1);
    adress_to_row.push(adress2);

    adress_to_row
    

  }

  pub fn hex_row_line(&self, address : u16) -> Vec<u8> {

    let start = usize::from(address);
    let end =  usize::from(address + self.hex_buffer.lenght_);
 
    let mut buffer_to_row = &self.hex_buffer.bytes[start..end].to_vec();

    let fin = buffer_to_row.clone();

    fin
    

  }


  pub fn ascii_line_row(&self, address : u16) -> Vec<String> {

    let start = usize::from(address);
    let end =  usize::from(address + self.hex_buffer.lenght_);
 
    let mut buffer_to_row = &self.hex_buffer.bytes[start..end].to_vec();
    let mut vec_str = Vec::new();

    for byte in buffer_to_row.iter() {
        
        vec_str.push(format!("{:02X}", byte));
    }

    vec_str

  }
    
    
}











