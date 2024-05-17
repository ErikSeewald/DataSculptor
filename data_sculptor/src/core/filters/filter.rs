use std::hash::{DefaultHasher, Hash, Hasher};
use indexmap::IndexMap;
use crate::core::data_containers::{DayDataParsed, EntryKey};

// ---------- FILTER ----------

/// Struct representing a single user defined filter.
/// The filter logic is defined by its [`FilterExpression`] attribute.
/// Must be of a specific [`FilterType`].
pub struct Filter
{
    pub title: String,
    pub expression: FilterExpression,
    pub filter_type: FilterType
}

/// Defines the field a filter acts on.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum FilterType
{
    Date, Key, Value
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FilterID
{
    numeric_id: u64
}

impl From<&FilterExpression> for FilterID
{
    fn from(expression: &FilterExpression) -> Self
    {
        let mut hasher = DefaultHasher::new();
        expression.hash(&mut hasher);
        Self{numeric_id: hasher.finish()}
    }
}

/// Runs the given [`DayDataParsed`] through all given
/// filters and returns whether it is valid to show.
/// This is the case if:
///
/// 1. The date matches the date filter condition
///
/// 2. The values match the value filter condition
///
/// This means we filter out days completely through date and value filters,
/// meanwhile the key filters only filter which key value pairs are shown.
pub(crate) fn filter_day(day: &DayDataParsed, date_filters: &IndexMap<FilterID, Filter>,
    value_filters: &IndexMap<FilterID, Filter>) -> bool
{
    // DATE
    if let Some((key, _)) = day.entries.iter().next()
    {
        let data = EvalData{day, key, filter_type: &FilterType::Date};
        for (_, filter) in date_filters
        {
            if !filter.expression.evaluate(&data)
            {
                return false;
            }
        }
    }

    for (key, _) in &day.entries
    {
        let data = EvalData{day, key, filter_type: &FilterType::Value};

        // VALUE
        for (_, filter) in value_filters
        {
            if !filter.expression.evaluate(&data)
            {
                return false;
            }
        }
    }

    return true;
}

/// Runs the given [`EntryKey`] through all given key
/// filters and returns whether its key is valid under at least one filter condition.
pub fn filter_key(day: &DayDataParsed, key: &EntryKey,
                  key_filters: &IndexMap<FilterID, Filter>) -> bool
{
    if key_filters.is_empty()
    {
        return true;
    }

    let data = EvalData{day, key, filter_type: &FilterType::Key};
    for (_, filter) in key_filters
    {
        if filter.expression.evaluate(&data)
        {
            return true;
        }
    }
    return false;
}

// ---------- FILTER EXPRESSION ----------

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

// ---------- FILTER COMMAND ----------

/// Enum representing all supported ways to filter data in data_sculptor.
#[derive(Hash)]
pub enum FilterCommand
{
    /// Checks if the value contains the given String
    Contains(String),

    /// Checks if the value corresponding to the given key contains the given String
    ///
    /// # Args (index order)
    /// - `key`: String title of the key of the key-value pair to check
    /// - `keyword`: String that the value should contain
    KeyValueContains(String, String)
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
                }
            _ => {true}
        }
    }
}