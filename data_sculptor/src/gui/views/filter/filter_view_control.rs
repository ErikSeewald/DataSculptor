//! Module implementing the control functions for the [`FilterView`]

use iced::{Command};
use crate::gui::gui_message::GUIMessage;
use crate::core::filters::filter::{FilterType, Filter, FilterID};
use crate::core::filters::{expression_parser};
use indexmap::IndexMap;

/// View for displaying and setting filters for the data list
pub struct FilterView
{
    pub filter_type: FilterType,
    pub filters: IndexMap<FilterID, Filter>, // index map to preserve order in list display
    pub selected_filter: Option<FilterID>,
    pub(crate) input_value: String,
}

impl From<FilterType> for FilterView
{
    fn from(filter_type: FilterType) -> Self
    {
        Self
        {
            filter_type,
            filters: IndexMap::new(),
            selected_filter: None,
            input_value: String::new(),
        }
    }
}

/// Implementation of the control functions for the list view
impl FilterView
{
    // UPDATE
    pub fn update(&mut self, message: GUIMessage) -> Command<GUIMessage>
    {
        match message
        {
            GUIMessage::ClickFilter(filter_id) => {self.click_filter(filter_id)}
            GUIMessage::FilterInputChanged(input) => {self.update_input(input)}
            GUIMessage::AddFilter => {self.add_filter()}
            _ => {Command::none()}
        }
    }

    fn click_filter(&mut self, filter_id: FilterID) -> Command<GUIMessage>
    {
        if let Some(selected_id) = &self.selected_filter
        {
            if selected_id == &filter_id
            {
                self.filters.shift_remove(selected_id);
                self.selected_filter = None;
                return Command::none();
            }
        }

        self.selected_filter = Some(filter_id);
        Command::none()
    }

    fn update_input(&mut self, input: String) -> Command<GUIMessage>
    {
        if input.len() < 500
        {
            self.input_value = input;
        }

        Command::none()
    }

    fn add_filter(&mut self) -> Command<GUIMessage>
    {
        let parse_result = expression_parser::parse(&self.filter_type, self.input_value.as_str());
        if let Some(filter_expression) = parse_result
        {
            self.filters.insert
            (
                FilterID::from(&filter_expression),
                Filter
                {
                    title: self.input_value.clone(),
                    expression: filter_expression,
                    filter_type: self.filter_type.clone()
                }
            );
            self.input_value.clear();
        }

        Command::none()
    }

    /// Handles updating variables when the [`FilterView`] is exited.
    pub fn exit_view(&mut self)
    {
        self.selected_filter = None;
    }
}
