//! Module handling the display of the list of filters for a specific [`FilterType`]

use iced::{Alignment, Element, Length, theme};
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
    for filter in filter_view.filters.iter()
    {
        if filter_index % 4 == 0
        {
            filter_column = filter_column.push(current_row);
            current_row = Row::new().spacing(20)
        }

        current_row = current_row.push
        (
            Button::new(filter.title.as_str())
            .on_press(GUIMessage::SelectFile)
            .padding(20)
            .style(theme::Button::custom(gui_theme::FilterButtonTheme))
        );

        filter_index += 1;
    }

    Scrollable::new
        (
            Container::new(filter_column)
                .width(Length::Fill)
                .center_x()
        )
        .into()
}