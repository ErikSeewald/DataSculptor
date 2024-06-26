//! Module for the [`DataManager`] that sits at the core of data_sculptor.

use crate::core::data_containers::{DayDataParsed, DayDataUnparsed, parse_and_sort_by_date};
use crate::file_io::data_loader;

/// Struct holding and managing all parsed data for the runtime of the program.
pub struct DataManager
{
    pub data: Vec<DayDataParsed>
}

impl DataManager
{
    /// Clears all currently loaded data and attempts to load the data at the given file path.
    /// Returns a tuple of the structure:
    /// ([bool]: success of the operation, [String]: Potential failure message)
    pub fn load_data(&mut self, file_path: &str) -> (bool, String)
    {
        self.data.clear();

        let data_unparsed: Vec<DayDataUnparsed>;
        match data_loader::load_data_file(file_path)
        {
            Ok(data) => {data_unparsed = data;}
            Err(e) => {return (false, e.to_string());}
        }

        let mut data_parsed: Vec<DayDataParsed>;
        match parse_and_sort_by_date(data_unparsed)
        {
            Ok(parsed) => {data_parsed = parsed;}
            Err(e) => {return (false, e.to_string());}
        }

        self.data.append(&mut data_parsed);

        return (true, String::from("Loaded successfully"));
    }
}