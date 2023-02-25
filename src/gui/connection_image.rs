use iced::theme;
use iced::widget::{checkbox, column, container, svg, Container, image};
use iced::{color, Element, Length, Sandbox, Settings, Renderer};
use crate::app::{Message};


pub fn dsc_connection_image<'a>(width: u16) -> Container<'a, Message> {

   let content = container(
        // This should go away once we unify resource loading on native
        // platforms
        image("src/resources/mcu_connection.png")
        .width(width),
    )
    .width(Length::Fill)
    .center_x();

    content


}