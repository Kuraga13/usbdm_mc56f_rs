use iced::{
    alignment::Horizontal,
    widget::{Button, Container, Row, Text},
    Alignment, Element, Length, Sandbox, Settings,
    Renderer,
};

use iced_aw::{Card, Modal};

use crate::app::{Message, App};
use crate::errors::{Error};


pub fn error_notify_model<'a>(show_error_modal : bool, content: Element<'a, Message, iced::Renderer>, err :  Error) -> Element<'a, Message> {



        Modal::new(show_error_modal, content,  move|| {
            Card::new(
                Text::new("Error:".to_string()),
                Text::new(err.to_string()),
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

