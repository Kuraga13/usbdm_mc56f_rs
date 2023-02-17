
use crate::app::{Message };

use iced::widget::{button, Button, Container, checkbox, horizontal_rule, vertical_rule,row, container, Text, text, Column, Rule, column, pick_list};
use iced::theme::{self, Theme};
use iced::widget::rule::{Appearance, FillMode, StyleSheet};
use iced::widget::rule;
use iced_native::widget;

use iced::alignment::{self, Alignment};
use iced::{
     Application, Background,  Command, Element, Length, Settings, Subscription,
    Sandbox,Color,Vector,
};

#[derive(Default)]
pub struct LineStyle
{
test : u16,

}
impl LineStyle {
    pub fn new(test : u16) -> Self {
        Self { test : test }
    }
}


impl rule::StyleSheet for LineStyle {

   type Style = Theme;
    
   fn appearance(&self, style: &Self::Style) -> rule::Appearance
   {
    rule::Appearance{
     /// The width (thickness) of the rule line.
    color: Color::BLACK,
    width: 1,
     /// The radius of the line corners.
    radius: 1.0,
    /// The [`FillMode`] of the rule.
    fill_mode: FillMode::Percent(50.00), }
    }
        
}

