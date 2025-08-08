//! Module implementing the display functions for the [`MenuView`]

use iced::{Element, Length};
use iced::widget::{button, Column, Container, Row, Space, Text};
use crate::gui::gui_message::GUIMessage;
use crate::gui::gui_style;
use crate::gui::views::list::list_view_control::ListView;
use crate::gui::views::menu::menu_view_control::MenuView;

/// Implementation of the display functions for the menu view
impl MenuView
{
    pub fn view(&self) -> Element<GUIMessage>
    {
        let top_row: Element<GUIMessage> = Row::new()
            .push
            (
                menu_button
                    (
                        String::from("  List View"),
                        GUIMessage::OpenView(ListView::view_title())
                    )
            )
            .spacing(50).into();

        let second_row: Element<GUIMessage> = Row::new()
            .push(empty_menu_button())
            .spacing(50).into();

        let buttons_column = Column::new()
            .push(top_row)
            .push(Space::with_height(30))
            .push(second_row);

        let buttons_container = Container::new
            (
                Row::new()
                    .push(Space::with_width(Length::FillPortion(1)))
                    .push(buttons_column)
                    .push(Space::with_width(Length::FillPortion(1)))
            )
            .padding(20)
            .style(gui_style::container_bar_style);

        Column::new()
            .push(Space::with_height(Length::FillPortion(3)))
            .push(buttons_container)
            .push(Space::with_height(Length::FillPortion(4)))
            .into()
    }
}

pub fn menu_button(title: String, on_press: GUIMessage) -> Element<'static, GUIMessage>
{
    button(Text::new(title).size(32))
        .on_press(on_press)
        .padding(10)
        .width(Length::Fixed(180.0))
        .height(Length::Fixed(64.0))
        .style(gui_style::ButtonStyle::style)
        .into()
}

pub fn empty_menu_button()-> Element<'static, GUIMessage>
{
    menu_button(String::new(), GUIMessage::OpenView(""))
}