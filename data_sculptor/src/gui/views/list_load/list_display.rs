use std::sync::{Arc, Mutex};
use iced::{Color, Element, Length};
use iced::widget::{Column, Container, Row, Scrollable, Space, Text};
use crate::core::data_manager::DataManager;
use crate::gui::gui_message::GUIMessage;

pub fn display_list(list: &Arc<Mutex<DataManager>>) -> Element<GUIMessage>
{
    let date_color = Color::new(0.4, 0.8, 0.5, 1.0);
    let value_color = Color::new(0.6, 0.8, 1.0, 1.0);

    let mut column: Column<GUIMessage> = Column::new().spacing(10);

    for day in &list.lock().unwrap().data
    {
        // DATE
        let date_text = Text::new(day.date.date_string.clone())
            .size(20)
            .style(date_color);

        column = column
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(date_text);

        // ENTRIES
        let mut entries_column = Column::new().spacing(10);
        for (key, value) in &day.entries
        {
            entries_column = entries_column.push
            (
                Row::new()
                    .push(Text::new(format!("        {}:", key.title)))
                    .push(Text::new(format!("   \"{}\"", value.string_value)).style(value_color))
            );
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