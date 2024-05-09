//! Module implementing the control functions for the [`FilterView`]

use std::collections::HashMap;
use iced::{Command};
use crate::gui::gui_message::GUIMessage;
use crate::core::filters::filter::{FilterType, Filter, FilterID};

/// View for displaying and setting filters for the data list
pub struct FilterView
{
    pub filter_type: FilterType,
    pub filters: HashMap<FilterID, Filter>,
    pub selected_filter: Option<FilterID>
}

impl From<FilterType> for FilterView
{
    fn from(filter_type: FilterType) -> Self
    {
        Self
        {
            filter_type,
            filters: HashMap::new(),
            selected_filter: None
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
            _ => {Command::none()}
        }
    }

    fn click_filter(&mut self, filter_id: FilterID) -> Command<GUIMessage>
    {
        if let Some(selected_id) = &self.selected_filter
        {
            if selected_id == &filter_id
            {
                self.filters.remove(selected_id);
            }
        }

        self.selected_filter = Some(filter_id);
        Command::none()
    }
}

