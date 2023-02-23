use iced::{
    alignment::Horizontal,
    widget::{Button, Container, Row, Text},
    Alignment, Element, Length, Sandbox, Settings,
    Renderer,
};

use iced_aw::{Card, Modal};

use crate::app::{Message, App};
use crate::errors::{Error, get_title_message_error_modal};


pub fn error_notify_model<'a>(show_error_modal : bool, content: Element<'a, Message, iced::Renderer>, err :  Error) -> Element<'a, Message> {



        let mut error_string =  get_title_message_error_modal(err);

        let title = error_string.0;
        let error_entry = err.to_string();
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

