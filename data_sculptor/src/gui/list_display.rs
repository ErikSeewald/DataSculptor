use std::sync::{Arc, Mutex};
use iced::{Element, Length};
use iced::widget::{Column, Container, Scrollable, Text};
use crate::core::data_manager::DataManager;
use crate::gui::gui_core::GUIMessage;

pub fn display_list(list: &Arc<Mutex<DataManager>>) -> Element<GUIMessage>
{
    let mut column: Column<GUIMessage> = Column::new().spacing(10);

    for day in &list.lock().unwrap().data
    {
        let date_text = Text::new(day.date.date_string.clone()).size(20);
        column = column.push(date_text);

        let mut entries_column = Column::new().spacing(10);
        for (key, value) in &day.entries
        {
            let entry_text = Text::new
                (
                    format!("        {}:   \"{}\"", key.title, value.string_value)
                );

            entries_column = entries_column.push(entry_text);
        }

        column = column.push(entries_column);
    }

    Scrollable::new
        (
            Container::new(column)
                .width(Length::Fill)
                .center_x()
        )
        .into()
}