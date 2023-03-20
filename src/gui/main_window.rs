use iced::theme::Scrollable;
use iced::widget::{column as col, Column};
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,Row, Container, scrollable
};
use iced::{alignment, theme, Application, Color, Element, Length};

use iced_aw::menu::{ItemHeight, ItemWidth, MenuBar, MenuTree, PathHighlight};
use iced_aw::quad;

use crate::errors::{Error};
use crate::app::{Message, App, UsbdmAppStatus, TargetStatus};
use crate::usbdm::settings::{TargetVddSelect};
use crate::usbdm::feedback::{PowerStatus};
use crate::dsc_target::target_factory::{TargetSelector};
use super::styling::{PowerButtonStyle, ButtonStyle, EnablePowerButtonStyle};

use super::hexbuffer_widget::{TableContents,table_contents };

impl TargetVddSelect {
    pub const ALL: [TargetVddSelect; 2] = [
        TargetVddSelect::Vdd3V3,
        TargetVddSelect::Vdd5V,

    ];
}





pub fn main_page<'a>(_app: &App) -> Column<'a, Message, iced::Renderer>
{
    let pick_list_power = pick_list(
        &TargetVddSelect::ALL[..],
        Some(_app.selected_power),
        Message::PowerSelect,
    );

    let set_power_button =  power_button(&_app.status, &_app.power_status);
    

    let mb = MenuBar::new(vec![
            file_system_menu(_app),
            programmer_actions_menu(_app),
            information_menu(_app),
            view_customization_menu(_app),
            target_selection_menu(_app),
        ])
        .item_width(ItemWidth::Static(180))
        .item_height(ItemHeight::Static(25))
    .spacing(4.0)
    .bounds_expand(30)
    .path_highlight(Some(PathHighlight::MenuActive));

    let r = row!(mb, horizontal_space(Length::Fill),  pick_list_power, horizontal_space(Length::Fixed(3.0)), set_power_button)
        .padding([2, 8])
        .align_items(alignment::Alignment::Center);
  

    let top_bar_style: fn(&iced::Theme) -> container::Appearance =
            |_theme| container::Appearance {
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            };

    let top_bar = container(r).width(Length::Fill).style(top_bar_style);



    let test_test_line   = vec![vec!["test_test1".to_string(), "test_test2".to_string(), "test_test3".to_string(),]; 4500];
    let test_addr_line   = vec![vec!["01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F".to_string(), "test_test2".to_string(),]; 4500];
    

    let table_test = table_contents(20.00, _app.buffer.download_all_u8(), || test_buffer_double_click() );
    
    let test_test = scrollable(Container::new(table_test).align_y(alignment::Vertical::Center));
    
    let body = test_test;


    let c = col![top_bar, body];
    

    c        


         
 
         
         
}

pub fn test_buffer_double_click() ->  Message
{

Message::TestBufferDoubleClick

}





pub fn power_button<'a>( state : &UsbdmAppStatus, power_state : &PowerStatus) -> button::Button<'a, Message, iced::Renderer> {

    let power_button = match state
    {
       UsbdmAppStatus::NotConnected      =>  
       {

        empty_labeled_button("VDD").style(iced::theme::Button::Custom(Box::new(PowerButtonStyle {})))
       
       }  
          
       UsbdmAppStatus::Connected  =>  
       {
        match power_state
        {
         PowerStatus::PowerOn  =>  
         {

            labeled_button("VDD", Message::PowerToggle).style(iced::theme::Button::Custom(Box::new(EnablePowerButtonStyle {})))

         }
         PowerStatus::PowerOff  => 
         {

            labeled_button("VDD", Message::PowerToggle).style(iced::theme::Button::Custom(Box::new(PowerButtonStyle {})))

         } 
        }
       }

       UsbdmAppStatus::Errored    => 
       {

        empty_labeled_button("VDD").style(iced::theme::Button::Custom(Box::new(PowerButtonStyle {})))

       }
    };

    power_button

}



pub fn base_button<'a>(
    content: impl Into<Element<'a, Message, iced::Renderer>>,
    msg: Message,
) -> button::Button<'a, Message, iced::Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        .on_press(msg)
}

pub fn base_button_empty<'a>(
    content: impl Into<Element<'a, Message, iced::Renderer>>,
) -> button::Button<'a, Message, iced::Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        
}

pub fn labeled_button<'a>(label: &str, msg: Message) -> button::Button<'a, Message, iced::Renderer> {


    base_button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),

        msg,)

}


pub fn empty_labeled_button<'a>(label: &str) -> button::Button<'a, Message, iced::Renderer> {


    base_button_empty(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),)

}

pub fn menu_button<'a>(label: &str) -> button::Button<'a, Message, iced::Renderer> {
    labeled_button(label, Message::Menu)
}

pub fn about_button_item<'a>(label: &str, msg : Message) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(labeled_button(label, msg).width(Length::Fill).height(Length::Fill))
}

pub fn connect_button_item<'a>(label: &str, msg : Message) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(labeled_button(label, msg).width(Length::Fill).height(Length::Fill))
}

pub fn target_button_item<'a>(label: &str, msg : Message) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(labeled_button(label, msg).width(Length::Fill).height(Length::Fill))
}

pub fn file_button_item<'a>(label: &str, msg : Message) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(labeled_button(label, msg).width(Length::Fill).height(Length::Fill))
}

pub fn programmer_button_item<'a>(label: &str, msg : Message, state : &UsbdmAppStatus, target_state : &TargetStatus) -> MenuTree<'a, Message, iced::Renderer> {

    match state
    {
    
    UsbdmAppStatus::Connected =>
    {
        match target_state
        {
            TargetStatus::Connected => 
            {
                MenuTree::new(labeled_button(label, msg).width(Length::Fill).height(Length::Fill))
            }

            _ =>
            {
              MenuTree::new(empty_labeled_button(label).width(Length::Fill).height(Length::Fill))
            }

        }
        
    }

    UsbdmAppStatus::NotConnected =>
    {
        MenuTree::new(empty_labeled_button(label).width(Length::Fill).height(Length::Fill))
    }
    
    _ => 
    {
        MenuTree::new(empty_labeled_button(label).width(Length::Fill).height(Length::Fill))
    }

    }
    
}

pub fn menu_item<'a>(label: &str) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(menu_button(label).width(Length::Fill).height(Length::Fill))
}

pub fn empty_item<'a>(label: &str) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(empty_labeled_button(label).width(Length::Fill).height(Length::Fill))
}


pub fn color_item<'a>(color: impl Into<Color>) -> MenuTree<'a, Message, iced::Renderer> {
    let color = color.into();
    MenuTree::new(base_button(circle(color), Message::ColorChange(color)))
}

pub fn sub_menu<'a>(
    label: &str,
    msg: Message,
    children: Vec<MenuTree<'a, Message, iced::Renderer>>,
) -> MenuTree<'a, Message, iced::Renderer> {
    let handle = svg::Handle::from_path(format!(
        "{}/caret-right-fill.svg",
        env!("CARGO_MANIFEST_DIR")
    ));
    let arrow = svg(handle)
        .width(Length::Shrink)
        .style(theme::Svg::custom_fn(|theme| svg::Appearance {
            color: Some(theme.extended_palette().background.base.text),
        }));

    MenuTree::with_children(
        base_button(
            row![
                text(label)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .vertical_alignment(alignment::Vertical::Center),
                arrow
            ],
            msg,
        )
        .width(Length::Fill)
        .height(Length::Fill),
        children,
    )
}



pub fn separator<'a>() -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(quad::Quad {
        color: [0.5; 3].into(),
        border_radius: 4.0.into(),
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    })
}

pub fn dot_separator<'a>() -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(
        text("·························")
            .size(30)
            .width(Length::Fill)
            .height(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
}

pub fn labeled_separator<'a>(label: &'a str) -> MenuTree<'a, Message, iced::Renderer> {
    let q_1 = quad::Quad {
        color: [0.5; 3].into(),
        border_radius: 4.0.into(),
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    };
    let q_2 = quad::Quad {
        color: [0.5; 3].into(),
        border_radius: 4.0.into(),
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    };

    MenuTree::new(row![
        q_1,
        text(label)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
        q_2,
    ])
}

pub fn circle<'a>(color: Color) -> quad::Quad {
    let radius = 10.0;

    quad::Quad {
        color,
        inner_bounds: quad::InnerBounds::Square(radius * 2.0),
        border_radius: radius.into(),
        ..Default::default()
    }
}

pub fn programmer_actions_menu<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
   

    let root = MenuTree::with_children(
        menu_button("Programmer"),
        vec![
            connect_button_item("Connect", Message::Connect),
            programmer_button_item("Read", Message::ReadTarget, &_app.status, &_app.target_status),
            programmer_button_item("TestButton", Message::TestFeedback, &_app.status, &_app.target_status),
            empty_item("Write"),
            empty_item("Erase"),
        ],
    )
    .width(110);

    root
}


pub fn file_system_menu<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {

    let root = MenuTree::with_children(
        menu_button("File"),
        vec![
            file_button_item("Open(s19/bin)", Message::OpenFile),
            file_button_item("Save(s19/bin)", Message::SaveFile),
            //file_button_item("Save As", Message::TestFeedback),
    
        ],
    )
    .width(110);

    root
}

pub fn information_menu<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {


    let connection_image_item = MenuTree::new(
        container(toggler(
            Some("Connection image".to_string()),
            app.show_conn_image,
            Message::ConnectionImageOpen,
        ))
        .padding([0, 8])
        .height(Length::Fill)
        .align_y(alignment::Vertical::Center),
    );


    let root = MenuTree::with_children(
        menu_button("Info"),
        vec![
            about_button_item("About", Message::OpenAboutCard),
            dot_separator(),
            connection_image_item,
            dot_separator(),
        ],
    );

    root
}

pub fn view_customization_menu<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let [r, g, b, _] = app.theme.palette().primary.into_rgba8();

    let primary = sub_menu(
        "Adjust Theme",
        Message::Menu,
        vec![
            MenuTree::new(slider(0..=255, r, move |x| {
                Message::ColorChange(Color::from_rgb8(x, g, b))
            })),
            MenuTree::new(slider(0..=255, g, move |x| {
                Message::ColorChange(Color::from_rgb8(r, x, b))
            })),
            MenuTree::new(slider(0..=255, b, move |x| {
                Message::ColorChange(Color::from_rgb8(r, g, x))
            })),
        ],
    );

    let back_style: fn(&iced::Theme) -> container::Appearance = |theme| container::Appearance {
        background: Some(theme.extended_palette().primary.base.color.into()),
        ..Default::default()
    };
   
   let back = container(col![])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(back_style); 

    let back_ = MenuTree::new(back);

    let root = MenuTree::with_children(
        menu_button("View"),
        vec![
  
            MenuTree::new(
                row![toggler(
                    Some("Dark Mode".into()),
                    app.dark_mode,
                    Message::ThemeChange
                )]
                .padding([0, 8]),
            ),
            color_item([0.28, 0.36, 0.37]),
            color_item([0.32, 0.32, 0.4]),
            color_item([0.56, 0.55, 0.39]),
            primary,
            back_,
        ],
    );

    root
}

pub fn target_selection_menu<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {

    let root = MenuTree::with_children(
        menu_button("Target"),
        vec![
        target_button_item("MC56F8011", Message::TargetSelect(TargetSelector::Mc56f8011)),
        target_button_item("MC56F8025", Message::TargetSelect(TargetSelector::Mc56f8025)),
        target_button_item("MC56F8035", Message::TargetSelect(TargetSelector::Mc56f8035)),
        ],
    )
    .width(110);

    root
}