use crate::core::data_containers::{DayDataParsed, EntryKey};
use crate::core::filters::filter::FilterType;
use crate::core::filters::filter_commands::FilterCommand;

/// A logical expression made up of [`FilterCommand`]s connected
/// to each other by logical operators like 'and', 'or', etc.
#[derive(Hash)]
pub enum FilterExpression
{
    SingleCommand(FilterCommand),
    Not(Box<FilterExpression>),
    And(Box<FilterExpression>, Box<FilterExpression>),
    Or(Box<FilterExpression>, Box<FilterExpression>),
    Xor(Box<FilterExpression>, Box<FilterExpression>),
    Nor(Box<FilterExpression>, Box<FilterExpression>),
    Nand(Box<FilterExpression>, Box<FilterExpression>),
    Xnor(Box<FilterExpression>, Box<FilterExpression>),
}

/// Struct holding references to all data needed for a [`FilterExpression`]s evaluation function.
pub struct EvalData<'a>
{
    /// The [`DayDataParsed`] containing the key-value pair to evaluate.
    pub day: &'a DayDataParsed,

    /// The [`EntryKey`] of the key-value pair to evaluate.
    pub key: &'a EntryKey,

    ///The [`FilterType`] of the filter that implements the expression
    pub filter_type: &'a FilterType
}

impl FilterExpression
{
    /// Evaluates the given data based on the expression logic and returns wether or not the
    /// data is valid to show under the filter rules.
    ///
    /// # Arguments
    ///
    /// * `data` - The [`EvalData`] to evaluate.
    ///
    /// # Returns
    /// * filtered out (i.e. value invalid): false
    ///
    /// * not filtered out (i.e. value valid): true
    pub fn evaluate(&self, data: &EvalData) -> bool
    {
        match self
        {
            FilterExpression::SingleCommand(cmd) => match data.filter_type
            {
                FilterType::Date => cmd.apply_date_filter(data),
                FilterType::Key => cmd.apply_key_filter(data),
                FilterType::Value => cmd.apply_value_filter(data),
            },
            FilterExpression::Not(inner) => !inner.evaluate(data),
            FilterExpression::And(a, b) => a.evaluate(data) && b.evaluate(data),
            FilterExpression::Or(a, b) => a.evaluate(data) || b.evaluate(data),
            FilterExpression::Xor(a, b) => a.evaluate(data) ^ b.evaluate(data),
            FilterExpression::Nor(a, b) => !(a.evaluate(data) || b.evaluate(data)),
            FilterExpression::Nand(a, b) => !(a.evaluate(data) && b.evaluate(data)),
            FilterExpression::Xnor(a, b) => !(a.evaluate(data) ^ b.evaluate(data)),
        }
    }
}