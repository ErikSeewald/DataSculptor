//! Module implementing ways to filter data in data_sculptor.

use crate::core::data_containers::{DateKey, EntryRef};

/// Enum representing all supported ways to filter data in data_sculptor.
pub enum FilterCommand
{
    Contains(String)
}

impl FilterCommand
{
    /// Checks the given [`DateKey`] and returns whether it is valid under the rules
    /// of the [`FilterCommand`] that implements this function.
    ///
    /// # Returns
    /// - filtered out (i.e. value invalid): false
    ///
    /// - not filtered out (i.e. value valid): true
    pub fn apply_date_filter(&self, date: &DateKey, ) -> bool
    {
        match self
        {
            FilterCommand::Contains(keyword) => date.date_string.contains(keyword),
        }
    }

    /// Checks the given [`EntryRef`] and returns whether its key is valid under the rules
    /// of the [`FilterCommand`] that implements this function.
    ///
    /// # Returns
    /// - filtered out (i.e. value invalid): false
    ///
    /// - not filtered out (i.e. value valid): true
    pub fn apply_key_filter(&self, entry: &EntryRef) -> bool
    {
        match self
        {
            FilterCommand::Contains(keyword) => entry.key.title.contains(keyword),
        }
    }

    /// Checks the given [`EntryRef`] and returns whether its value is valid under the rules
    /// of the [`FilterCommand`] that implements this function.
    ///
    /// # Returns
    /// - filtered out (i.e. value invalid): false
    ///
    /// - not filtered out (i.e. value valid): true
    pub fn apply_value_filter(&self, entry: &EntryRef) -> bool
    {
        match self
        {
            FilterCommand::Contains(keyword) => entry.value.string_value.contains(keyword),
        }
    }
}