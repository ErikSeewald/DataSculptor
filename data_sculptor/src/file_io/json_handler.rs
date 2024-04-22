//! Handles the loading of json files into a list of [`DayDataUnparsed`]

use std::collections::HashMap;
use std::fs;
use std::io::Error;
use crate::core::data_containers::DayDataUnparsed;

pub fn load_data_str(data_str: &str) -> Result<Vec<DayDataUnparsed>, serde_json::Error>
{
    let data_map: HashMap<String, HashMap<String, String>> = serde_json::from_str(&data_str)?;

    let mut days: Vec<DayDataUnparsed> = Vec::new();
    for (date, entries) in data_map
    {
        days.push(DayDataUnparsed {date, entries });
    }
    Ok(days)
}

pub fn load_data_file(file_path: &str) -> Result<Vec<DayDataUnparsed>, Error>
{
    let data_str: String = fs::read_to_string(file_path)?;
    match load_data_str(data_str.as_str())
    {
        Ok(data) => {Ok(data)}
        Err(e) => {Err(Error::from(e))}
    }
}