//! This module is the core manager of the graphical user interface.
//!
//! It manages the iced application and, based on its current state,
//! switches between displaying the different views of the 'views' module.

use iced::{Task, Element, Theme};
use std::sync::{Arc, Mutex};
use iced::widget::Column;
use crate::core::data_manager::DataManager;
use crate::gui::views::gui_view_type::GUIViewType;
use crate::gui::gui_message::GUIMessage;
use crate::gui::views::list::list_view_control::ListView;
use crate::gui::views::menu::menu_view_control::MenuView;
use crate::file_io::asset_handler;

/// Initializes the iced application using an [`Arc`] of the [`DataManager`] that is shared
/// between all submodules of data_sculptor.
pub fn init(data_manager: Arc<Mutex<DataManager>>) -> iced::Result
{
    iced::application("Data Sculptor", MainGUI::update, MainGUI::view)
        .theme(|_| Theme::Dark)
        .run_with(| | MainGUI::new(data_manager))
}

/// Struct implementing the iced application. Also holds the shared [`DataManager`]
/// and all possible views as well as a [`GUIViewType`] enum to switch between them.
pub struct MainGUI
{
    pub data_manager: Arc<Mutex<DataManager>>,
    pub cur_view: GUIViewType,

    // VIEWS
    pub list_view: ListView,
    pub menu_view: MenuView
}

impl MainGUI
{
    /// Constructs the iced applications, builds the views and displays the default view.
    /// Returns the new MainGUI state and the initialization task.
    fn new(data_manager: Arc<Mutex<DataManager>>) -> (Self, Task<GUIMessage>)
    {
        let instance = Self
            {
                data_manager,
                cur_view: GUIViewType::MenuView,

                // VIEWS
                list_view: ListView::default(),
                menu_view: MenuView{}
            };

        return (instance, asset_handler::init_assets_task());
    }

    fn update(&mut self, message: GUIMessage) -> Task<GUIMessage>
    {
        // RETURN TO MENU
        if let GUIMessage::ReturnToView(view_name) = message
        {
            if view_name == MenuView::view_title()
            {
                self.cur_view = GUIViewType::MenuView;
            }
        }

        // OPEN NEW VIEW
        else if let GUIMessage::OpenView(view_name) = message
        {
            match view_name
            {
                _ if view_name == ListView::view_title() =>
                    {self.cur_view = GUIViewType::ListView},

                _ => {}
            }
        }

        // UPDATE CURRENT VIEW
        match self.cur_view
        {
            GUIViewType::ListView => {self.list_view.update(message, &self.data_manager)}
            GUIViewType::MenuView => {self.menu_view.update(message)}
            _ => {Task::none()}
        }
    }

    fn view(&self) -> Element<GUIMessage>
    {
        match self.cur_view
        {
            GUIViewType::ListView => {self.list_view.view(&self.data_manager)}
            GUIViewType::MenuView => {self.menu_view.view()}
            _ => {Column::new().into()}
        }
    }
}