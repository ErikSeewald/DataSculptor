use std::hash::{Hash, Hasher};
use crate::core::filters::command_parser;
use crate::core::filters::filter_expression::EvalData;

/// Enum representing all supported ways to filter data in data_sculptor.
pub enum FilterCommand
{
    /// Checks if the value contains the given String
    Contains(String),

    /// Checks if the value corresponding to the given key contains the given String
    ///
    /// # Args (index order)
    /// - `key`: String title of the key of the key-value pair to check
    /// - `keyword`: String that the value should contain
    KeyValueContains(String, String),

    /// Tries to parse the value to a number by removing all non number characters
    /// and does an operation on it.  If it fails to parse, false is returned.
    /// If it parses successfully, the number is compared to the given number with the given
    /// operation.
    ///
    /// # Args (index order)
    /// - `op`: String representation of an operator. Supported: '>', '<'
    /// - `num`: Number to compare the value to
    ///
    /// # Examples
    /// - Numop('>', 8) will return true if the value parses to a number and that number
    /// is greater than 8
    NumOp(String, f32),


    /// Tries to parse the value corresponding to the given key to a number by removing
    /// all non number characters and does an operation on it.
    /// If it fails to parse, false is returned.
    /// If it parses successfully, the number is compared to the given number with the given
    /// operation.
    ///
    /// # Args (index order)
    /// - `key`: String title of the key of the key-value pair to check
    /// - `op`: String representation of an operator. Supported: '>', '<'
    /// - `num`: Number to compare the value to
    ///
    /// # Examples
    /// - Numop('speed', '<', 12) will return true if the value corresponding to 'speed' parses
    /// successfully and is less than 12
    KeyValueNumOp(String, String, f32)
}

impl Hash for FilterCommand
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        match self
        {
            FilterCommand::Contains(s) =>
                {
                    command_parser::Keywords::Contains.hash(state);
                    s.hash(state)
                }

            FilterCommand::KeyValueContains(k, s) =>
                {
                    command_parser::Keywords::KeyValueContains.hash(state);
                    k.hash(state);
                    s.hash(state)
                }
            FilterCommand::NumOp(o, f) =>
                {
                    command_parser::Keywords::NumOp.hash(state);
                    o.hash(state);
                    f.to_bits().hash(state)
                },

            FilterCommand::KeyValueNumOp(k, o, f) =>
                {
                    command_parser::Keywords::KeyValueNumOp.hash(state);
                    k.hash(state);
                    o.hash(state);
                    f.to_bits().hash(state)
                }
        }
    }
}

impl FilterCommand
{
    /// Applies the [`FilterCommand`] to the given [`EvalData`] under date filter rules.
    ///
    /// # Returns
    /// * `false` - filtered out (i.e. data invalid):
    /// * `true` - not filtered out (i.e. data valid):
    pub fn apply_date_filter(&self, data: &EvalData) -> bool
    {
        match self
        {
            FilterCommand::Contains(keyword) =>
                {
                    data.day.date.date_string.contains(keyword)
                },

            FilterCommand::NumOp(op, num) =>
                {
                    num_op(&data.day.date.date_string, op, num)
                },

            _ => {true}
        }
    }

    /// Applies the [`FilterCommand`] to the given [`EvalData`] under key filter rules.
    ///
    /// # Returns
    /// * `false` - filtered out (i.e. data invalid):
    /// * `true` - not filtered out (i.e. data valid):
    pub fn apply_key_filter(&self, data: &EvalData) -> bool
    {
        match self
        {
            FilterCommand::Contains(keyword) =>
                {
                    data.key.title.contains(keyword)
                },

            FilterCommand::NumOp(op, num) =>
                {
                    num_op(&data.key.title, op, num)
                },

            _ => {true}
        }
    }

    /// Applies the [`FilterCommand`] to the given [`EvalData`] under value filter rules.
    ///
    /// # Returns
    /// * `false` - filtered out (i.e. data invalid):
    /// * `true` - not filtered out (i.e. data valid):
    pub fn apply_value_filter(&self, data: &EvalData) -> bool
    {
        match self
        {
            FilterCommand::KeyValueContains(key, keyword) =>
                {
                    for (day_key, day_value) in &data.day.entries
                    {
                        if &day_key.title == key
                        {
                            return day_value.string_value.contains(keyword);
                        }
                    }

                    return true;
                },

            FilterCommand::KeyValueNumOp(key, op, num) =>
                {
                    for (day_key, day_value) in &data.day.entries
                    {
                        if &day_key.title == key
                        {
                            return num_op(&day_value.string_value, op, num);
                        }
                    }

                    return true;
                },

            _ => {true}
        }
    }
}

/// Tries to parse the value to a number by removing all non number characters
/// and does an operation on it.  If it fails to parse, false is returned.
/// If it parses successfully, the number is compared to the given number with the given
/// operation.
///
/// # Args (index order)
/// - 'value': The value to parse
/// - `op`: String representation of an operator. Supported: '>', '<'
/// - `num`: Number to compare the value to
fn num_op(value: &String, op: &String, num: &f32) -> bool
{
    let num_string = remove_all_non_num_chars(value);
    match num_string.parse::<f32>()
    {
        Ok(parsed_value) =>
        {
            match op.as_str()
            {
                ">" => parsed_value > *num,
                "<" => parsed_value < *num,
                _ => false,
            }
        },

        Err(_) => false,
    }
}

fn remove_all_non_num_chars(value: &String) -> String
{
    // A lot faster than using regex apparently
    value.chars()
        .filter(|&c| c.is_digit(10) || c == '.')
        .collect()
}