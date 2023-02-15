use iced::theme::Rule;
use iced::alignment;
use iced::widget::{column, container, image, row, text, Column, Container, Row, Space};
use crate::app::{Message };
use iced::{Length, Renderer};



pub struct HexBuffer {
 pub   bytes: Vec<u8>,
 pub   start_addr: u16,
 pub   end_addr: u16,
 pub   lenght_: u16,
}

impl HexBuffer
{

    pub fn new() -> Self {
        HexBuffer {
            bytes: vec![0xFF; 0x0F],
            start_addr: 0x0000,
            end_addr: 0xfe00,
            lenght_ : 0x0F,
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

    dbg!(address[0]);
    dbg!(address[1]);

    self.bytes.insert(0, address[0]);
    self.bytes.insert(1, address[1]);


 }

 pub fn view(&self) -> Row<'static, Message> {

  

    let  final_row = self.hex_one_row_line_view();


    final_row


  }

  fn hex_one_row_line_view(&self) -> Row<'static, Message> {

    let address_slice = &self.start_addr.to_be_bytes();
  
    let adress1: u8 = address_slice[0];
    let adress2: u8 = address_slice[1];

    let mut adress_to_row = Vec::new();
    adress_to_row.push(adress1);
    adress_to_row.push(adress2);
    adress_to_row.extend(&self.bytes);

    Row::with_children(
        adress_to_row
        .iter()
        .enumerate()
        .map(|( idx, byte)| {
            container(
                column![row![text(format!("{:02X}", byte)).size(18)]]
            .align_items(iced::Alignment::Center)
            .height(Length::Units(25)),
            )
        .width(Length::Units(25))// space between bytes in a row Units(20)
        .center_y()
        .into() 
        })       
        .collect(), 
   
      )


  }

 pub fn ascii_one_row_line_view(&self) -> Row<'static, Message> {

    let mut ascii_row: Vec<u8> = Vec::new();
    ascii_row.extend(&self.bytes);

    Row::with_children(
        ascii_row
        .iter()
        .enumerate()
        .map(|( idx, byte)| {
            container(
                column![row![text(format!("{}", *byte as char)).size(18)]]
            .align_items(iced::Alignment::Center)
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
