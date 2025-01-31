//! Module implementing the control functions for the [`MenuView`]

use iced::{Task};
use crate::gui::gui_message::GUIMessage;

/// The main menu of the data_sculptor application.
pub struct MenuView
{

}

/// Implementation of the control functions for the menu view
impl MenuView
{
    // UPDATE
    pub fn update(&mut self, message: GUIMessage) -> Task<GUIMessage>
    {
        match message
        {
            _ => {Task::none()}
        }
    }

    pub fn view_title() -> &'static str
    {
        "menu_view"
    }
}