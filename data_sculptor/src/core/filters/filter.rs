use std::hash::{DefaultHasher, Hash, Hasher};
use crate::core::filters::filter_commands::{FilterCommand};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum FilterType
{
    Date, Key, Value
}

pub struct Filter
{
    pub title: String,
    pub command: FilterCommand
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FilterID
{
    numeric_id: u64
}

impl From<&FilterCommand> for FilterID
{
    fn from(command: &FilterCommand) -> Self
    {
        let mut hasher = DefaultHasher::new();
        command.hash(&mut hasher);
        Self{numeric_id: hasher.finish()}
    }
}