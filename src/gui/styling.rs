
use crate::app::{Message };

use iced::widget::{button, scrollable, Button, Container, checkbox, horizontal_rule, vertical_rule,row, container, Text, text, Column, Rule, column, pick_list};
use iced::theme::{self, Theme};
use iced::widget::rule::{Appearance, FillMode, StyleSheet};
use iced::widget::rule;
use iced_native::widget;
use iced_native::widget::scrollable::style::Scrollbar;
use iced_native::widget::progress_bar;
use iced::alignment::{self, Alignment};
use iced::{
     Application, Background,  Command, Element, Length, Settings, Subscription,
    Sandbox,Color,Vector,
};

pub struct ScrollerHexBuffer;
impl scrollable::StyleSheet for ScrollerHexBuffer {
    type Style = iced::Theme;
    /// Produces the style of an active scrollbar.
    fn active(&self, style: &Self::Style) -> Scrollbar
    {
        scrollable::Scrollbar{

            background    : Some(Color::WHITE.into()),
            border_radius : 1.0,
            border_width  : 1.0,
            border_color  : Color::BLACK.into(),
            scroller      : scrollable::Scroller{

                            color         : Color::BLACK.into(),
                            border_radius : 1.0,
                            border_width  : 1.0,
                            border_color  : Color::BLACK.into(),
            }
        }
    }

    /// Produces the style of a hovered scrollbar.
    fn hovered(&self, style: &Self::Style) -> Scrollbar
    {
        {
            scrollable::Scrollbar{
    
                background    : Some(Color::WHITE.into()),
                border_radius : 1.0,
                border_width  : 1.0,
                border_color  : Color::BLACK.into(),
                scroller      : scrollable::Scroller{
    
                                color         : Color::TRANSPARENT.into(),
                                border_radius : 10.0,
                                border_width  : 10.0,
                                border_color  : Color::WHITE.into(),
                }
            }
        }
    }

    /// Produces the style of a scrollbar that is being dragged.
    fn dragging(&self, style: &Self::Style) -> Scrollbar {
        self.hovered(style)
    }

    /// Produces the style of an active horizontal scrollbar.
    fn active_horizontal(&self, style: &Self::Style) -> Scrollbar {
        self.active(style)
    }

    /// Produces the style of a hovered horizontal scrollbar.
    fn hovered_horizontal(&self, style: &Self::Style) -> Scrollbar {
        self.hovered(style)
    }

    /// Produces the style of a horizontal scrollbar that is being dragged.
    fn dragging_horizontal(&self, style: &Self::Style) -> Scrollbar {
        self.hovered_horizontal(style)
    }
}



pub struct EnablePowerButtonStyle;
impl button::StyleSheet for EnablePowerButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            background: Some(Color::from_rgb8( 52, 226, 3).into()),
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


#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ProgressBarMy {
   pub percentage_of_h: f32,
}
pub struct ProgressBarStyle;

impl progress_bar::StyleSheet for ProgressBarStyle {
   type Style = ProgressBarMy;

   fn appearance(&self, style: &Self::Style) -> progress_bar::Appearance {

      progress_bar::Appearance {
         background: Background::Color(Color::from_rgb8(0xCE, 0xCE, 0xCE)),
         bar: Background::Color(Color::from_rgb8(0x77, 0xC6, 0x0F)),
         border_radius: 3.0,
      }
   }
}