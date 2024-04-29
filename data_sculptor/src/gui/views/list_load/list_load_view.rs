//! Module implementing the [`ListLoadView`]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use iced::{Background, Color, Command, Element, Length};
use iced::widget::{button, Column, Space, Row, Container, container};
use crate::core::data_manager::DataManager;
use crate::core::filter::{FilterType};
use crate::gui::gui_message::GUIMessage;
use crate::gui::views::filter::filter_view::FilterView;
use crate::gui::views::list_load::{list_display, messages};

/// Lets the user load a data file and display it in a scrollable list.
///
/// Also handles displaying file io errors.
pub struct ListLoadView
{
    pub loaded_valid_file: bool,
    pub load_error_msg: String,
    pub loading_file: bool,
    filter_views: HashMap<FilterType, FilterView>,
    opened_filter_view: Option<FilterType>
}

impl Default for ListLoadView
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

impl ListLoadView
{
    // UPDATE
    pub fn update(&mut self, message: GUIMessage, dm: &Arc<Mutex<DataManager>>) -> Command<GUIMessage>
    {
        match message
        {
            GUIMessage::SelectFile => {self.select_file()}
            GUIMessage::FileSelected(path) => {self.file_selected(path, dm)}
            GUIMessage::OpenFilterView(filter_type) => {self.open_filter_view(filter_type)}
            GUIMessage::ReturnToView(view_name) => {self.return_to_view(view_name)}
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
        if view_name != ListLoadView::view_title()
        {
            return Command::none()
        }

        self.opened_filter_view = None;
        Command::none()
    }

    // VIEW
    pub fn view<'a>(&'a self, data_manager: &'a Arc<Mutex<DataManager>>) -> Element<GUIMessage>
    {
        // SHOW FILTER VIEW IF ONE IS OPENED
        if let Some(filter_view) = &self.opened_filter_view
        {
            return self.filter_views.get(filter_view).unwrap().view()
        }

        //TOP ROW
        let top_row: Element<GUIMessage> = Row::new()
            .push
            (
                button("Date filters")
                    .on_press(GUIMessage::OpenFilterView(FilterType::Date))
                    .padding(10)
            )
            .push
            (
                button("Key filters")
                    .on_press(GUIMessage::OpenFilterView(FilterType::Key))
                    .padding(10)
            )
            .push
            (
                button("Value filters")
                    .on_press(GUIMessage::OpenFilterView(FilterType::Value))
                    .padding(10)
            )
            .push
            (
                Space::with_width(Length::FillPortion(10))
            )
            .push
            (
                button("Select file")
                    .on_press(GUIMessage::SelectFile)
                    .padding(10)
            )
            .push
            (
                Space::with_width(Length::FillPortion(1))
            )
            .spacing(20).into();

        let top_row_container = Container::new(top_row).style(container::Appearance
        {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.23))),
            border: Default::default(),
            text_color: None,
            shadow: Default::default(),
        }).padding(20);

        let msg_container = messages::build_message_container(&self);
        let data_list_display = list_display::display_list(data_manager);

        Column::new()
            .push(Space::with_height(15))
            .push(top_row_container)
            .push(Space::with_height(20))
            .push(msg_container)
            .push(data_list_display)
            .into()
    }

    pub fn view_title() -> &'static str
    {
        "list_load_view"
    }
}