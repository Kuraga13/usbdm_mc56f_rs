
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


pub struct PowerButtonStyle;
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

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

pub struct ButtonStyle;
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



#[derive(Default)]
pub struct LineStyle;


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

