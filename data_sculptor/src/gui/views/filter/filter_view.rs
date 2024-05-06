//! Module implementing the [`FilterView`]

use iced::{Command, Element, Alignment, theme};
use iced::widget::{Button, Column, Container, Row, Space, TextInput};
use crate::gui::gui_message::GUIMessage;
use crate::core::filter::{FilterType, Filter};
use crate::gui::views::list_load::list_load_view::ListLoadView;
use crate::gui::gui_theme;
use crate::gui::views::filter::filter_list_display::display_filter_list;

/// View for displaying and setting filters for the data list
pub struct FilterView
{
    pub filter_type: FilterType,
    pub filters: Vec<Filter>
}

impl From<FilterType> for FilterView
{
    fn from(filter_type: FilterType) -> Self
    {
        Self
        {
            filter_type,
            filters: Vec::new()
        }
    }
}

impl FilterView
{
    // UPDATE
    pub fn update(&mut self, message: GUIMessage) -> Command<GUIMessage>
    {
        match message
        {
            _ => {Command::none()}
        }
    }

    // VIEW
    pub fn view(&self) -> Element<GUIMessage>
    {
        // SAVE AND EXIT
        let save_button = Button::new("Save and exit")
            .on_press(GUIMessage::ReturnToView(ListLoadView::view_title()))
            .padding(10)
            .style(theme::Button::custom(gui_theme::ButtonTheme));

        // INPUT ROW
        let text_input = TextInput::new(
            "Eh",
            "Enter text...",
        )
            .padding(10)
            .size(20);

        let add_button = Button::new("Add filter")
            .on_press(GUIMessage::SelectFile)
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
        let filter_list = display_filter_list(&self);

        // CONSTRUCT
        Column::new()
            .padding(20)
            .push(Space::with_height(15))
            .push(Row::new().push(save_button))
            .push(Space::with_height(15))
            .push(input_row_container)
            .push(Space::with_height(15))
            .push(filter_list)
            .into()
    }
}

