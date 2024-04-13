use std::collections::HashMap;
use std::fs;
use std::io::Error;

use crate::data::data_containers::DataDayUnparsed;

type FlexibleData = HashMap<String, String>;
type DateDataMap = HashMap<String, FlexibleData>;

pub fn load_data_str(data_str: &str) -> Result<Vec<DataDayUnparsed>, serde_json::Error>
{
    let data_map: DateDataMap = serde_json::from_str(&data_str)?;

    let mut data_days: Vec<DataDayUnparsed> = Vec::new();
    for entry in data_map
    {
        data_days.push(DataDayUnparsed::from(entry));
    }
    Ok(data_days)
}

pub fn load_data_json(file_path: &str) -> Result<Vec<DataDayUnparsed>, Error>
{
    let mut data_str: String = fs::read_to_string(file_path)?;

    data_str.replace_range(0..1,"{");
    data_str.replace_range(data_str.len()-1..data_str.len(),"}");

    match load_data_str(data_str.as_str())
    {
        Ok(data) => {Ok(data)}
        Err(e) => {Err(Error::from(e))}
    }
}