use iced::widget::{button, Column, Space};
use iced::{Application, Command, Element, Settings, Theme};

use std::sync::{Arc, Mutex};
use crate::core::data_manager::DataManager;
use crate::gui::{messages, list_display, gui_util};

pub fn init(data_manager: Arc<Mutex<DataManager>>) -> iced::Result 
{
    MainGUI::run(Settings::with_flags(data_manager))
}

pub struct MainGUI
{
    data_manager: Arc<Mutex<DataManager>>,
    pub loaded_valid_file: bool,
    pub load_error_msg: String,
    pub loading_file: bool
}

#[derive(Debug, Clone)]
pub enum GUIMessage
{
    OpenFile,
    FileSelected(String)
}

impl Application for MainGUI
{
    type Executor = iced::executor::Default;
    type Message = GUIMessage;
    type Theme = Theme;
    type Flags = Arc<Mutex<DataManager>>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>)
    {
        (
            Self
            {
                data_manager: flags,
                loaded_valid_file: true,
                load_error_msg: String::new(),
                loading_file: false
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {String::from("Data Sculptor")}

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>
    {
        match message
        {
            GUIMessage::OpenFile =>
            {
                let file = rfd::FileDialog::new().pick_file();
                let path = file.map(|f| f.as_path().to_string_lossy().into_owned());

                if let Some(file_path) = path
                {
                    self.loading_file = true;
                    self.loaded_valid_file = true; // set to true here, may change after parse
                    Command::perform(async move { file_path }, GUIMessage::FileSelected)
                }
                else {Command::none()}
            }

            GUIMessage::FileSelected(path) =>
            {
                (self.loaded_valid_file, self.load_error_msg) =
                self.data_manager.lock().unwrap().load_data(path.as_str());

                if self.loaded_valid_file
                {
                    self.loading_file = false;
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message>
    {
        let top_row = gui_util::center_in_new_row
            (
                button("Load file")
                    .on_press(GUIMessage::OpenFile)
                    .padding(10)
                    .into()
            );

        let msg_container = messages::build_message_container(&self);
        let data_list_display = list_display::display_list(&self.data_manager);

        Column::new()
            .spacing(10)
            .push(Space::with_height(10))
            .push(top_row)
            .push(msg_container)
            .push(data_list_display)
            .into()
    }

    fn theme(&self) -> Self::Theme {Theme::Dark}
}
