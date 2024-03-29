use iced::{
    alignment::Horizontal, alignment::Vertical, 
    widget::{Button, Container, Row, Text, Column},
    Alignment, Element, Length, Sandbox, Settings,
    Renderer, 
};
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,scrollable, Tooltip, vertical_space, image, progress_bar
};
use iced_aw::{Modal};
use iced_aw::{style::CardStyles, Card};

use iced_native::widget::tooltip::Position;
use super::styling::{ProgressBarMy, ProgressBarStyle};
use crate::app::{Message, App, TargetStatus};
use crate::dsc_target::target_factory::TargetSelector;
use crate::errors::{Error, get_title_message_error_modal};
use crate::VERSION;
use std::path::PathBuf;

pub fn error_notify_model<'a>(show_error_modal : bool, content: Element<'a, Message, iced::Renderer>, err :  Error) -> Element<'a, Message> {



        let mut error_string =  get_title_message_error_modal(err.clone());

        let title = error_string.0;
        let error_entry = err.to_string().clone();
        let message = error_string.1 + &error_entry;
        

        Modal::new(show_error_modal, content,  move|| {
            Card::new(
                Text::new(title.clone()),
                Text::new(message.clone()),
            )
            .foot(
                Row::new()
                    .spacing(10)
                    .padding(5)
                    .width(Length::Fill)
                    .push(
                        Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                            .width(Length::Fill)
                            .on_press(Message::OkButtonPressed),
                    ),
            )
            .max_width(300.00)
            //.width(Length::Shrink)
            .on_close(Message::OkButtonPressed)
            .into()
        })
        .backdrop(Message::OkButtonPressed)
        .on_esc(Message::OkButtonPressed)
        .into()
}


pub fn nofiy_user_model<'a>(show_notify : bool, content: Element<'a, Message, iced::Renderer>, title : String, msg : String) -> Element<'a, Message> {


    Modal::new(show_notify, content,  move|| {
        Card::new(
            Text::new(title.clone()),
            Text::new(msg.clone()),
        )
        .foot(
            Row::new()
                .spacing(10)
                .padding(5)
                .width(Length::Fill)
                .push(
                    Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                        .width(Length::Fill)
                        .on_press(Message::OkButtonPressedNotify),
                ),
        )
        .max_width(300.00)
        //.width(Length::Shrink)
        .on_close(Message::OkButtonPressedNotify)
        .into()
    })
    .backdrop(Message::OkButtonPressedNotify)
    .on_esc(Message::OkButtonPressedNotify)
    .into()
}




pub fn erase_write_confirm_modal<'a>(show_confirmation : bool, content: Element<'a, Message, iced::Renderer>, status : TargetStatus) -> Element<'a, Message> {


    let mut title   =  String::new();
    let mut message =  String::new();

    match status
    {

        TargetStatus::InProgrammingWrite => 
        {
         
            title   = "Confirm Write Target".to_string();
            message = (format!("Are you sure you want write target?"));
   
   
        }
        TargetStatus::InProgrammingErase=> 
        {
         
            title   = "Confirm Erase Target".to_string();
            message = (format!("This action will erase all flash memory"));
   
        }
        _ =>
        {
         
            title   = "Empry Confirmation! Please Report".to_string();
            message = ":\n".to_string();
   
   
        }
    }
    

    Modal::new(show_confirmation, content,  move|| {
        Card::new(
            Text::new(title.clone()),
            Text::new(message.clone()),
        )
        .foot(
            Row::new()
                .spacing(10)
                .padding(5)
                .width(Length::Fill)
                .push(
                    Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                        .width(Length::Fill)
                        .on_press(Message::WriteEraseConfirmation(true)),
                )
                .push(
                    Button::new(Text::new("Cancel").horizontal_alignment(Horizontal::Center))
                        .width(Length::Fill)
                        .on_press(Message::WriteEraseConfirmation(false)),
                ),
        )
        .max_width(300.00)
        //.width(Length::Shrink)
        .on_close(Message::WriteEraseConfirmation(false))
        .into()
    })
    .into()
}


pub fn about_card<'a>(show_about_card : bool, content: Element<'a, Message, iced::Renderer>, )  -> Element<'a, Message>
{

    Modal::new(show_about_card, content,  move|| { Card::new(
        Text::new("USBDM_rs").size(25),
        Column::new()
            //.push(Text::new("Body").size(42))
            .push(Text::new("DSC (mc56f80xx) programmer").size(15))
            .push(vertical_space(5.0))
            .push(Text::new("License: GPL v2").size(15))
            .push(vertical_space(5.0))
            .push(Row::new().push(Text::new("Version: ").size(15)).push(Text::new(VERSION.to_string())))
            .push(vertical_space(5.0))
            .push( Text::new("Author Kuraga ").size(15))
            .push(vertical_space(5.0))
            .push(Row::new()
             .spacing(10)
             .push(Text::new("Project rep:"))
             .push(get_button_github()))
             .push(vertical_space(5.0))
    )   
      .foot(
        Row::new()
            .spacing(10)
            .padding(5)
            .width(Length::Fill)
            
            .push(
                Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                    .width(Length::Fill)
                    .on_press(Message::CloseAboutCard),
            ),
      )
      .on_close(Message::CloseAboutCard)
      .max_width(300.00)
      //.width(Length::Shrink)
      .on_close(Message::CloseAboutCard)
      .into() })
    .backdrop(Message::CloseAboutCard)
    .on_esc(Message::CloseAboutCard)
    .into()
    

}






pub fn get_button_github() -> Tooltip<'static, Message> {
    let content = button(
        Text::new("usbdm_mc56f_rs".to_string())
           // .font(ICONS)
            .size(15)
           .horizontal_alignment(Horizontal::Center)
           .vertical_alignment(Vertical::Center),
    )
   // .height(Length::Fixed(45.0))
    .width(Length::Fixed(100.0))
    .on_press(Message::OpenGithub);

    let tool_tip = Tooltip::new(content, "Github", Position::Right);

    tool_tip

}

pub fn get_kuraga_remont_ru() -> Tooltip<'static, Message> {
    let content = button(
        Text::new("Site".to_string())
           // .font(ICONS)
            .size(15)
           .horizontal_alignment(Horizontal::Center)
           .vertical_alignment(Vertical::Center),
    )
   // .height(Length::Fixed(45.0))
    .width(Length::Fixed(50.0))
    .on_press(Message::OpenGithub);

    let tool_tip = Tooltip::new(content, "kuraga-remont.ru", Position::Right);

    tool_tip

}
//static DSC_56F8035_IMAGE: &[u8] = include_bytes!("../gui/resources/mcu_connection.jpeg");
//let handle = image::Handle::from_memory(DSC_56F8035_IMAGE);

pub fn connection_image_modal<'a>(width: u16, show_conn_image : bool, content: Element<'a, Message, iced::Renderer>, img_path : PathBuf, target_name : String )  -> Element<'a, Message>
{
    
    let title =  "Connection of ".to_string() + & target_name.to_uppercase(); 
    
    Modal::new(show_conn_image, content,  move|| { 

    
        let handle = image::Handle::from_path(img_path.clone());

        Card::new( 
        Text::new(title.clone()).size(25),
        container(image(handle).width(width)).center_x()
       )
      .foot(
        Row::new()
            .spacing(10)
            .padding(5)
            .width(Length::Fill)
      )
      .on_close(Message::ConnectionImageOpen(false))
      .max_width(1500.00)
      //.width(Length::Shrink)
      .on_close(Message::ConnectionImageOpen(false))
      .into() })
    .backdrop(Message::ConnectionImageOpen(false))
    .on_esc(Message::ConnectionImageOpen(false))
    .into()
    

}

pub fn progress_bar_modal<'a>(target_programming : bool, content: Element<'a, Message, iced::Renderer>, prg_value : f32, status : TargetStatus )  -> Element<'a, Message>
{

    let mut title   =  String::new();
    let mut message =  String::new();

    match status
    {
        TargetStatus::InProgrammingRead =>       
        {
         
         title   = "Read Target".to_string();
         message = (format!("reading target... {prg_value:.2}%"));


         }

        TargetStatus::InProgrammingWrite => 
        {
         
            title   = "Write Target".to_string();
            message = (format!("writing target... {prg_value:.2}%"));
   
   
        }
        TargetStatus::InProgrammingVerify=> 
        {
         
            title   = "Verify Target".to_string();
            message = (format!("verify target with buffer... {prg_value:.2}%"));
   
   
        }
        TargetStatus::InProgrammingErase=> 
        {
         
            title   = "Erase Target".to_string();
            message = (format!("erase... {prg_value:.2}%"));
   
   
        }
        _ =>
        {
         
            title   = "Programming end".to_string();
            message = ":\n".to_string();
   
   
        }
    }

    Modal::new(target_programming, content,  move|| { 
        let progress_bar = progress_bar(0.0..=100.0, prg_value).height(Length::Fixed(20.0));
        Card::new(
        Text::new(title.clone()).size(15).horizontal_alignment(Horizontal::Center),
        Row::new()
        .spacing(10)
        .padding(10)
        .align_items(Alignment::Center)
        .push(text(message.clone()))
        .push(progress_bar)
    )   
      .foot(
        Row::new()
            .width(Length::Shrink),
      )
      .on_close(Message::TargetProgramminEnd)
      .max_width(300.00)
      //.width(Length::Shrink)
      .on_close(Message::TargetProgramminEnd)
      .into() })
    .backdrop(Message::TargetProgramminEnd)
    .on_esc(Message::TargetProgramminEnd)
    .into()

}