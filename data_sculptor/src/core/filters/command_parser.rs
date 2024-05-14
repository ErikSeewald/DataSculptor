//! Module for parsing [`String`]s into [`FilterCommand`]s.

use crate::core::filters::filter::FilterType;
use crate::core::filters::filter::FilterCommand;

/// Keywords corresponding to [`FilterCommand`]s.
enum Keywords
{
    Contains,
    KeyValueContains
}

impl Keywords
{
    /// The &str representation of the [`FilterCommand`] Keyword
    pub fn cmd_str(&self) -> &'static str
    {
        match &self
        {
            Keywords::Contains => {"contains"}
            Keywords::KeyValueContains => {"kv-contains"}
        }
    }
    /// The length of the &str representation of the [`FilterCommand`] Keyword
    pub fn cmd_len(&self) -> usize
    {
        self.cmd_str().len()
    }
}

/// Tries to parse the given input string to a [`FilterCommand`]
///
/// # Returns
/// - `Some([`FilterCommand`])` if the command was successfully parsed
/// - `None` if parsing was unsuccessful
pub fn parse(filter_type: &FilterType, input: String,) -> Option<FilterCommand>
{
    if input.starts_with(Keywords::Contains.cmd_str())
    {
        if filter_type != &FilterType::Value
        {
            return parse_contains(&input[Keywords::Contains.cmd_len()..]);
        }
    }

    if input.starts_with(Keywords::KeyValueContains.cmd_str())
    {
        if filter_type == &FilterType::Value
        {
            return parse_kv_contains(&input[Keywords::KeyValueContains.cmd_len()..])
        }
    }

    None
}

fn parse_contains(input: &str) -> Option<FilterCommand>
{
    let contains_string = get_contains_string_and_very_position(input)?;
    Some(FilterCommand::Contains(contains_string))
}

/// Gets the string keyword inside quotation marks for 'contains' filters.
/// Verifies that the keyword is the very last part of the input. Returns None if that
/// is not the case.
fn get_contains_string_and_very_position(input: &str) -> Option<String>
{
    let start = input.find(" \"")?;

    if start != 0
    {
        return None;
    }

    let end = input[start + 2..].find("\"")? + start + 2;
    if end + 1 != input.len()
    {
        return None
    }

    return Some(String::from(&input[start + 2..end]))
}

fn parse_kv_contains(input: &str) -> Option<FilterCommand>
{
    // KEY
    let start = input.find(" \"")?;

    if start != 0
    {
        return None;
    }

    let end = input[start + 2..].find("\"")? + start + 2;
    let key = String::from(&input[start + 2..end]);

    // CONTAINS
    let contains_string = get_contains_string_and_very_position(&input[end + 1..])?;
    Some(FilterCommand::KeyValueContains(key, contains_string))
}