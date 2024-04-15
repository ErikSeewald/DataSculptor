use iced::{Element, Length};
use iced::widget::{Column, Container, Text};
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
    msg_column.push(Text::new("Error while loading file:").size(25))
        .push(Text::new(error_msg).size(15))
}

fn loading_message(msg_column: Column<GUIMessage>) -> Column<GUIMessage>
{
    msg_column.push(
        Text::new("Loading..")
            .size(25)
    )
}