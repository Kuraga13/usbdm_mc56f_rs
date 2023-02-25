use iced::{
    alignment::Horizontal, alignment::Vertical, 
    widget::{Button, Container, Row, Text, Column},
    Alignment, Element, Length, Sandbox, Settings,
    Renderer, 
};
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,scrollable, Tooltip, vertical_space, 
};
use iced_aw::{Modal};
use iced_aw::{style::CardStyles, Card};

use iced_native::widget::tooltip::Position;

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
            .push( Text::new("Author Kuraga").size(15))
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
      .style(CardStyles::Primary)
      .on_close(Message::CloseAboutCard)
      .max_width(300.00)
      //.width(Length::Shrink)
      .on_close(Message::CloseAboutCard)
      .into() })
    .backdrop(Message::CloseAboutCard)
    .on_esc(Message::CloseAboutCard)
    .into()
    

}





 /* 
pub const ICONS: Font = Font::External {
    name: "icons",
    bytes: include_bytes!("../../resources/icons.ttf"),
};
*/

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
   // .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::OpenGithub);

    let tool_tip = Tooltip::new(content, "Github", Position::Right);

    tool_tip

}










pub fn error_notify_model_from_iced<'a>(show_error_modal : bool, content: Element<'a, Message, iced::Renderer>, err :  Error) -> Element<'a, Message> {

    if show_error_modal{

    use self::modal::Modal;
    let mut error_string =  get_title_message_error_modal(err);

    let title = error_string.0;
    let error_entry = err.to_string();
    let message = error_string.1 + &error_entry;

     let modal_ =  Card::new(
    Text::new(title.clone()),
    Text::new(message.clone()),)
    .foot(
     Row::new()
        .spacing(10)
        .padding(5)
        .width(Length::Fill)
        .push(
            Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
                .width(Length::Fill)
                .on_press(Message::OkButtonPressed), ),)
        .max_width(300.00);
        //.width(Length::Shrink)


       Modal::new(content, modal_)
            .on_blur(Message::OkButtonPressed)
            .into()
        }
        else { content.into() }
        

}

mod modal {
    use iced_native::alignment::Alignment;
    use iced_native::widget::{self, Tree};
    use iced_native::{
        event, layout, mouse, overlay, renderer, Clipboard, Color, Element,
        Event, Layout, Length, Point, Rectangle, Shell, Size, Widget,
    };

    /// A widget that centers a modal element over some base element
    pub struct Modal<'a, Message, Renderer> {
        base: Element<'a, Message, Renderer>,
        modal: Element<'a, Message, Renderer>,
        on_blur: Option<Message>,
    }

    impl<'a, Message, Renderer> Modal<'a, Message, Renderer> {
        /// Returns a new [`Modal`]
        pub fn new(
            base: impl Into<Element<'a, Message, Renderer>>,
            modal: impl Into<Element<'a, Message, Renderer>>,
        ) -> Self {
            Self {
                base: base.into(),
                modal: modal.into(),
                on_blur: None,
            }
        }

        /// Sets the message that will be produces when the background
        /// of the [`Modal`] is pressed
        pub fn on_blur(self, on_blur: Message) -> Self {
            Self {
                on_blur: Some(on_blur),
                ..self
            }
        }
    }

    impl<'a, Message, Renderer> Widget<Message, Renderer>
        for Modal<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
        Message: Clone,
    {
        fn children(&self) -> Vec<Tree> {
            vec![Tree::new(&self.base), Tree::new(&self.modal)]
        }

        fn diff(&self, tree: &mut Tree) {
            tree.diff_children(&[&self.base, &self.modal]);
        }

        fn width(&self) -> Length {
            self.base.as_widget().width()
        }

        fn height(&self) -> Length {
            self.base.as_widget().height()
        }

        fn layout(
            &self,
            renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            self.base.as_widget().layout(renderer, limits)
        }

        fn on_event(
            &mut self,
            state: &mut Tree,
            event: Event,
            layout: Layout<'_>,
            cursor_position: Point,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
        ) -> event::Status {
            self.base.as_widget_mut().on_event(
                &mut state.children[0],
                event,
                layout,
                cursor_position,
                renderer,
                clipboard,
                shell,
            )
        }

        fn draw(
            &self,
            state: &Tree,
            renderer: &mut Renderer,
            theme: &<Renderer as iced_native::Renderer>::Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor_position: Point,
            viewport: &Rectangle,
        ) {
            self.base.as_widget().draw(
                &state.children[0],
                renderer,
                theme,
                style,
                layout,
                cursor_position,
                viewport,
            );
        }

        fn overlay<'b>(
            &'b mut self,
            state: &'b mut Tree,
            layout: Layout<'_>,
            _renderer: &Renderer,
        ) -> Option<overlay::Element<'b, Message, Renderer>> {
            Some(overlay::Element::new(
                layout.position(),
                Box::new(Overlay {
                    content: &mut self.modal,
                    tree: &mut state.children[1],
                    size: layout.bounds().size(),
                    on_blur: self.on_blur.clone(),
                }),
            ))
        }

        fn mouse_interaction(
            &self,
            state: &Tree,
            layout: Layout<'_>,
            cursor_position: Point,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.base.as_widget().mouse_interaction(
                &state.children[0],
                layout,
                cursor_position,
                viewport,
                renderer,
            )
        }

        fn operate(
            &self,
            state: &mut Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn widget::Operation<Message>,
        ) {
            self.base.as_widget().operate(
                &mut state.children[0],
                layout,
                renderer,
                operation,
            );
        }
    }

    struct Overlay<'a, 'b, Message, Renderer> {
        content: &'b mut Element<'a, Message, Renderer>,
        tree: &'b mut Tree,
        size: Size,
        on_blur: Option<Message>,
    }

    impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
        for Overlay<'a, 'b, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
        Message: Clone,
    {
        fn layout(
            &self,
            renderer: &Renderer,
            _bounds: Size,
            position: Point,
        ) -> layout::Node {
            let limits = layout::Limits::new(Size::ZERO, self.size)
                .width(Length::Fill)
                .height(Length::Fill);

            let mut child = self.content.as_widget().layout(renderer, &limits);
            child.align(Alignment::Center, Alignment::Center, limits.max());

            let mut node = layout::Node::with_children(self.size, vec![child]);
            node.move_to(position);

            node
        }

        fn on_event(
            &mut self,
            event: Event,
            layout: Layout<'_>,
            cursor_position: Point,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
        ) -> event::Status {
            let content_bounds = layout.children().next().unwrap().bounds();

            if let Some(message) = self.on_blur.as_ref() {
                if let Event::Mouse(mouse::Event::ButtonPressed(
                    mouse::Button::Left,
                )) = &event
                {
                    if !content_bounds.contains(cursor_position) {
                        shell.publish(message.clone());
                        return event::Status::Captured;
                    }
                }
            }

            self.content.as_widget_mut().on_event(
                self.tree,
                event,
                layout.children().next().unwrap(),
                cursor_position,
                renderer,
                clipboard,
                shell,
            )
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            theme: &Renderer::Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor_position: Point,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: renderer::BorderRadius::from(0.0),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                Color {
                    a: 0.80,
                    ..Color::BLACK
                },
            );

            self.content.as_widget().draw(
                self.tree,
                renderer,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor_position,
                &layout.bounds(),
            );
        }

        fn operate(
            &mut self,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn widget::Operation<Message>,
        ) {
            self.content.as_widget().operate(
                self.tree,
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        }

        fn mouse_interaction(
            &self,
            layout: Layout<'_>,
            cursor_position: Point,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.content.as_widget().mouse_interaction(
                self.tree,
                layout.children().next().unwrap(),
                cursor_position,
                viewport,
                renderer,
            )
        }
    }

    impl<'a, Message, Renderer> From<Modal<'a, Message, Renderer>>
        for Element<'a, Message, Renderer>
    where
        Renderer: 'a + iced_native::Renderer,
        Message: 'a + Clone,
    {
        fn from(modal: Modal<'a, Message, Renderer>) -> Self {
            Element::new(modal)
        }
    }
}