//! Module defining the themes applied in the data_sculptor gui

use iced::{Border, Color, Theme};
use iced::Background;
use iced::border::Radius;
use iced::widget::button::{Appearance, StyleSheet};
use iced::widget::container;

// -----BUTTONS-----
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

pub struct FilterButtonTheme;
impl StyleSheet for FilterButtonTheme
{
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance
    {
        Appearance
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.4, 0.2))),
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
            background: Some(Background::Color(Color::from_rgb(0.0, 0.5, 0.25))),
            ..self.active(_style)
        }
    }

    fn pressed(&self, _style: &Self::Style) -> Appearance
    {
        Appearance
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.6, 0.35))),
            ..self.active(_style)
        }
    }
}

pub struct DeleteButtonTheme;
impl StyleSheet for DeleteButtonTheme
{
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance
    {
        Appearance
        {
            background: Some(Background::Color(Color::from_rgb(0.6, 0.25, 0.25))),
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
            background: Some(Background::Color(Color::from_rgb(0.75, 0.25, 0.25))),
            ..self.active(_style)
        }
    }

    fn pressed(&self, _style: &Self::Style) -> Appearance
    {
        Appearance
        {
            background: Some(Background::Color(Color::from_rgb(0.8, 0.25, 0.25))),
            ..self.active(_style)
        }
    }
}

// -----CONTAINERS-----
pub fn container_bar_style() -> container::Appearance
{
    container::Appearance
    {
        background: Some(Background::Color(Color::from_rgb(0.2, 0.22, 0.23))),
        border: Default::default(),
        text_color: None,
        shadow: Default::default(),
    }
}
