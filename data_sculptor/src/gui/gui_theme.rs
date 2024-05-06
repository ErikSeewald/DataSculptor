//! Module defining the themes applied in the data_sculptor gui
//!
use iced::{Border, Color, Shadow, Theme, Vector};
use iced::Background;
use iced::border::Radius;
use iced::widget::button::{Appearance, StyleSheet};

pub struct ButtonTheme;

impl StyleSheet for ButtonTheme
{
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance
    {
        Appearance
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.6, 0.3))),
            text_color: Color::WHITE,
            border: Border
            {
                color: Color::from_rgb(0.1, 0.4, 0.25),
                width: 0.0,
                radius: Radius::from(2),
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance
    {
        Appearance
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.7, 0.35))),
            ..self.active(_style)
        }
    }

    fn pressed(&self, _style: &Self::Style) -> Appearance
    {
        Appearance
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.8, 0.45))),
            ..self.active(_style)
        }
    }
}
