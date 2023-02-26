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

    buffer     : Vec<Vec<String>>,
}


impl Default for HexBuffer {

        fn default() -> Self {
            Self{

                buffer     : vec![vec!["00 00 00 00 00 00 00 00 00 00 00 00 00 00".to_string()]; 4500]       
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

    let in_string = format!("{:02X}", byte);
 
    one_line_vec.push(in_string);
    if(one_line_vec.len() == 16)
    {
 
    self.buffer.push(one_line_vec.clone());
    one_line_vec.clear();
    
    }  
  } 

    Ok(())
}

 pub fn download_u8(&mut self) -> Vec<u8> {

    let mut byte_vec = Vec::new();
    for str_vec in self.buffer.iter()
    {
        for mut str_ in str_vec.iter()
        {
            byte_vec.append(&mut str_.clone().into_bytes()) 

        }  
    }
    
   byte_vec

}

 pub fn download_string(&self) -> Vec<Vec<String>>{


    self.buffer.clone()


 }
}


pub struct TableContents<Message> {
    item_height:f32,
    contents:Vec<Vec<String>>,
    on_double_clicked: Box<dyn Fn() -> Message>,

}

impl <Message>TableContents<Message> {
    pub fn new(item_height:f32,
        contents:Vec<Vec<String>>,
                on_double_clicked:impl Fn() -> Message + 'static
    ) -> Self {
        Self {item_height,contents,on_double_clicked:Box::new(on_double_clicked)}
    }
}

pub fn table_contents< Message>(item_height:f32,
    contents:Vec<Vec<String>>,
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
        let mut end_y=viewport.y + viewport.height;
        let mut number_of_element = (viewport_layout_y /self.item_height) as i32;
        let mut element_bounds=
        Rectangle
        {
            x:layout.bounds().x, 
            y:(self.item_height*number_of_element as f32)+layout.bounds().y, 
            width:layout.bounds().width, height:self.item_height
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

            let mut text_bounds=element_bounds;
            text_bounds.x + (text_bounds.x * 0.1);
            text_bounds.y=element_bounds.center_y();
       

            if let Some(itemvec)=contents.get(number_of_element as usize) {

            text_bounds.width/=itemvec.len() as f32;
            for item in itemvec.iter(){;

                        renderer.fill_text(
                            iced_native::text::Text {
                                content: format!("{}",item.as_str()).as_str(),
                                bounds: text_bounds,
                                size: 20.0,
                                color: Color::BLACK,
                                font: Font::Default,
                                horizontal_alignment: Horizontal::Left,
                                vertical_alignment: Vertical::Center,
                            });
                            
                        text_bounds.x+=text_bounds.width;
                    }
                }        
           
            element_bounds.y+=element_bounds.height;
            number_of_element+=1;
        }
    }

    
        
    fn layout(&self, renderer: &iced::Renderer, limits: &Limits) -> Node {
        layout::Node::new(Size{
           
            width: limits.max().width,
            height:  self.item_height*((self.contents.len()) as f32)
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


