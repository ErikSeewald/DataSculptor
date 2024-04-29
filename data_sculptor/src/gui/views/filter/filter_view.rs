//! Module implementing the [`FilterView`]

use iced::{Command, Element};
use iced::widget::Column;
use crate::gui::gui_message::GUIMessage;

/// View for displaying and setting filters for the data list
pub struct FilterView
{
}

impl Default for FilterView
{
    fn default() -> Self
    {
        Self
        {
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
            GUIMessage::SelectFile => {Command::none()}
            GUIMessage::FileSelected(_) => {Command::none()}
        }
    }

    // VIEW
    pub fn view(&self) -> Element<GUIMessage>
    {
        Column::new()
            .into()
    }
}

