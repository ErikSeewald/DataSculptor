//! Module implementing the display functions for the [`ListView`]

use std::sync::{Arc, Mutex};
use iced::{Color, Element, Length, theme};
use iced::widget::{button, Column, Container, Row, Scrollable, Space, Text};
use crate::core::data_containers::{DateKey, EntryRef};
use crate::core::data_manager::DataManager;
use crate::core::filters::filter::FilterType;
use crate::gui::gui_message::GUIMessage;
use crate::gui::gui_theme;
use crate::gui::views::list_load::list_view_control::ListView;

/// Implementation of the display functions for the list view
impl ListView
{
    pub fn view<'a>(&'a self, data_manager: &'a Arc<Mutex<DataManager>>) -> Element<GUIMessage>
    {
        // SHOW FILTER VIEW IF ONE IS OPENED
        if let Some(filter_view) = &self.opened_filter_view
        {
            return self.filter_views.get(filter_view).unwrap().view();
        }

        //TOP ROW
        let top_row: Element<GUIMessage> = Row::new()
            .push
            (
                button("Date filters")
                    .on_press(GUIMessage::OpenFilterView(FilterType::Date))
                    .padding(10)
                    .style(theme::Button::custom(gui_theme::ButtonTheme))
            )
            .push
            (
                button("Key filters")
                    .on_press(GUIMessage::OpenFilterView(FilterType::Key))
                    .padding(10)
                    .style(theme::Button::custom(gui_theme::ButtonTheme))
            )
            .push
            (
                button("Value filters")
                    .on_press(GUIMessage::OpenFilterView(FilterType::Value))
                    .padding(10)
                    .style(theme::Button::custom(gui_theme::ButtonTheme))
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
                    .style(theme::Button::custom(gui_theme::ButtonTheme))
            )
            .push
            (
                Space::with_width(Length::FillPortion(1))
            )
            .spacing(20).into();

        let top_row_container = Container::new(top_row)
            .padding(20)
            .style(gui_theme::container_bar_style());

        let msg_container = self.build_message_container();
        let data_list_display = self.display_list(data_manager);

        Column::new()
            .push(Space::with_height(15))
            .push(top_row_container)
            .push(Space::with_height(20))
            .push(msg_container)
            .push(data_list_display)
            .into()
    }

    fn display_list(&self, data_manager: &Arc<Mutex<DataManager>>) -> Element<GUIMessage>
    {
        let date_color = Color::new(0.4, 0.8, 0.5, 1.0);
        let value_color = Color::new(0.6, 0.8, 1.0, 1.0);

        let mut column: Column<GUIMessage> = Column::new().spacing(10);

        for day in &data_manager.lock().unwrap().data
        {
            // DATE
            if !self.filter_date(&day.date)
            {
                continue; // skip filtered dates
            }

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
                if !self.filter_key_and_value(&EntryRef{date: &day.date, key, value})
                {
                    continue; // skip filtered keys/values
                }

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

    /// Runs the given [`DateKey`] through all currently active
    /// filters and returns whether it is valid to show
    fn filter_date(&self, date: &DateKey) -> bool
    {
        for (_, filter) in &self.filter_views.get(&FilterType::Date).unwrap().filters
        {
            if !filter.command.apply_date_filter(date)
            {
                return false;
            }
        }
        return true;
    }

    /// Runs the key and value in the given [`EntryRef`] through all currently active
    /// filters and returns whether it is valid to show
    fn filter_key_and_value(&self, entry: &EntryRef) -> bool
    {
        // KEY
        for (_, filter) in &self.filter_views.get(&FilterType::Key).unwrap().filters
        {
            if !filter.command.apply_key_filter(entry)
            {
                return false;
            }
        }

        // VALUE
        for (_, filter) in &self.filter_views.get(&FilterType::Value).unwrap().filters
        {
            if !filter.command.apply_value_filter(entry)
            {
                return false;
            }
        }

        return true;
    }

    /// Builds the message container with the correct error message based on the state
    /// of the given [`ListView`].
    pub fn build_message_container(&self) -> Element<GUIMessage>
    {
        let mut msg_column: Column<GUIMessage> = Column::new().spacing(20);

        if !self.loaded_valid_file
        {
            msg_column = file_load_error(msg_column, self.load_error_msg.clone());
        }

        else if self.loading_file
        {
            msg_column = loading_message(msg_column);
        }

        Container::new(msg_column)
            .width(Length::Fill)
            .center_x()
            .into()
    }
}

fn file_load_error(msg_column: Column<GUIMessage>, error_msg: String)
                   -> Column<GUIMessage>
{
    let error_color = Color::new(1.0, 0.2, 0.2, 1.0);

    msg_column
        .push
        (
            Text::new("Error while loading file:")
                .size(25)
                .style(error_color)
        )
        .push(Text::new(error_msg).size(15).style(error_color))
}

fn loading_message(msg_column: Column<GUIMessage>) -> Column<GUIMessage>
{
    msg_column
        .push
        (
            Text::new("Loading..")
                .size(25)
        )
}