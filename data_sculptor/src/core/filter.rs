
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum FilterType
{
    Date, Key, Value
}

pub struct Filter
{
    pub title: String,
    pub id: FilterID
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FilterID
{
    numeric_id: u64
}

impl From<u64> for FilterID
{
    fn from(numeric_id: u64) -> Self
    {
        Self{numeric_id}
    }
}