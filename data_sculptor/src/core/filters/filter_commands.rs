//! Module implementing ways to filter data in data_sculptor.

use crate::core::data_containers::{DateKey, EntryRef};

/// Enum representing all supported ways to filter data in data_sculptor.
#[derive(Hash)]
pub enum FilterCommand
{
    /// Checks if the value contains the given String
    ///
    /// # Args (index order)
    /// - `invert`: Boolean flag indicating whether the value of the operation should be inverted
    /// - `keyword`: String that the value should contain
    Contains(bool, String),

    /// Checks if the value corresponding to the given key contains the given String
    ///
    /// # Args (index order)
    /// - `invert`: Boolean flag indicating whether the value of the operation should be inverted
    /// - `key`: String title of the key of the key-value pair to check
    /// - `keyword`: String that the value should contain
    KeyValueContains(bool, String, String)
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
            FilterCommand::Contains(invert, keyword) =>
                {
                    invert ^ date.date_string.contains(keyword)
                },
            _ => {true}
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
            FilterCommand::Contains(invert, keyword) =>
                {
                    invert ^ entry.key.title.contains(keyword)
                },
            _ => {true}
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
            FilterCommand::KeyValueContains(invert, key, keyword) =>
                {
                    if key == &entry.key.title
                    {
                        return invert ^ entry.value.string_value.contains(keyword);
                    }
                    return true; // accept all other key-value pairs by default
                }
            _ => {true}
        }
    }
}