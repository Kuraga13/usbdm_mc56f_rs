use iced::widget::{column as col, Column};
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,
};
use iced::{alignment, theme, Application, Color, Element, Length};

use iced_aw::menu::{ItemHeight, ItemWidth, MenuBar, MenuTree, PathHighlight};
use iced_aw::quad;


use crate::usb_interface::{UsbInterface, find_usbdm_as, find_usbdm,};
use crate::errors::{Error};
use crate::settings::{TargetVddSelect};
use crate::programmer::{Programmer};
use crate::hexbuff_widget::{HexBufferView, HexBuffMsg, HexBuffer};


#[derive(Debug, Clone)]
enum UsbdmAppStatus {
    
    Start,
    Connected,
    //Errored(Error),
    Errored,

}








#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SizeOption {
    Uniform,
    Static,
}
impl SizeOption {
    const ALL: [SizeOption; 2] = [SizeOption::Uniform, SizeOption::Static];
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

#[derive(Debug, Clone)]
pub enum Message {
    Debug(String),
    ValueChange(u8),
    CheckChange(bool),
    ToggleChange(bool),
    ColorChange(Color),
    Flip,
    ThemeChange(bool),
    TextChange(String),
    SizeOption(SizeOption),

    Init,
    Start,
    Connect,
    Disconnect,
    PowerSelect(TargetVddSelect),
    SetPower,
    TestFeedback,
}

pub struct App {

    selected_power : TargetVddSelect,
    programmer     : Option<Programmer>,

    status         : UsbdmAppStatus,
    buff           : Vec<HexBuffer>,

    buffer_view    : Vec<HexBufferView>,

    title: String,
    value: u8,
    check: bool,
    toggle: bool,
    theme: iced::Theme,
    flip: bool,
    dark_mode: bool,
    text: String,
    size_option: SizeOption,
}
impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let theme = iced::Theme::custom(theme::Palette {
            primary: Color::from([0.23, 0.61, 0.81]),
            danger: Color::from([1.0, 0.00, 0.00]),
            success: Color::from([0.70, 0.75, 0.02]),
            ..iced::Theme::Light.palette()
        });

        (   
            
            

            Self {
                title: "Usbdm_rs".to_string(),
                value: 0,
                check: false,
                toggle: false,
                theme,
                flip: false,
                dark_mode: false,
                text: "Text Input".into(),
                size_option: SizeOption::Static,


                selected_power : TargetVddSelect::VddOff,
                programmer     : None,
                status         : UsbdmAppStatus::Start,
                buff           : vec![HexBuffer::new()],
                buffer_view    : vec![HexBufferView::default()],
            },
            iced::Command::none(),
        )
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Debug(s) => {
                self.title = s.clone();
            }
            Message::ValueChange(v) => {
                self.value = v;
                self.title = v.to_string();
            }
            Message::CheckChange(c) => {
                self.check = c;
                self.title = c.to_string();
            }
            Message::ToggleChange(t) => {
                self.toggle = t;
                self.title = t.to_string();
            }
            Message::ColorChange(c) => {
                self.theme = iced::Theme::custom(theme::Palette {
                    primary: c,
                    ..self.theme.palette()
                });
                self.title = format!("[{:.2}, {:.2}, {:.2}]", c.r, c.g, c.b);
            }
            Message::Flip => self.flip = !self.flip,
            Message::ThemeChange(b) => {
                self.dark_mode = b;
                let primary = self.theme.palette().primary;
                if b {
                    self.theme = iced::Theme::custom(theme::Palette {
                        primary,
                        ..iced::Theme::Dark.palette()
                    })
                } else {
                    self.theme = iced::Theme::custom(theme::Palette {
                        primary,
                        ..iced::Theme::Light.palette()
                    })
                }
            }
            Message::TextChange(s) => {
                self.text = s.clone();
                self.title = s;
            }
            Message::SizeOption(so) => {
                self.size_option = so;
                self.title = self.size_option.to_string();
            }

            Message::Init => 
            {
                
                self.status  = UsbdmAppStatus::Start;

            }

            Message::Start => 
            {
                
            

            }
            

            Message::Connect => 
            {    
              let check_connect = find_usbdm();

              match check_connect
                 {
                    Ok(check_connect) =>
                    {

                    println!("Try claim usb");
                    let usb_int = UsbInterface::new(check_connect).expect("Programmer Lost Connection");
                    self.programmer = Some(Programmer::new(usb_int)); 
                    self.status  = UsbdmAppStatus::Connected;


                    }
                    Err(_e) =>
                    {
                    dbg!("Programmer not connected");
                    self.status = UsbdmAppStatus::Errored;
                    self.title =  String::from("Programmer not found! Check connection").clone();
                    }
                 }
            } 

            Message::Disconnect => 
            {    
            
                match &self.programmer
                {   
                Some(_programmer) =>{ 
                println!("Try disconnect and drop");
                drop(&mut self.programmer);
                self.programmer = None;
                self.status  = UsbdmAppStatus::Start; 
                
                }
                    
                None => {}
                } 
                
            } 


            Message::SetPower => 
            {    
            
             let usbdm =  self.programmer.as_mut().expect("");
             match &self.selected_power{   

                TargetVddSelect::VddOff     => {if let Err(_e) =  usbdm.set_vdd_off()  {} else {}}
                TargetVddSelect::Vdd3V3     => {if let Err(_e) =  usbdm.set_vdd_3_3v() {} else {}}
                TargetVddSelect::Vdd5V      => {if let Err(_e) =  usbdm.set_vdd_5v()   {} else {}}
                TargetVddSelect::VddEnable  => {if let Err(_e) =  usbdm.set_vdd_off()  {} else {}}
                TargetVddSelect::VddDisable => {if let Err(_e) =  usbdm.set_vdd_off()  {} else {}}}

            }   


            Message::PowerSelect(power_select) => 
            {

                self.selected_power = power_select;

            }


            Message::TestFeedback =>
            {
               
               let usbdm =  self.programmer.as_mut().expect("");
               if let Err(_e) = usbdm.refresh_feedback() {  };
               usbdm.set_bdm_options();
               usbdm.set_target_mc56f();
               usbdm.dsc_connect();
    
            } 



        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {

        let pick_list_power = pick_list(
            &TargetVddSelect::ALL[..],
            Some(self.selected_power),
            Message::PowerSelect,
        );

        let set_power_button = match self.status
        {
           UsbdmAppStatus::Start      =>  
           {

            iced::widget::Button::new(iced::widget::Text::new("VDD")).style(iced::theme::Button::Custom(Box::new(PowerButtonStyle {})))

           }  
              
           UsbdmAppStatus::Connected  =>  
           {

            iced::widget::Button::new(iced::widget::Text::new("VDD")).style(theme::Button::Primary).on_press(Message::SetPower)

           }
           UsbdmAppStatus::Errored    => 
           {

            iced::widget::Button::new(iced::widget::Text::new("VDD")).style(theme::Button::Secondary)

           }    

        };
        


        let pick_size_option = pick_list(
            &SizeOption::ALL[..],
            Some(self.size_option),
            Message::SizeOption,
        );

        let mb = match self.size_option {
            SizeOption::Uniform => {
                MenuBar::new(vec![menu_1_1(self), menu_1(self), menu_2(self), menu_3(self), menu_4(self)])
                    .item_width(ItemWidth::Uniform(180))
                    .item_height(ItemHeight::Uniform(25))
            }
            SizeOption::Static => MenuBar::new(vec![
                menu_1_1(self),
                menu_1(self),
                menu_2(self),
                menu_3(self),
                menu_4(self),
                menu_5(self),
            ])
            .item_width(ItemWidth::Static(180))
            .item_height(ItemHeight::Static(25)),
        }
        .spacing(4)
        .bounds_expand(30)
        .path_highlight(Some(PathHighlight::MenuActive));

        let r = row!(mb, horizontal_space(Length::Fill), pick_size_option, horizontal_space(Length::Units(3)), pick_list_power, horizontal_space(Length::Units(3)), set_power_button)
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
        let back = container(col![])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(back_style);

        //let page_buffer = Column::with_children(
        //    self.buff.iter().map(HexBuffer::view).collect(),
       // );
               

        let c = if self.flip {
            col![back, top_bar, ]
        } else {
            col![top_bar, back,]
        };

        c.into()
    }

 
}

struct PowerButtonStyle;
impl button::StyleSheet for PowerButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}

struct ButtonStyle;
impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            border_radius: 4.0.into(),
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}

fn base_button<'a>(
    content: impl Into<Element<'a, Message, iced::Renderer>>,
    msg: Message,
) -> button::Button<'a, Message, iced::Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        .on_press(msg)
}

fn base_button_empty<'a>(
    content: impl Into<Element<'a, Message, iced::Renderer>>,
) -> button::Button<'a, Message, iced::Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        
}

fn labeled_button<'a>(label: &str, msg: Message) -> button::Button<'a, Message, iced::Renderer> {


    base_button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),

        msg,)

}


fn empty_labeled_button<'a>(label: &str) -> button::Button<'a, Message, iced::Renderer> {


    base_button_empty(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),)

}

fn debug_button<'a>(label: &str) -> button::Button<'a, Message, iced::Renderer> {
    labeled_button(label, Message::Debug(label.into()))
}


fn connect_button_item<'a>(label: &str, msg : Message) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(labeled_button(label, msg).width(Length::Fill).height(Length::Fill))
}

fn programmer_button_item<'a>(label: &str, msg : Message, state : &UsbdmAppStatus) -> MenuTree<'a, Message, iced::Renderer> {

    match state
    {
    
    UsbdmAppStatus::Connected =>
    {
        MenuTree::new(labeled_button(label, msg).width(Length::Fill).height(Length::Fill))
    }
    
    _ => 
    {
        MenuTree::new(empty_labeled_button(label).width(Length::Fill).height(Length::Fill))
    }

    }



    
}

fn debug_item<'a>(label: &str) -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(debug_button(label).width(Length::Fill).height(Length::Fill))
}

fn color_item<'a>(color: impl Into<Color>) -> MenuTree<'a, Message, iced::Renderer> {
    let color = color.into();
    MenuTree::new(base_button(circle(color), Message::ColorChange(color)))
}

fn sub_menu<'a>(
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

fn debug_sub_menu<'a>(
    label: &str,
    children: Vec<MenuTree<'a, Message, iced::Renderer>>,
) -> MenuTree<'a, Message, iced::Renderer> {
    sub_menu(label, Message::Debug(label.into()), children)
}

fn separator<'a>() -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(quad::Quad {
        color: [0.5; 3].into(),
        border_radius: 4.0.into(),
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    })
}

fn dot_separator<'a>() -> MenuTree<'a, Message, iced::Renderer> {
    MenuTree::new(
        text("·························")
            .size(30)
            .width(Length::Fill)
            .height(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
}

fn labeled_separator<'a>(label: &'a str) -> MenuTree<'a, Message, iced::Renderer> {
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

fn circle<'a>(color: Color) -> quad::Quad {
    let radius = 10.0;

    quad::Quad {
        color,
        inner_bounds: quad::InnerBounds::Square(radius * 2.0),
        border_radius: radius.into(),
        ..Default::default()
    }
}

fn menu_1<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
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
            programmer_button_item("Read", Message::TestFeedback, &_app.status),
            programmer_button_item("Write", Message::TestFeedback, &_app.status),
            programmer_button_item("Erase", Message::TestFeedback, &_app.status),
        ],
    )
    .width(110);

    root
}


fn menu_1_1<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
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
            programmer_button_item("Save", Message::TestFeedback, &_app.status),
            programmer_button_item("Save As", Message::TestFeedback, &_app.status),
            programmer_button_item("Erase", Message::TestFeedback, &_app.status),
        ],
    )
    .width(110);

    root
}

fn menu_2<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {
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
        horizontal_space(Length::Units(8)),
        slider(0..=255, app.value, Message::ValueChange)
    ]);

    let txn = MenuTree::new(text_input("", &app.text, Message::TextChange));

    let root = MenuTree::with_children(
        debug_button("Widgets"),
        vec![
            programmer_button_item("Test_Feedback", Message::TestFeedback, &app.status),
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

fn menu_3<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {
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

fn menu_4<'a>(_app: &App) -> MenuTree<'a, Message, iced::Renderer> {
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

fn menu_5<'a>(app: &App) -> MenuTree<'a, Message, iced::Renderer> {
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
