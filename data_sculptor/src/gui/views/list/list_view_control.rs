//! Module implementing the control functions for the [`ListView`]

use std::sync::{Arc, Mutex};
use iced::{Command};
use indexmap::IndexMap;
use crate::core::data_manager::DataManager;
use crate::core::filters::filter::{Filter, FilterID, FilterType};
use crate::file_io::{data_writer, file_dialogs};
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
    pub filter_views: [FilterView; 3],
    pub(crate) opened_filter_view: Option<FilterType>
}

impl Default for ListView
{
    fn default() -> Self
    {
        Self
        {
            loaded_valid_file: true,
            load_error_msg: "".to_string(),
            loading_file: false,
            filter_views:
            [
                FilterView::from(FilterType::Date),
                FilterView::from(FilterType::Key),
                FilterView::from(FilterType::Value)
            ],
            opened_filter_view: None
        }
    }
}

/// Implementation of the control functions for the list view
impl ListView
{
    // UPDATE
    pub fn update(&mut self, message: GUIMessage, dm: &Arc<Mutex<DataManager>>) -> Command<GUIMessage>
    {
        if let Some(filter_view) = self.opened_filter_view.clone()
        {
            match message
            {
                GUIMessage::ReturnToView(view_name) => {self.return_to_view(view_name)}
                _ =>
                    {
                        self.get_filter_view_mut(&filter_view).update(message)
                    }
            }
        }

        else
        {
            match message
            {
                GUIMessage::SelectFile => {self.select_file()}
                GUIMessage::SaveFile => {self.save_file(dm)}
                GUIMessage::FileSelected(path) => {self.file_selected(path, dm)}
                GUIMessage::OpenFilterView(filter_type) => {self.open_filter_view(filter_type)}
                _ => {Command::none()}
            }
        }
    }

    fn select_file(&mut self) -> Command<GUIMessage>
    {
        if let Some(file_path) = file_dialogs::pick_file()
        {
            self.loading_file = true;

            // For display reasons: Assume file is valid during loading, only change
            // if it fails.
            self.loaded_valid_file = true;

            Command::perform(async move { file_path }, GUIMessage::FileSelected)
        } else { Command::none() }
    }

    fn save_file(&mut self, dm: &Arc<Mutex<DataManager>>) -> Command<GUIMessage>
    {
        if let Some(file_path) = file_dialogs::save_json_file()
        {
            data_writer::write_data_filtered(file_path, &dm.lock().unwrap().data, &self);
        }
        Command::none()
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

    pub fn get_filters(&self, filter_type: &FilterType) -> &IndexMap<FilterID, Filter>
    {
        &self.get_filter_view(filter_type).filters
    }

    pub fn get_filter_view(&self, filter_type: &FilterType) -> & FilterView
    {
        match filter_type
        {
            FilterType::Date => &self.filter_views[0],
            FilterType::Key => &self.filter_views[1],
            FilterType::Value => &self.filter_views[2],
        }
    }

    pub fn get_filter_view_mut(&mut self, filter_type: &FilterType) -> &mut FilterView
    {
        match filter_type
        {
            FilterType::Date => &mut self.filter_views[0],
            FilterType::Key => &mut self.filter_views[1],
            FilterType::Value => &mut self.filter_views[2],
        }
    }

    pub fn view_title() -> &'static str
    {
        "list_view"
    }
}