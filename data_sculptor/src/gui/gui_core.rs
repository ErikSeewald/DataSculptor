use iced::{Application, Command, Element, Settings, Theme};
use std::sync::{Arc, Mutex};
use crate::core::data_manager::DataManager;
use crate::gui::views::gui_view_type::GUIViewType;
use crate::gui::views::list_load::list_load_view::ListLoadView;
use crate::gui::gui_message::GUIMessage;

//! This module is the core manager of the graphical user interface.
//!
//! It manages the iced application and, based on its current state,
//! switches between displaying the different views of the 'views' module.

/// Initializes the iced application using an [`Arc`] of the [`DataManager`] that is shared
/// between all submodules of data_sculptor.
pub fn init(data_manager: Arc<Mutex<DataManager>>) -> iced::Result 
{
    MainGUI::run(Settings::with_flags(data_manager))
}
/// Struct implementing the iced application. Also holds the shared data_manager
/// and all possible views as well as a [`GUIViewType`] enum to switch between them.
pub struct MainGUI
{
    pub data_manager: Arc<Mutex<DataManager>>,
    pub cur_view: GUIViewType,

    // VIEWS
    pub list_load_view: ListLoadView
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
                cur_view: GUIViewType::ListLoadView,

                // VIEWS
                list_load_view: ListLoadView::default()
            };

        return (instance, Command::none());
    }

    fn title(&self) -> String {String::from("Data Sculptor")}

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>
    {
        match self.cur_view
        {
            GUIViewType::ListLoadView => {self.list_load_view.update(message, &self.data_manager)}
        }
    }

    fn view(&self) -> Element<Self::Message>
    {
        match self.cur_view
        {
            GUIViewType::ListLoadView => {self.list_load_view.view(&self.data_manager)}
        }
    }

    fn theme(&self) -> Self::Theme {Theme::Dark}
}