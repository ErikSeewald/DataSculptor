//! Defines the [`GUIMessage`] enum

use crate::core::filters::filter::{FilterID, FilterType};

/// Enum holding all types of messages to be received and handled by the gui application.
/// Sorted by the view module they belong to.
#[derive(Debug, Clone)]
pub enum GUIMessage
{
    // GENERAL
    ReturnToView(&'static str),

    // LIST VIEW
    SelectFile,
    FileSelected(String),
    OpenFilterView(FilterType),

    // FILTER VIEW
    ClickFilter(FilterID),
    FilterInputChanged(String),
    AddFilter,
    DeleteFilter(FilterID),
}