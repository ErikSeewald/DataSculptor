use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use chrono::NaiveDate;
use std::cmp::Eq;

// DAY DATA UNPARSED
pub struct DayDataUnparsed
{
    pub date: String,
    pub entries: HashMap<String, String>
}

// DAY DATA PARSED
pub struct DayDataParsed
{
    date: NaiveDate,
    entries: HashMap<EntryKey, EntryValue>
}

#[derive(Eq, Hash, PartialEq)]
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
            ParseError::InvalidDate(date) => {write!(f, "The date {} is unparseable!", date)}
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse(unparsed: DayDataUnparsed) -> Result<DayDataParsed, ParseError>
{
    // DATE
    let date_format = "%Y-%m-%d";
    let naive_date: NaiveDate;

    match NaiveDate::parse_from_str(&*unparsed.date, date_format) {
        Ok(date) => naive_date = date,
        Err(_) => return Err(ParseError::InvalidDate(unparsed.date))
    }

    // ENTRIES
    let mut entries: HashMap<EntryKey, EntryValue> = HashMap::new();
    for (key, value) in unparsed.entries
    {
        entries.insert(EntryKey {title: key }, EntryValue{string_value: value});
    }

    // CONSTRUCT
    Ok(DayDataParsed
    {
        date: naive_date,
        entries
    })
}

pub fn parse_and_sort_by_date(unparsed_days: Vec<DayDataUnparsed>)
    -> Result<Vec<DayDataParsed>, ParseError>
{
    let mut parsed_days: Vec<DayDataParsed> = Vec::new();
    for day in unparsed_days
    {
        parsed_days.push(parse(day)?);
    }
    parsed_days.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(parsed_days)
}