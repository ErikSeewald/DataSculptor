use std::collections::HashMap;
use chrono::NaiveDate;

pub struct DataDayUnparsed
{
    date: String,
    entries: HashMap<String, String>
}

impl From<(String, HashMap<String, String>)> for DataDayUnparsed
{
    fn from(value: (String, HashMap<String, String>)) -> Self
    {
        DataDayUnparsed
        {
            date: value.0,
            entries: value.1
        }
    }
}

struct DataDayParsed
{
    date: NaiveDate,
    entries: HashMap<EntryKey, EntryValue>
}

struct EntryKey
{
    title: String
}

struct EntryValue
{
    answer: String
}