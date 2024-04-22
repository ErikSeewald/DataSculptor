//! Utility functions for the GUI of data_sculptor

use iced::{Element, Length};
use iced::widget::{Row, Space};
use crate::gui::gui_message::GUIMessage;

pub fn center_in_new_row(element: Element<GUIMessage>) -> Element<GUIMessage>
{
    Row::new()
        .push(Space::with_width(Length::FillPortion(3)))
        .push(element)
        .push(Space::with_width(Length::FillPortion(3)))
        .into()
}