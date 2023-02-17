use iced::theme::Rule;
use iced::alignment;
use iced::widget::{column, container, image, row, text, vertical_rule, horizontal_rule, Column, Container, Row, Space};
use crate::app::{Message };
use iced::{Length, Color, Renderer};
use iced_aw::Grid;


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
            bytes: vec![0xFF; 0xFE00],
            start_addr: 0x0000,
            end_addr: 0xfe00,
            lenght_ : 0x0F,
            hex_index : 0, 
        }
        
    }

    pub fn set_bytes(&mut self, bytes: Vec<u8>, start_addr: u16, end_addr: u16) {
        self.bytes = bytes;
        self.start_addr = start_addr;
        self.end_addr = end_addr;
    }


    fn title(&self) -> String {
        todo!()
    }

    pub fn width(&self) -> iced::Length {
        iced::Length::Units(20)
    }

    pub fn height(&self) -> iced::Length {
        iced::Length::Units(20)
    }

   // fn layout(&self, _: iced::Layout) -> iced::Layout {
   //     iced::Layout::default()
 //   }

 // .width(Length::FillPortion(2))]

 pub fn preapare_buffer(&mut self) 
 {
    
    let address = &self.start_addr.to_be_bytes();


    self.bytes.insert(0, address[0]);
    self.bytes.insert(1, address[1]);


 }

 pub fn view(&self) -> Column<'static, Message> {

  
 
    let  final_buff = self.one_line_column(self.start_addr);
    

    final_buff


  }

  pub fn buffer(&self) -> Column<'static, Message> {

    let mut buffer_to_row: Vec<u8> = Vec::new();
    buffer_to_row.extend(&self.bytes);

    if(buffer_to_row.len() != usize::from(self.end_addr))
    {
        dbg!(buffer_to_row.len());
    }

    let rows_amount = (self.end_addr - self.start_addr) / 0x0F;

    let mut start_index = self.start_addr;

    let mut ret_colum = Column::new();

    for i in 0..rows_amount
    {
        ret_colum = ret_colum.push(
        self.one_line_column(start_index),
        );
        start_index += self.lenght_;
        
    }

    ret_colum

  }


  pub fn one_line_column(&self, start_address : u16) -> Column<'static, Message> {

    let end_line = start_address + self.lenght_;

  

    let  address_row = self.address_row(start_address);;
    let  hex_row = self.hex_one_row_line_view(start_address, end_line);
    let  ascii_row = self.ascii_one_row_line_view(start_address, end_line);
    let  buffer = column![

        row![
        address_row.height(Length::Shrink).padding(10),
    
        hex_row.height(Length::Shrink).padding(10),
    
        ascii_row.height(Length::Shrink).padding(10),] ];

    buffer


  }


  pub fn demo_row(&self) -> Row<'static, Message> {


    let mut demo_row = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    ///let mut space_start =  Space::new(Length::Units(1), Length::Units(1),);


    Row::with_children(
        demo_row
        .iter()
        .enumerate()
        .map(|( idx, byte, )| {
            container(
                column![row!
                [text(format!("{:02X}", byte)).size(18).style(Color::from([0.05882, 0.72157, 0.10196]))]]
            .height(Length::Units(25)),
            )
        .width(Length::Units(25))// space between bytes in a row Units(20)
        .center_y()
        .into() 
        })       
        .collect(), 
   
      )
    


  }


 pub fn address_row(&self, address : u16) -> Row<'static, Message> {

    let address_slice = &self.start_addr.to_be_bytes();
  
    let adress1: u8 = address_slice[0];
    let adress2: u8 = address_slice[1];

    let mut adress_to_row = Vec::new();
    adress_to_row.push(adress1);
    adress_to_row.push(adress2);

    Row::with_children(
        adress_to_row
        .iter()
        .enumerate()
        .map(|( idx, byte)| {
            container(
                column![row![text(format!("{:02X}", byte)).size(18).style(Color::from([0.05882, 0.72157, 0.10196]))]]
            .height(Length::Units(25)),
            )
        .width(Length::Units(20))// space between bytes in a row Units(20)
        .center_y()
        .into() 
        })       
        .collect(), 
   
      )
    


  }


  fn hex_one_row_line_view(&self, start_line : u16, end_line : u16) -> Row<'static, Message> {

    let start = usize::from(start_line);
    let end =  usize::from(end_line);
 
    let mut buffer_to_row = &self.bytes[start..end].to_vec();

    Row::with_children(
        buffer_to_row
        .iter()
        .enumerate()
        .map(|( idx, byte)| {
            container(
                column![row![text(format!("{:02X}", byte)).size(18)]]
            .height(Length::Units(25)),
            )
        .width(Length::Units(25))// space between bytes in a row Units(20)
        .center_y()
        .into() 
        })       
        .collect(), 
   
      )


  }

 pub fn ascii_one_row_line_view(&self, start_line : u16, end_line : u16) -> Row<'static, Message> {

    let start = usize::from(start_line);
    let end =  usize::from(end_line);

    let mut ascii_row = &self.bytes[start..end].to_vec();

    Row::with_children(
        ascii_row
        .iter()
        .enumerate()
        .map(|( idx, byte)| {
            container(
                row![text(format!("{}", *byte as char)).size(18),
                ]
            .height(Length::Units(25)),
            )
      
        .width(Length::Units(15))// space between bytes in a row Units(20)
        .center_y()
        .into() 
        })       
        .collect(), 
   
      )


  }

  


    fn update(&mut self, _message: Message) -> iced::Command<Message> {
        iced::Command::none()
    }

}
