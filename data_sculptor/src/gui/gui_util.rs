use iced::{Element, Length};
use iced::widget::{Row, Space};
use crate::gui::gui_core::GUIMessage;

pub fn center_in_new_row(element: Element<GUIMessage>) -> Element<GUIMessage>
{
    Row::new()
        .push(Space::with_width(Length::FillPortion(3)))
        .push(element)
        .push(Space::with_width(Length::FillPortion(3)))
        .into()
}