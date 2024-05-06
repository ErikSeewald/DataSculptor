
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FilterType
{
    Date, Key, Value
}

pub struct Filter
{
    pub title: String
}