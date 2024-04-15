use crate::core::data_containers::{DayDataParsed, DayDataUnparsed, parse_and_sort_by_date};
use crate::file_io::json_handler;

pub struct DataManager
{
    pub data: Vec<DayDataParsed>
}

impl DataManager
{
    pub fn load_data(&mut self, file_path: &str) -> (bool, String)
    {
        let data_unparsed: Vec<DayDataUnparsed>;
        match json_handler::load_data_file(file_path)
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

        self.data.clear();
        self.data.append(&mut data_parsed);

        return (true, String::from("Loaded successfully"));
    }
}