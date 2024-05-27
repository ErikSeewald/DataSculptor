//! Module for parsing [`String`]s into [`FilterCommand`]s.
use chrono::NaiveDate;
use crate::core::data_containers::{DATE_FORMAT};
use crate::core::filters::filter::FilterType;
use crate::core::filters::filter_commands::FilterCommand;

/// Keywords corresponding to [`FilterCommand`]s.
#[derive(Hash)]
pub enum Keywords
{
    Contains,
    KeyValueContains,
    NumOp,
    KeyValueNumOp,
    Date
}

impl Keywords
{
    /// The &str representation of the [`FilterCommand`] Keyword
    pub fn cmd_str(&self) -> &'static str
    {
        match &self
        {
            Keywords::Contains => "contains",
            Keywords::KeyValueContains => "kv-contains",
            Keywords::NumOp => "numop",
            Keywords::KeyValueNumOp => "kv-numop",
            Keywords::Date => "date"
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
/// - `Some`([`FilterCommand`]) if the command was successfully parsed
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

    if input.starts_with(Keywords::NumOp.cmd_str())
    {
        if filter_type != &FilterType::Value
        {
            return parse_numop(&input[Keywords::NumOp.cmd_len()..]);
        }
    }

    if input.starts_with(Keywords::KeyValueNumOp.cmd_str())
    {
        if filter_type == &FilterType::Value
        {
            return parse_kv_numop(&input[Keywords::KeyValueNumOp.cmd_len()..]);
        }
    }

    if input.starts_with(Keywords::Date.cmd_str())
    {
        if filter_type == &FilterType::Date
        {
            return parse_date_op(&input[Keywords::Date.cmd_len()..]);
        }
    }

    None
}

fn parse_contains(input: &str) -> Option<FilterCommand>
{
    let contains_string = get_quotation_string_and_verify_is_last(input)?;
    Some(FilterCommand::Contains(contains_string))
}

/// Gets the string keyword inside quotation marks for 'contains' filters.
/// Verifies that the keyword is the very last part of the input. Returns None if that
/// is not the case.
fn get_quotation_string_and_verify_is_last(input: &str) -> Option<String>
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
    let contains_string = get_quotation_string_and_verify_is_last(&input[end + 1..])?;
    Some(FilterCommand::KeyValueContains(key, contains_string))
}

fn parse_numop(input: &str) -> Option<FilterCommand>
{
    let (op, num) = get_numop_params(input)?;
    return Some(FilterCommand::NumOp(op, num));
}

fn get_numop_params(input: &str) -> Option<(String, f32)>
{
    let start = input.find(" \"")?;

    if start != 0
    {
        return None;
    }

    let end = input[start + 2..].find("\"")? + start + 2;
    let op = &input[start + 2..end];

    let num_string = get_quotation_string_and_verify_is_last(&input[end + 1..])?;

    let num = match num_string.parse::<f32>()
    {
        Ok(num) => num,
        Err(_) => return None
    };

    return Some((String::from(op), num));
}

fn parse_kv_numop(input: &str) -> Option<FilterCommand>
{
    // KEY
    let start = input.find(" \"")?;

    if start != 0
    {
        return None;
    }

    let end = input[start + 2..].find("\"")? + start + 2;
    let key = String::from(&input[start + 2..end]);

    let (op, num) = get_numop_params(&input[end + 1..])?;
    return Some(FilterCommand::KeyValueNumOp(key, op, num));
}

fn parse_date_op(input: &str) -> Option<FilterCommand>
{
    let start: usize;
    let mut op = String::new();

    if let Some(index) = input.find("before")
    {
        start = index + 6;
        op += "before";
    }

    else if let Some(index) = input.find("after")
    {
        start = index + 5;
        op += "after";
    }

    else
    {
        return None;
    }

    let date_string = get_quotation_string_and_verify_is_last(&input[start..])?;
    let date = NaiveDate::parse_from_str(date_string.as_str(), DATE_FORMAT).ok()?;
    return Some(FilterCommand::Date(op, date));
}