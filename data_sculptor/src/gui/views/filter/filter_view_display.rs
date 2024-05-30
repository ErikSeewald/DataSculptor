//! Module implementing the display functions for the [`FilterView`]

use iced::{Alignment, Element, Length, theme};
use iced::widget::{Button, Column, Container, Row, Scrollable, Space, text_input};
use crate::gui::gui_message::GUIMessage;
use crate::gui::gui_theme;
use crate::gui::views::filter::filter_view_control::FilterView;
use crate::gui::views::list::list_view_control::ListView;

/// Implementation of the display functions for the list view
impl FilterView
{
    pub fn view(&self) -> Element<GUIMessage>
    {
        // SAVE AND EXIT
        let save_button = Button::new("Save and exit")
            .on_press(GUIMessage::ReturnToView(ListView::view_title()))
            .padding(10)
            .style(theme::Button::custom(gui_theme::ButtonTheme));

        // INPUT ROW
        let text_input = text_input(
            "Filter syntax description in README.md",
            &self.input_value
        )
            .on_input(GUIMessage::FilterInputChanged)
            .on_submit(GUIMessage::AddFilter)
            .padding(10)
            .size(20);

        let add_button = Button::new("Add filter")
            .on_press(GUIMessage::AddFilter)
            .padding(10)
            .style(theme::Button::custom(gui_theme::ButtonTheme));

        let input_row = Row::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(text_input)
            .push(add_button);

        let input_row_container = Container::new(input_row)
            .padding(20)
            .style(gui_theme::container_bar_style());

        // FILTERS
        let filter_list = self.display_filter_list();

        // CONSTRUCT
        Column::new()
            .push
            (
                Row::new()
                    .push(save_button)
                    .padding(8)
            )
            .push(input_row_container)
            .push(Space::with_height(15))
            .push(filter_list)
            .into()
    }

    pub fn display_filter_list(&self) -> Element<GUIMessage>
    {
        let mut filter_column = Column::new().spacing(20);

        let mut filter_index = 0;
        let mut current_row = Row::new();
        for (id, filter) in &self.filters
        {
            if filter_index % 3 == 0
            {
                filter_column = filter_column.push(current_row);
                current_row = Row::new().spacing(20)
            }

            // FILTER BOX
            let delete_button = Button::new("X")
                .style(theme::Button::custom(gui_theme::DeleteButtonTheme))
                .padding([7, 10])
                .width(Length::Fixed(30.0))
                .on_press(GUIMessage::DeleteFilter(id.clone()));

            let filter_button = Button::new(filter.title.as_str())
                .style(theme::Button::custom(gui_theme::FilterButtonTheme))
                .clip(true)
                .padding(20)
                .width(Length::Fixed(250.0))
                .on_press(GUIMessage::ClickFilter(id.clone()));

            let filter_box = Row::new()
                .spacing(0)
                .push(filter_button)
                .push(delete_button);

            current_row = current_row.push(filter_box);
            filter_index += 1;
        }
        filter_column = filter_column.push(current_row);


        Scrollable::new
            (
                Container::new(filter_column)
                    .width(Length::Fill)
                    .center_x()
            )
            .into()
    }
}