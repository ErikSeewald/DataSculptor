use iced::widget::{button, Column, Container, Row, scrollable, Scrollable, Space, Text};
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
        let mut column: Column<Self::Message> = Column::new().spacing(20);
        for data in &self.data_manager.lock().unwrap().data
        {
            let date_text = Text::new(data.date.date_string.clone()).size(20);
            column = column.push(date_text);

            let mut entries_column = Column::new().spacing(10);
            for (key, value) in &data.entries
            {
                let entry_text = Text::new(format!("    {}: {}", key.title, value.string_value));
                entries_column = entries_column.push(entry_text);
            }

            column = column.push(entries_column);
        }

        let scroll = Scrollable::new(Container::new(column).width(Length::Fill).center_x());

        let top_row = Row::new()
            .push(Space::with_width(Length::FillPortion(3)))
            .push
            (
                button("Load file")
                    .on_press(GUIMessage::OpenFile)
                    .padding(10)
            )
            .push(Space::with_width(Length::FillPortion(3)));

        Column::new()
            .spacing(10)
            .push(Space::with_height(10))
            .push(top_row)
            .push(scroll)
            .into()
    }

    fn theme(&self) -> Self::Theme {Theme::GruvboxDark}
}
