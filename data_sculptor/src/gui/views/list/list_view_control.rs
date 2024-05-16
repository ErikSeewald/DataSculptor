//! Module implementing the control functions for the [`ListView`]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use iced::{Command};
use crate::core::data_manager::DataManager;
use crate::core::filters::filter::{FilterType};
use crate::gui::gui_message::GUIMessage;
use crate::gui::views::filter::filter_view_control::FilterView;

/// Lets the user load a data file and display it in a scrollable list.
///
/// Also handles displaying file io errors.
pub struct ListView
{
    pub loaded_valid_file: bool,
    pub load_error_msg: String,
    pub loading_file: bool,
    pub(crate) filter_views: HashMap<FilterType, FilterView>,
    pub(crate) opened_filter_view: Option<FilterType>
}

impl Default for ListView
{
    fn default() -> Self
    {
        let mut instance = Self
        {
            loaded_valid_file: true,
            load_error_msg: "".to_string(),
            loading_file: false,
            filter_views: HashMap::new(),
            opened_filter_view: None
        };
        
        instance.filter_views.insert(FilterType::Date, FilterView::from(FilterType::Date));
        instance.filter_views.insert(FilterType::Key, FilterView::from(FilterType::Key));
        instance.filter_views.insert(FilterType::Value, FilterView::from(FilterType::Value));

        return instance;
    }
}

/// Implementation of the control functions for the list view
impl ListView
{
    // UPDATE
    pub fn update(&mut self, message: GUIMessage, dm: &Arc<Mutex<DataManager>>) -> Command<GUIMessage>
    {
        if let Some(filter_view) = &self.opened_filter_view
        {
            match message
            {
                GUIMessage::ReturnToView(view_name) => {self.return_to_view(view_name)}
                _ =>
                    {
                        self.filter_views.get_mut(filter_view).unwrap().update(message)
                    }
            }
        }

        else
        {
            match message
            {
                GUIMessage::SelectFile => {self.select_file()}
                GUIMessage::FileSelected(path) => {self.file_selected(path, dm)}
                GUIMessage::OpenFilterView(filter_type) => {self.open_filter_view(filter_type)}
                _ => {Command::none()}
            }
        }
    }

    fn select_file(&mut self) -> Command<GUIMessage>
    {
        let file = rfd::FileDialog::new().pick_file();
        let path = file.map(|f| f.as_path().to_string_lossy().into_owned());

        if let Some(file_path) = path
        {
            self.loading_file = true;

            // For display reasons: Assume file is valid during loading, only change
            // if it fails.
            self.loaded_valid_file = true;

            Command::perform(async move { file_path }, GUIMessage::FileSelected)
        } else { Command::none() }
    }

    fn file_selected(&mut self, path: String, dm: &Arc<Mutex<DataManager>>) -> Command<GUIMessage>
    {
        self.loading_file = false;

        let mut unwrapped_dm = dm.lock().unwrap();
        (self.loaded_valid_file, self.load_error_msg) = unwrapped_dm.load_data(path.as_str());

        Command::none()
    }

    fn open_filter_view(&mut self, filter_type: FilterType) -> Command<GUIMessage>
    {
        self.opened_filter_view = Some(filter_type);
        Command::none()
    }

    fn return_to_view(&mut self, view_name: &str) -> Command<GUIMessage>
    {
        if view_name != ListView::view_title()
        {
            return Command::none()
        }

        self.opened_filter_view = None;
        Command::none()
    }

    pub fn view_title() -> &'static str
    {
        "list_load_view"
    }
}