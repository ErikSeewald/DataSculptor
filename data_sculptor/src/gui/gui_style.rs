//! Module defining the styles applied in the data_sculptor gui

use iced::{Border, Color, Shadow, Theme, Vector};
use iced::Background;
use iced::border::Radius;
use iced::widget::{button, container};

// -----BUTTONS-----
pub struct ButtonStyle;
impl ButtonStyle
{
    pub fn style(_: &Theme, status: button::Status) -> button::Style
    {
        match status
        {
            button::Status::Active => {ButtonStyle::active()}
            button::Status::Hovered => {ButtonStyle::hovered()}
            button::Status::Pressed => {ButtonStyle::pressed()}
            button::Status::Disabled => {Default::default()}
        }
    }

    fn active() -> button::Style
    {
        button::Style
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.6, 0.3))),
            text_color: Color::WHITE,
            border: Border
            {
                color: Color::from_rgb(0.1, 0.4, 0.25),
                width: 0.0,
                radius: Radius::from(2),
            },
            shadow: Shadow
            {
                color: Color::from_rgb(0.12, 0.15, 0.12),
                offset: Vector::new(1.0, 1.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        }

    }

    fn hovered() -> button::Style
    {
        button::Style
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.7, 0.35))),
            ..ButtonStyle::active()
        }
    }

    fn pressed() -> button::Style
    {
        button::Style
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.8, 0.45))),
            ..ButtonStyle::active()
        }
    }
}

pub struct FilterButtonStyle;
impl FilterButtonStyle
{
    pub fn style(_: &Theme, status: button::Status) -> button::Style
    {
        match status
        {
            button::Status::Active => {FilterButtonStyle::active()}
            button::Status::Hovered => {FilterButtonStyle::hovered()}
            button::Status::Pressed => {FilterButtonStyle::pressed()}
            button::Status::Disabled => {Default::default()}
        }
    }

    fn active() -> button::Style
    {
        button::Style
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.4, 0.2))),
            text_color: Color::WHITE,
            border: Border
            {
                color: Color::from_rgb(0.1, 0.4, 0.25),
                width: 0.0,
                radius: Radius::from(2),
            },
            shadow: Shadow
            {
                color: Color::from_rgb(0.08, 0.11, 0.08),
                offset: Vector::new(2.0, 2.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        }
    }

    fn hovered() -> button::Style
    {
        button::Style
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.5, 0.25))),
            ..FilterButtonStyle::active()
        }
    }

    fn pressed() -> button::Style
    {
        button::Style
        {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.6, 0.35))),
            ..FilterButtonStyle::active()
        }
    }
}

pub struct DeleteButtonStyle;
impl DeleteButtonStyle
{
    pub fn style(_: &Theme, status: button::Status) -> button::Style
    {
        match status
        {
            button::Status::Active => {DeleteButtonStyle::active()}
            button::Status::Hovered => {DeleteButtonStyle::hovered()}
            button::Status::Pressed => {DeleteButtonStyle::pressed()}
            button::Status::Disabled => {Default::default()}
        }
    }

    fn active() -> button::Style
    {
        button::Style
        {
            background: Some(Background::Color(Color::from_rgb(0.6, 0.25, 0.25))),
            text_color: Color::WHITE,
            border: Border
            {
                color: Color::from_rgb(0.1, 0.4, 0.25),
                width: 0.0,
                radius: Radius::from(2),
            },
            shadow: Shadow
            {
                color: Color::from_rgb(0.12, 0.1, 0.06),
                offset: Vector::new(2.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        }
    }

    fn hovered() -> button::Style
    {
        button::Style
        {
            background: Some(Background::Color(Color::from_rgb(0.75, 0.25, 0.25))),
            ..DeleteButtonStyle::active()
        }
    }

    fn pressed() -> button::Style
    {
        button::Style
        {
            background: Some(Background::Color(Color::from_rgb(0.8, 0.25, 0.25))),
            ..DeleteButtonStyle::active()
        }
    }
}

// -----CONTAINERS-----
pub fn container_bar_style(_: &Theme) -> container::Style
{
    container::Style
    {
        background: Some(Background::Color(Color::from_rgb(0.2, 0.22, 0.23))),
        border: Default::default(),
        text_color: None,
        shadow: Shadow
        {
            color: Color::from_rgb(0.11, 0.11, 0.11),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 10.0,
        },
    }
}
