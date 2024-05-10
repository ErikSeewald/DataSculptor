//! Module for parsing [`String`]s into [`FilterCommand`]s.

use crate::core::filters::filter::FilterType;
use crate::core::filters::filter_commands::FilterCommand;

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
    let invert: bool;
    let not_keyword = "not ";
    if input.starts_with(not_keyword)
    {
        invert = true;
        parse_command(filter_type, &input[not_keyword.len()..], invert)
    }

    else
    {
        invert = false;
        parse_command(filter_type, &input, invert)
    }
}

/// Parses the given command, assuming that the invert flag has already been set and the
/// invert keyword has been cut out of the input &str
fn parse_command(filter_type: &FilterType, input: &str, invert: bool) -> Option<FilterCommand>
{
    if input.starts_with(Keywords::Contains.cmd_str()) && filter_type != &FilterType::Value
    {
        return parse_contains(&input[Keywords::Contains.cmd_len()..], invert);
    }

    if input.starts_with(Keywords::KeyValueContains.cmd_str()) && filter_type == &FilterType::Value
    {
        return parse_kv_contains(&input[Keywords::KeyValueContains.cmd_len()..], invert)
    }

    None
}

fn parse_contains(input: &str, invert: bool) -> Option<FilterCommand>
{
    let contains_string = get_contains_string_and_very_position(input)?;
    Some(FilterCommand::Contains(invert, contains_string))
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

fn parse_kv_contains(input: &str, invert: bool,) -> Option<FilterCommand>
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
    Some(FilterCommand::KeyValueContains(invert, key, contains_string))
}