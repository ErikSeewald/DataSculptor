use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use chrono::NaiveDate;

// DAY DATA UNPARSED
pub struct DayDataUnparsed
{
    pub date: String,
    pub entries: HashMap<String, String>
}

// DAY DATA PARSED
struct DayDataParsed
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
    string_value: String
}


// PARSING
#[derive(Debug)]
pub enum ParseError
{
    InvalidDate(String)
}

impl Display for ParseError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            ParseError::InvalidDate(date) => {write!(f, "The date {} is unparsable!", date)}
        }
    }
}

impl std::error::Error for ParseError {}


impl From<&DayDataUnparsed> for DayDataParsed
{
    fn from(value: &DayDataUnparsed) -> Result<Self, ParseError>
    {
        // DATE
        let date_format = "%Y-%m-%d";
        let mut naive_date: NaiveDate;

        match NaiveDate::parse_from_str(&*value.date, date_format) {
            Ok(date) => naive_date = date,
            Err(e) => return Err(ParseError::InvalidDate(value.date.clone()))
        }

        // ENTRIES
        let mut entries: HashMap<EntryKey, EntryValue> = HashMap::new();
        for (key, entry_value) in value.entries
        {
            entries.insert(EntryKey {title: key}, EntryValue{string_value: entry_value});
        }

        // CONSTRUCT
        Ok(Self
        {
            date: naive_date,
            entries
        })
    }
}

pub fn parse_and_sort_by_date(unparsed_days: &Vec<DayDataUnparsed>) -> Vec<DayDataParsed>
{
    let mut parsed_days: Vec<DayDataParsed> = Vec::new();
    for day in unparsed_days
    {
        parsed_days.push(DayDataParsed::from(day));
    }
    parsed_days.sort_by(|a, b| a.date.cmp(&b.date));

    return parsed_days;
}