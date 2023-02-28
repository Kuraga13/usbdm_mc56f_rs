use iced_native::layout::{Node, Limits};
use iced_native::widget::{Tree, Text};
use iced_native::{Color, Element, Length, Point, Rectangle, Size, Clipboard, Shell};
use iced_native::event::Status;
use iced_native::Background;
use iced::{alignment, alignment::Horizontal,alignment::Vertical };
use iced_native::{
    widget,Font,
    layout,
    renderer::{self, Style},
    Renderer, Widget,Layout
};

use crate::app::{Message };
use crate::errors::Error;

pub struct HexBuffer
{

    buffer     : Vec<Vec<u8>>,
}


impl Default for HexBuffer {
    fn default() -> Self {
        Self{
            buffer     : vec![vec![0xFF; 16]; 0x2001]    
        }
    }
}

impl HexBuffer
{
    pub fn new(start : usize, end : u32) -> Self {
  
        Self{
            ..Default::default()
    }
  }


 pub fn upload(&mut self, new_buff : Vec<u8>) -> Result<(), Error> {


  self.buffer.clear();

  let mut one_line_vec = Vec::new();

     for byte in new_buff.iter() {

       one_line_vec.push(*byte);

       if(one_line_vec.len() == 16) {

       self.buffer.push(one_line_vec.clone());
       one_line_vec.clear(); 

      }  
    } 

    Ok(())
}

 pub fn download_in_one(&mut self) -> Vec<u8> {


    
    let mut byte_vec = Vec::new();
    for one_line_vec in self.buffer.clone().iter()
    {
        for mut one_byte in one_line_vec.iter()
        {
            byte_vec.push(* one_byte) 

        }  
    }
    
   byte_vec

}

pub fn download_string(&self) -> Vec<Vec<String>> {



    let mut all_string_buffer = Vec::new();
    let mut one_line_string = Vec::new();
  
    for one_line_vec in self.buffer.iter() {

        for byte in one_line_vec.iter()
        {
           let in_string =   format!("{:02X?}", byte);
           one_line_string.push(in_string);  
              
           if(one_line_string.len() == 16)
           {
            all_string_buffer.push(one_line_string.clone());
            one_line_string.clear(); 
           }  
        }
    } 
  
    all_string_buffer.clone()

}

pub fn download_all_u8(&self) -> Vec<Vec<u8>> {


    
  self.buffer.clone()
  

}


}


pub const FONT_BYTES :  &[u8] = include_bytes!("./fonts/CourierNewPS-BoldMT.ttf");
static FONT_NAME : &str =  "CourierNewPS-BoldMT";


pub struct TableContents<Message> {
    item_height:f32,
    contents:Vec<Vec<u8>>,
    on_double_clicked: Box<dyn Fn() -> Message>,
    address : u32,

}

impl <Message>TableContents<Message> {
    pub fn new(item_height:f32,
        contents:Vec<Vec<u8>>,
                on_double_clicked:impl Fn() -> Message + 'static
    ) -> Self {
        Self {item_height,contents,on_double_clicked:Box::new(on_double_clicked), address : 0,}
    }
}

pub fn table_contents< Message>(item_height:f32,
    contents:Vec<Vec<u8>>,
                              on_double_clicked:impl Fn() -> Message + 'static) -> TableContents<Message> {
    TableContents::new(item_height,contents,on_double_clicked)
}

impl<Message:std::clone::Clone> Widget<Message, iced::Renderer> for TableContents<Message>  

{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Fill
    }
    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>) -> Status {
        Status::Ignored
    }

    fn draw(
        & self,
        _state: &widget::Tree,
        renderer: &mut iced::Renderer,
        _theme: &iced::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        viewport: &Rectangle,
    ) {
        use iced_native::text::Renderer as text_renderer;
        let mut viewport_layout_y=(viewport.y-layout.bounds().y);

        let mut offset_first_line_x = viewport.x + 20.0;


        let mut end_y=viewport.y + viewport.height;
        let mut number_of_element: u32 = (viewport_layout_y / self.item_height) as u32;
        let mut element_bounds=
        Rectangle
        {
            x: layout.bounds().x, 
            y:(self.item_height*number_of_element as f32)+layout.bounds().y, 
            width:layout.bounds().width, 
            height:self.item_height 
        };
        let mut contents=self.contents.clone();


        while element_bounds.y < end_y{
            let mut rectangle_bounds = element_bounds;
            if element_bounds.y + element_bounds.height > end_y {
                rectangle_bounds.height = end_y - element_bounds.y;
            }

            renderer.fill_quad(
                renderer::Quad {
                    bounds: rectangle_bounds,
                    border_radius: Default::default(),
                    border_width: 0.0,
                    border_color: Color::WHITE,
                }, Background::Color(Color::WHITE)
            );

            let mut adress_bounds=element_bounds;
            adress_bounds.x += offset_first_line_x;
            let offset_fisrt_line_y = element_bounds.center_y() + 10.00;
            adress_bounds.y=offset_fisrt_line_y;

            let mut text_bounds=adress_bounds;
            text_bounds.x = adress_bounds.x * 3.9;
        

            let mut ascii_bounds=text_bounds;
            ascii_bounds.x = text_bounds.x * 5.5;
     

            let mut is_new_line = true;
      
            if let Some(itemvec) = contents.get(number_of_element as usize) {
                text_bounds.width  = 21.0;
                ascii_bounds.width = 10.0;
          
                for item in itemvec.iter(){
                    let ascii = item.clone();
                    let hex_byte    = item.clone();
                    let address = number_of_element * 0x10;

                    if(is_new_line) {
                        renderer.fill_text(
                            iced_native::text::Text {
                                content: format!("{:05X?}", address).as_str(),
                                bounds: adress_bounds,
                                size: 15.0,
                                color: Color::from_rgb8(8, 54, 191),
                                font: Font::External { name : FONT_NAME, bytes : FONT_BYTES},
                                horizontal_alignment: Horizontal::Left,
                                vertical_alignment: Vertical::Center,});
                        is_new_line = false;
                    };

                    renderer.fill_text(
                        iced_native::text::Text {
                            content:  format!("{:02X?}", hex_byte).as_str(),
                            bounds: text_bounds,
                            size: 15.0,
                            color: Color::BLACK,
                            font: Font::External { name : FONT_NAME, bytes : FONT_BYTES},
                            horizontal_alignment: Horizontal::Left,
                            vertical_alignment: Vertical::Center,});
                        
                    renderer.fill_text(
                        iced_native::text::Text {
                            content: &{if (ascii >= 33 && ascii <= 126) {(ascii as char)} else {'.'}}.to_string(),
                            bounds: ascii_bounds,
                            size: 13.0,
                            color: Color::from_rgb8(8, 54, 191),
                            font: Font::External { name : FONT_NAME, bytes : FONT_BYTES},
                            horizontal_alignment: Horizontal::Left,
                            vertical_alignment: Vertical::Center,});
                            
                    text_bounds.x+=text_bounds.width; // add one element
                    ascii_bounds.x+=ascii_bounds.width; // add one element
                }
            }        
            
            is_new_line = true;
            element_bounds.y+=element_bounds.height;
            number_of_element+=1;
        }
    }

    
        
    fn layout(&self, renderer: &iced::Renderer, limits: &Limits) -> Node {
        layout::Node::new(Size{
           
            width: limits.max().width,
            height:  (self.item_height*((self.contents.len() + 1)) as f32)
        })
    }

}

impl<'a, Message> From<TableContents<Message>> for Element< 'a,Message, iced::Renderer>
    where Message:'a,
          Message:std::clone::Clone
{
    fn from(table_contents: TableContents<Message>) -> Self {
        Self::new(table_contents)
    }
}

