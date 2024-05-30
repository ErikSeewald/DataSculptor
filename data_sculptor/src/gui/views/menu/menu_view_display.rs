//! Module implementing the display functions for the [`MenuView`]

use iced::{Color, Element, Length, theme};
use iced::widget::{button, Column, Container, Row, Space, Text};
use crate::gui::gui_message::GUIMessage;
use crate::gui::gui_theme;
use crate::gui::views::list::list_view_control::ListView;
use crate::gui::views::menu::menu_view_control::MenuView;

/// Implementation of the display functions for the menu view
impl MenuView
{
    pub fn view(&self) -> Element<GUIMessage>
    {
        // TITLE
        let title: Element<GUIMessage> = Row::new()
            .push(Space::with_width(Length::FillPortion(1)))
            .push
            (
                Text::new("Data Sculptor")
                    .size(80)
                    .style(Color::new(0.4, 0.8, 0.5, 1.0))
            )
            .push(Space::with_width(Length::FillPortion(1)))
            .into();

        //TOP ROW
        let top_row: Element<GUIMessage> = Row::new()
            .push
            (
                button("List view")
                    .on_press(GUIMessage::OpenView(ListView::view_title()))
                    .padding(10)
                    .style(theme::Button::custom(gui_theme::ButtonTheme))
            )
            .spacing(20).into();

        let top_row_container = Container::new(top_row)
            .padding(20)
            .style(gui_theme::container_bar_style());

        Column::new()
            .push(Space::with_height(30))
            .push(title)
            .push(Space::with_height(30))
            .push(top_row_container)
            .push(Space::with_height(20))
            .into()
    }
}