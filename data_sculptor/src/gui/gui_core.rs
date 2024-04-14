use iced::widget::{button, Column, Row, Space};
use iced::{Application, Command, Element, Length, Settings, Theme};

use std::sync::{Arc, Mutex};
use crate::core::data_manager::DataManager;

pub fn init(data_manager: Arc<Mutex<DataManager>>) -> iced::Result 
{
    MainGUI::run(Settings::with_flags(data_manager))
}

struct MainGUI
{
    data_manager: Arc<Mutex<DataManager>>
}

#[derive(Debug, Clone)]
pub enum GUIMessage
{
    OpenFile,
    FileSelected(String),
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
            {data_manager: flags},
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
                    Command::perform(async move { file_path }, GUIMessage::FileSelected)
                }
                else {Command::none()}
            }

            GUIMessage::FileSelected(path) =>
            {
                self.data_manager.lock().unwrap().load_data(path.as_str());
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message>
    {
        Row::new()
            .push(Space::with_width(Length::FillPortion(3)))
            .push
            (
                Column::new()
                    .push(Space::with_height(Length::FillPortion(1)))
                    .push
                    (
                        button("Load file")
                            .on_press(GUIMessage::OpenFile)
                            .padding(10)
                    )
                    .push(Space::with_height(Length::FillPortion(8)))
            )
            .push(Space::with_width(Length::FillPortion(3)))
            .into()
    }

    fn theme(&self) -> Self::Theme {Theme::GruvboxDark}
}
