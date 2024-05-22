use std::hash::{DefaultHasher, Hash, Hasher};
use indexmap::IndexMap;
use crate::core::data_containers::{DayDataParsed, EntryKey};
use crate::core::filters::filter_expression::{EvalData, FilterExpression};

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
    pub numeric_id: u64
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
pub fn filter_day(day: &DayDataParsed, date_filters: &IndexMap<FilterID, Filter>,
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