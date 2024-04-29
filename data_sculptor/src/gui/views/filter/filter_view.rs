//! Module implementing the [`FilterView`]

use iced::{Command, Element, Alignment};
use iced::widget::{Button, Column, Row, TextInput};
use crate::gui::gui_message::GUIMessage;
use crate::core::filter::{FilterType, Filter};
use crate::gui::views::list_load::list_load_view::ListLoadView;

/// View for displaying and setting filters for the data list
pub struct FilterView
{
    filter_type: FilterType,
    filters: Vec<Filter>
}

impl From<FilterType> for FilterView
{
    fn from(filter_type: FilterType) -> Self
    {
        Self
        {
            filter_type,
            filters: Vec::new()
        }
    }
}

impl FilterView
{
    // UPDATE
    pub fn update(&mut self, message: GUIMessage) -> Command<GUIMessage>
    {
        match message
        {
            _ => {Command::none()}
        }
    }

    // VIEW
    pub fn view(&self) -> Element<GUIMessage>
    {
        let text_input = TextInput::new(
            "Enter text...",
            "Enter text...",
        )
            .padding(10)
            .size(20);

        let button = Button::new("Submit")
            .on_press(GUIMessage::SelectFile)
            .padding(10);

        let return_button = Button::new("Return")
            .on_press(GUIMessage::ReturnToView(ListLoadView::view_title()))
            .padding(10);

        Column::new()
            .spacing(20)
            .push
            (
                Row::new()
                    .push(return_button)
            )
            .push
            (
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(20)
                    .push(text_input)
                    .push(button)
            )
            .into()
    }
}

