//! Module handling the display of the list of filters for a specific [`FilterType`]

use iced::{Element, Length, theme};
use iced::widget::{Button, Column, Container, Row, Scrollable};
use crate::gui::gui_message::GUIMessage;
use crate::gui::gui_theme;
use crate::gui::views::filter::filter_view::FilterView;

/// Displays all filters of the given [`FilterView`]
pub fn display_filter_list(filter_view: &FilterView) -> Element<GUIMessage>
{
    let mut filter_column = Column::new().spacing(20);

    let mut filter_index = 0;
    let mut current_row = Row::new();
    for (id, filter) in filter_view.filters.iter()
    {
        if filter_index % 3 == 0
        {
            filter_column = filter_column.push(current_row);
            current_row = Row::new().spacing(20)
        }

        // FILTER BUTTON
        let mut draw_as_selected = false;
        if let Some(selected_filter) = &filter_view.selected_filter
        {
            draw_as_selected = selected_filter == id;
        }

        let mut filter_button: Button<GUIMessage> =
            if draw_as_selected
            {
                Button::new("Click again to delete")
                    .style(theme::Button::custom(gui_theme::FilterButtonSelectedTheme))
            }
            else
            {
                Button::new(filter.title.as_str())
                    .style(theme::Button::custom(gui_theme::FilterButtonTheme))
            };
        filter_button = filter_button
            .padding(20)
            .width(Length::Fixed(250.0))
            .on_press(GUIMessage::ClickFilter(id.clone()));

        current_row = current_row.push(filter_button);
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