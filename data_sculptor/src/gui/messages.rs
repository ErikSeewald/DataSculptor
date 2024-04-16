use iced::{Color, Element, Length};
use iced::widget::{Column, Container, Text, text};
use crate::gui::gui_core::{GUIMessage, MainGUI};

pub fn build_message_container(state: &MainGUI) -> Element<GUIMessage>
{
    let mut msg_column: Column<GUIMessage> = Column::new().spacing(20);

    if !state.loaded_valid_file
    {
        msg_column = file_load_error(
            msg_column, state.load_error_msg.clone());
    }

    else if state.loading_file
    {
        msg_column = loading_message(msg_column);
    }

    Container::new(msg_column)
        .width(Length::Fill)
        .center_x()
        .into()
}

fn file_load_error(msg_column: Column<GUIMessage>, error_msg: String)
    -> Column<GUIMessage>
{
    let error_color = Color::new(1.0, 0.2, 0.2, 1.0);

    msg_column.push(Text::new("Error while loading file:").size(25).style(error_color))
        .push(Text::new(error_msg).size(15).style(error_color))
}

fn loading_message(msg_column: Column<GUIMessage>) -> Column<GUIMessage>
{
    msg_column.push(
        Text::new("Loading..")
            .size(25)
    )
}