//! Module implementing the display functions for the [`ListView`]

use std::sync::{Mutex};
use iced::{Color, Element, Length};
use iced::widget::{button, Column, Container, Row, Scrollable, Space, Text};
use crate::core::data_manager::DataManager;
use crate::core::filters::filter;
use crate::core::filters::filter::{FilterType};
use crate::gui::gui_message::GUIMessage;
use crate::gui::gui_style;
use crate::gui::views::list::list_view_control::ListView;
use crate::gui::views::menu::menu_view_control::MenuView;

/// Implementation of the display functions for the list view
impl ListView
{
    pub fn view(&self, data_manager: &Mutex<DataManager>) -> Element<GUIMessage>
    {
        // SHOW FILTER VIEW IF ONE IS OPENED
        if let Some(filter_view) = &self.opened_filter_view
        {
            return self.get_filter_view(filter_view).view();
        }

        //TOP ROW
        let top_row: Element<GUIMessage> = Row::new()
            .push
            (
                button("Return to menu")
                    .on_press(GUIMessage::ReturnToView(MenuView::view_title()))
                    .padding(10)
                    .style(gui_style::ButtonStyle::style)
            )
            .padding(8).into();

        //SECOND ROW
        let second_row: Element<GUIMessage> = Row::new()
            .push
            (
                button("Date filters")
                    .on_press(GUIMessage::OpenFilterView(FilterType::Date))
                    .padding(10)
                    .style(gui_style::ButtonStyle::style)
            )
            .push
            (
                button("Key filters")
                    .on_press(GUIMessage::OpenFilterView(FilterType::Key))
                    .padding(10)
                    .style(gui_style::ButtonStyle::style)
            )
            .push
            (
                button("Value filters")
                    .on_press(GUIMessage::OpenFilterView(FilterType::Value))
                    .padding(10)
                    .style(gui_style::ButtonStyle::style)
            )
            .push
            (
                Space::with_width(Length::FillPortion(10))
            )
            .push
            (
                button("Save as")
                    .on_press(GUIMessage::SaveFile)
                    .padding(10)
                    .style(gui_style::ButtonStyle::style)
            )
            .push
            (
                button("Select file")
                    .on_press(GUIMessage::SelectFile)
                    .padding(10)
                    .style(gui_style::ButtonStyle::style)
            )
            .push
            (
                Space::with_width(Length::FillPortion(1))
            )
            .spacing(20).into();

        let second_row_container = Container::new(second_row)
            .padding(20)
            .style(gui_style::container_bar_style);

        let msg_container = self.build_message_container();
        let data_list_display = self.display_list(data_manager);

        Column::new()
            .push(Space::with_height(3))
            .push(top_row)
            .push(Space::with_height(3))
            .push(second_row_container)
            .push(Space::with_height(20))
            .push(msg_container)
            .push(data_list_display)
            .into()
    }

    /// Displays the data in the given [`DataManager`] filtered by all
    /// [`FilterView`]s. The filters have the following effect:
    ///
    /// 1. If the *date* filter does not match, the *whole day* is skipped
    ///
    /// 2. If the *key* filter does not match at least one key filter, only the *key* is skipped
    ///
    /// 3. If the *value* filter does not match, the *whole day* is skipped
    fn display_list(&self, data_manager: &Mutex<DataManager>) -> Element<GUIMessage>
    {
        let date_color = Color::new(0.4, 0.8, 0.5, 1.0);
        let value_color = Color::new(0.6, 0.8, 1.0, 1.0);

        let mut column: Column<GUIMessage> = Column::new().spacing(10);

        let date_filters = self.get_filters(&FilterType::Date);
        let key_filters = self.get_filters(&FilterType::Key);
        let value_filters = self.get_filters(&FilterType::Value);

        for day in &data_manager.lock().unwrap().data
        {
            if !filter::filter_day(&day, date_filters, value_filters)
            {
                continue; // skip filtered days (based on date and value filters)
            }

            // DATE
            let date_text = Text::new(day.date.date_string.clone())
                .size(20)
                .color(date_color);

            column = column
                .push(Space::with_height(Length::Fixed(10.0)))
                .push(date_text);

            // ENTRIES
            let mut entries_column = Column::new().spacing(10);
            for (key, value) in &day.entries
            {
                if !filter::filter_key(day, key, key_filters)
                {
                    continue; // do not show keys that are filtered out
                }

                let key_text = Text::new(format!("        {}:", key.title));
                let value_text = Text::new(format!("   \"{}\"", value.string_value))
                    .color(value_color);
                let value_container = Container::new(value_text); // For wrapping

                entries_column = entries_column.push
                (
                    Row::new()
                        .push(key_text)
                        .push(value_container)
                );
            }
            column = column.push(entries_column);
        }


        Scrollable::new
            (
                Container::new(column)
                    .center_x(Length::Fill)
            )
            .into()
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
            .center_x(Length::Fill)
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
                .color(error_color)
        )
        .push(Text::new(error_msg).size(15).color(error_color))
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