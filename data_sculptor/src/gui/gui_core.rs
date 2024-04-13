use iced::widget::{button, Button, Column, Text};
use iced::{Sandbox, Settings};

pub fn init() -> iced::Result
{
    MainGUI::run(Settings::default())
}

struct MainGUI
{
    button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum GUIMessage
{
    ButtonPressed,
}

impl Sandbox for MainGUI
{
    type Message = GUIMessage;

    fn new() -> Self
    {
        Self
        {
            button_state: button::State::new(),
        }
    }

    fn title(&self) -> String
    {
        String::from("Data Sculptor")
    }

    fn update(&mut self, message: Self::Message)
    {
        match message
        {
            GUIMessage::ButtonPressed => {println!("Button pressed!");}
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message>
    {
        Column::new()
            .push(
                Button::new(Text::new("Press me!"))
                    .on_press(GUIMessage::ButtonPressed),
            )
            .into()
    }
}