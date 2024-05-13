use std::hash::{DefaultHasher, Hash, Hasher};
use crate::core::filters::filter_expression::FilterExpression;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum FilterType
{
    Date, Key, Value
}

pub struct Filter
{
    pub title: String,
    pub expression: FilterExpression
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