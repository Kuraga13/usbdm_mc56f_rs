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
use crate::settings::{TargetVddSelect};
use crate::feedback::{PowerStatus};
use super::styling::{PowerButtonStyle, ButtonStyle, EnablePowerButtonStyle};
use super::connection_image::{dsc_connection_image};

use super::hexbuffer_widget::{TableContents,table_contents };

impl TargetVddSelect {
    pub const ALL: [TargetVddSelect; 2] = [
        TargetVddSelect::Vdd3V3,
        TargetVddSelect::Vdd5V,

    ];
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeOption {
    Uniform,
    Static,
}
impl SizeOption {
    pub  const ALL: [SizeOption; 2] = [SizeOption::Uniform, SizeOption::Static];
}
impl std::fmt::Display for SizeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Uniform => "Uniform",
                Self::Static => "Static",
            }
        )
    }
}



pub fn main_page<'a>(_app: &App) -> Column<'a, Message, iced::Renderer>
{
    let pick_list_power = pick_list(
        &TargetVddSelect::ALL[..],
        Some(_app.selected_power),
        Message::PowerSelect,
    );

    let set_power_button =  power_button(&_app.status, &_app.power_status);
    


    let pick_size_option = pick_list(
        &SizeOption::ALL[..],
        Some(_app.size_option),
        Message::SizeOption,
    );

    let mb = match _app.size_option {
        SizeOption::Uniform => {
            MenuBar::new(vec![menu_1_1(_app), menu_1(_app), menu_2(_app), menu_3(_app), menu_4(_app)])
                .item_width(ItemWidth::Uniform(180))
                .item_height(ItemHeight::Uniform(25))
        }
        SizeOption::Static => MenuBar::new(vec![
            menu_1_1(_app),
            menu_1(_app),
            menu_2(_app),
            menu_3(_app),
            menu_4(_app),
            menu_5(_app),
        ])
        .item_width(ItemWidth::Static(180))
        .item_height(ItemHeight::Static(25)),
    }
    .spacing(4.0)
    .bounds_expand(30)
    .path_highlight(Some(PathHighlight::MenuActive));

    let r = row!(mb, horizontal_space(Length::Fill), pick_size_option, horizontal_space(Length::Fixed(3.0)), pick_list_power, horizontal_space(Length::Fixed(3.0)), set_power_button)
        .padding([2, 8])
        .align_items(alignment::Alignment::Center);
  

    let top_bar_style: fn(&iced::Theme) -> container::Appearance =
            |_theme| container::Appearance {
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            };

    let top_bar = container(r).width(Length::Fill).style(top_bar_style);

    let back_style: fn(&iced::Theme) -> container::Appearance = |theme| container::Appearance {
            background: Some(theme.extended_palette().primary.base.color.into()),
            ..Default::default()
        };
     /* 
     let back = container(col![])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(back_style); */

    let test_test_line   = vec![vec!["test_test1".to_string(), "test_test2".to_string(), "test_test3".to_string(),]; 4500];
    let test_addr_line   = vec![vec!["01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F".to_string(),]; 4500];
    

    let table_test = table_contents(20.00, _app.buffer.download_string(), || test_buffer_double_click() );
     
    let image_conn = dsc_connection_image(1000);
    
    let test_test = scrollable(Container::new(table_test).align_y(alignment::Vertical::Center));
    
    let body =
    if(_app.show_conn_image)
    {
        col![image_conn]
    }
    else
    {
        
        col![test_test]
    };


    let c = if _app.flip {
            col![body, top_bar]
    } 
    else 
    {
            col![top_bar, body]
    };

    c        


         
 
         
         
}

pub fn test_buffer_double_click() ->  Message
{

Message::TestFeedback

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

pub fn debug_button<'a>(label: &str) -> button::Button<'a, Message, iced::Renderer> {
    labeled_button(label, Message::Debug(label.into()))
}

pub fn about_button_item<'a>(label: &str, msg : Message) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(labeled_button(label, msg).width(Length::Fill).height(Length::Fill))
}

pub fn connect_button_item<'a>(label: &str, msg : Message) -> MenuTree<'a, Message, iced::Renderer> {
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

pub fn debug_item<'a>(label: &str) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(debug_button(label).width(Length::Fill).height(Length::Fill))
}

pub fn about_buttton<'a>(label: &str) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(debug_button(label).width(Length::Fill).height(Length::Fill))
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

pub fn debug_sub_menu<'a>(
    label: &str,
    children: Vec<MenuTree<'a, Message, iced::Renderer>>,
) -> MenuTree<'a, Message, iced::Renderer> {
    sub_menu(label, Message::Debug(label.into()), children)
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

pub fn menu_1<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let sub_5 = debug_sub_menu(
        "SUB",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    );
    let sub_4 = debug_sub_menu(
        "SUB",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(180);
    let sub_3 = debug_sub_menu(
        "More sub menus",
        vec![
            debug_item("You can"),
            debug_item("nest menus"),
            sub_4,
            debug_item("how ever"),
            debug_item("You like"),
            sub_5,
        ],
    );
    let sub_2 = debug_sub_menu(
        "Another sub menu",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            sub_3,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(140);
    let sub_1 = debug_sub_menu(
        "A sub menu",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            sub_2,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(220);

    let root = MenuTree::with_children(
        debug_button("Programmer"),
        vec![
            connect_button_item("Connect", Message::Connect),
            programmer_button_item("Read", Message::ReadTarget, &_app.status, &_app.target_status),
            programmer_button_item("Write", Message::WriteTarget, &_app.status, &_app.target_status),
            programmer_button_item("Erase", Message::TestFeedback, &_app.status, &_app.target_status),
        ],
    )
    .width(110);

    root
}


pub fn menu_1_1<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let sub_5 = debug_sub_menu(
        "SUB",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    );
    let sub_4 = debug_sub_menu(
        "SUB",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(180);
    let sub_3 = debug_sub_menu(
        "More sub menus",
        vec![
            debug_item("You can"),
            debug_item("nest menus"),
            sub_4,
            debug_item("how ever"),
            debug_item("You like"),
            sub_5,
        ],
    );
    let sub_2 = debug_sub_menu(
        "Another sub menu",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            sub_3,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(140);
    let sub_1 = debug_sub_menu(
        "A sub menu",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            sub_2,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(220);

    let root = MenuTree::with_children(
        debug_button("File"),
        vec![
            connect_button_item("Open", Message::Connect),
            programmer_button_item("Save", Message::TestFeedback, &_app.status, &_app.target_status),
            programmer_button_item("Save As", Message::TestFeedback, &_app.status, &_app.target_status),
            programmer_button_item("Erase", Message::TestFeedback, &_app.status, &_app.target_status),
        ],
    )
    .width(110);

    root
}

pub fn menu_2<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let sub_1 = MenuTree::with_children(
        container(toggler(
            Some("Or as a sub menu item".to_string()),
            app.toggle,
            Message::ToggleChange,
        ))
        .padding([0, 8])
        .height(Length::Fill)
        .align_y(alignment::Vertical::Center),
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    );

    let bt = MenuTree::new(
        button(
            text("Button")
                .width(Length::Fill)
                .height(Length::Fill)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::Debug("Button".into())),
    );

    let cb =
        MenuTree::new(checkbox("Checkbox", app.check, Message::CheckChange).width(Length::Fill));

    let sld = MenuTree::new(row![
        "Slider",
        horizontal_space(Length::Fixed(8.0)),
        slider(0..=255, app.value, Message::ValueChange)
    ]);

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

    let txn = MenuTree::new(text_input("", &app.text, Message::TextChange));

    let root = MenuTree::with_children(
        debug_button("Info"),
        vec![
            about_button_item("About", Message::OpenAboutCard),
            programmer_button_item("Test_Feedback", Message::TestFeedback, &app.status, &app.target_status),
            connection_image_item,
            debug_item("as a menu item"),
            bt,
            cb,
            sld,
            txn,
            sub_1,
            separator(),
            debug_item("Seperators are also widgets"),
            labeled_separator("Separator"),
            debug_item("Item"),
            debug_item("Item"),
            dot_separator(),
            debug_item("Item"),
            debug_item("Item"),
        ],
    );

    root
}

pub fn menu_3<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let [r, g, b, _] = app.theme.palette().primary.into_rgba8();

    let primary = debug_sub_menu(
        "Primary",
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

    let root = MenuTree::with_children(
        debug_button("View"),
        vec![
            MenuTree::new(
                labeled_button("Inverse View", Message::Flip)
                    .width(Length::Fill)
                    .height(Length::Fill),
            ),
            MenuTree::new(
                row![toggler(
                    Some("Dark Mode".into()),
                    app.dark_mode,
                    Message::ThemeChange
                )]
                .padding([0, 8]),
            ),
            color_item([0.45, 0.25, 0.57]),
            color_item([0.15, 0.59, 0.64]),
            color_item([0.76, 0.82, 0.20]),
            color_item([0.17, 0.27, 0.33]),
            primary,
        ],
    );

    root
}

pub fn menu_4<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let dekjdaud = debug_sub_menu(
        "dekjdaud",
        vec![
            debug_item("ajrs"),
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"),
            debug_item("ajrs"),
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"),
        ],
    );

    let luyortp = debug_sub_menu(
        "luyortp",
        vec![
            debug_item("ajrs"), // 0
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"), // 13
        ],
    );

    let jcsu = debug_sub_menu(
        "jcsu",
        vec![
            debug_item("ajrs"), // 0
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            luyortp, // 11
            debug_item("mmdyrc"),
            debug_item("nquc"), // 13
        ],
    );

    let root = MenuTree::with_children(
        debug_button("Scroll"),
        vec![
            debug_item("ajrs"), // 0
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            jcsu, // 9
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"), // 13
            debug_item("ajrs"), // 14
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"), // 27
            debug_item("ajrs"), // 28
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            dekjdaud,
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"), // 41
            debug_item("ajrs"), // 42
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"), // 52
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"), // 55
        ],
    );

    root
}

pub fn menu_5<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {
    let slider_count = 3;
    let slider_width = 30;
    let spacing = 4;

    let [r, g, b, _] = app.theme.palette().primary.into_rgba8();

    let sliders = MenuTree::new(
        row![
            vertical_slider(0..=255, r, move |x| Message::ColorChange(Color::from_rgb8(
                x, g, b
            )))
            .width(30),
            vertical_slider(0..=255, g, move |x| Message::ColorChange(Color::from_rgb8(
                r, x, b
            )))
            .width(30),
            vertical_slider(0..=255, b, move |x| Message::ColorChange(Color::from_rgb8(
                r, g, x
            )))
            .width(30),
        ]
        .spacing(4),
    )
    .height(100);

    let root = MenuTree::with_children(
        debug_button("Static"),
        vec![labeled_separator("Primary"), sliders],
    )
    .width(slider_width * slider_count + (slider_count - 1) * spacing);

    root
}
