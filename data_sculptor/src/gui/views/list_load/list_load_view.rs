use std::sync::{Arc, Mutex};
use iced::{Command, Element};
use iced::widget::{button, Column, Space};
use crate::core::data_manager::DataManager;
use crate::gui::gui_message::GUIMessage;
use crate::gui::{gui_util};
use crate::gui::views::list_load::{list_display, messages};

pub struct ListLoadView
{
    pub loaded_valid_file: bool,
    pub load_error_msg: String,
    pub loading_file: bool
}

impl Default for ListLoadView
{
    fn default() -> Self
    {
        Self
        {
            loaded_valid_file: true,
            load_error_msg: "".to_string(),
            loading_file: false
        }
    }
}

impl ListLoadView
{
    // UPDATE
    pub fn update(&mut self, message: GUIMessage, list: &Arc<Mutex<DataManager>>) -> Command<GUIMessage>
    {
        match message
        {
            GUIMessage::SelectFile => {self.select_file()}
            GUIMessage::FileSelected(path) => {self.file_selected(path, list)}
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

    fn file_selected(&mut self, path: String, list: &Arc<Mutex<DataManager>>) -> Command<GUIMessage>
    {
        (self.loaded_valid_file, self.load_error_msg) =
            list.lock().unwrap().load_data(path.as_str());

        if self.loaded_valid_file
        {
            self.loading_file = false;
        }

        Command::none()
    }

    // VIEW
    pub fn view<'a>(&'a self, list: &'a Arc<Mutex<DataManager>>) -> Element<GUIMessage>
    {
        let top_row = gui_util::center_in_new_row
            (
                button("Select file")
                    .on_press(GUIMessage::SelectFile)
                    .padding(10)
                    .into()
            );

        let msg_container = messages::build_message_container(&self);
        let data_list_display = list_display::display_list(list);

        Column::new()
            .spacing(10)
            .push(Space::with_height(10))
            .push(top_row)
            .push(msg_container)
            .push(data_list_display)
            .into()
    }
}

