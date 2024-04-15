use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use chrono::NaiveDate;
use std::cmp::Eq;

pub const DATE_FORMAT: &str =  "%Y-%m-%d";

// DAY DATA UNPARSED
pub struct DayDataUnparsed
{
    pub date: String,
    pub entries: HashMap<String, String>
}

// DAY DATA PARSED
#[derive(Debug, PartialEq)]
pub struct DayDataParsed
{
    pub date: DateKey,
    pub entries: HashMap<EntryKey, EntryValue>
}

#[derive(Debug, PartialEq)]
pub struct DateKey
{
    pub naive_date: NaiveDate,
    pub date_string: String
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct EntryKey
{
    pub title: String
}

#[derive(PartialEq, Debug)]
pub struct EntryValue
{
    pub string_value: String
}


// PARSING
#[derive(PartialEq, Debug)]
pub enum ParseError
{
    InvalidDate(String),
    DuplicateDate(String)
}

impl Display for ParseError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            ParseError::InvalidDate(date) => {write!(f, "The date {} is unparseable!", date)}
            ParseError::DuplicateDate(date) => {write!(f, "The date {} is contained multiple times!", date)}
        }
    }
}
impl std::error::Error for ParseError {}

pub fn parse(unparsed: DayDataUnparsed) -> Result<DayDataParsed, ParseError>
{
    // DATE
    let naive_date: NaiveDate;

    match NaiveDate::parse_from_str(&*unparsed.date, DATE_FORMAT) {
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
        date: DateKey{ naive_date: naive_date, date_string: unparsed.date},
        entries
    })
}

pub fn parse_and_sort_by_date(unparsed_days: Vec<DayDataUnparsed>)
    -> Result<Vec<DayDataParsed>, ParseError>
{
    let mut parsed_days: Vec<DayDataParsed> = Vec::new();
    let mut added_dates: HashSet<String> = HashSet::new();
    for day in unparsed_days
    {
        // Disallow duplicate dates
        if !added_dates.insert(day.date.clone())
        {
            return Err(ParseError::DuplicateDate(day.date));
        }
        parsed_days.push(parse(day)?);
    }
    parsed_days.sort_by(|a, b| a.date.naive_date.cmp(&b.date.naive_date));

    Ok(parsed_days)
}