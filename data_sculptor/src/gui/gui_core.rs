//! This module is the core manager of the graphical user interface.
//!
//! It manages the iced application and, based on its current state,
//! switches between displaying the different views of the 'views' module.

use iced::{Application, Command, Element, Settings, Theme};
use std::sync::{Arc, Mutex};
use iced::widget::Column;
use crate::core::data_manager::DataManager;
use crate::gui::views::gui_view_type::GUIViewType;
use crate::gui::gui_message::GUIMessage;
use crate::gui::views::list::list_view_control::ListView;
use crate::gui::views::menu::menu_view_control::MenuView;

/// Initializes the iced application using an [`Arc`] of the [`DataManager`] that is shared
/// between all submodules of data_sculptor.
pub fn init(data_manager: Arc<Mutex<DataManager>>) -> iced::Result 
{
    MainGUI::run(Settings::with_flags(data_manager))
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

impl Application for MainGUI
{
    type Executor = iced::executor::Default;
    type Message = GUIMessage;
    type Theme = Theme;
    type Flags = Arc<Mutex<DataManager>>;

    /// Construct the iced applications and build all the default views and default
    /// to display one of them.
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>)
    {
        let instance = Self
            {
                data_manager: flags,
                cur_view: GUIViewType::MenuView,

                // VIEWS
                list_view: ListView::default(),
                menu_view: MenuView{}
            };

        return (instance, Command::none());
    }

    fn title(&self) -> String {String::from("Data Sculptor")}

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>
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
            _ => {Command::none()}
        }
    }

    fn view(&self) -> Element<Self::Message>
    {
        match self.cur_view
        {
            GUIViewType::ListView => {self.list_view.view(&self.data_manager)}
            GUIViewType::MenuView => {self.menu_view.view()}
            _ => {Column::new().into()}
        }
    }

    fn theme(&self) -> Self::Theme {Theme::Dark}
}