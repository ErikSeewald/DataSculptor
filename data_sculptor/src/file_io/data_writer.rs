//! Handles writing [`DayDataParsed`] to a json file.

use std::fs::File;
use std::io::{Write};
use indexmap::IndexMap;
use crate::core::data_containers::{DayDataParsed};
use crate::core::filters::filter;
use crate::core::filters::filter::{filter_key, FilterType};
use crate::gui::views::list::list_view_control::ListView;

/// Writes the filtered data to a file at the given filepath.
///
/// # Arguments
/// * `file_path` - `String` specifying the path where the new file will be created.
/// * `days` - Vector of `DayDataParsed` structs that hold the data to write.
/// * `list_view` - `ListView` that contains filters to be applied to the data.
pub fn write_data_filtered(file_path: String, days: &Vec<DayDataParsed>, list_view: &ListView)
{
    match File::create(&file_path)
    {
        Ok(file) =>
        {
            _write_filtered_to_file(file, days, list_view);
        },

        Err(e) => panic!("Failed to create file: {}", e)
    }
}

fn _write_filtered_to_file(mut file: File, days: &Vec<DayDataParsed>, list_view: &ListView)
{
    let mut filtered_days: IndexMap<String, IndexMap<String, String>> = IndexMap::new();

    let date_filters = list_view.get_filters(&FilterType::Date);
    let key_filters = list_view.get_filters(&FilterType::Key);
    let value_filters = list_view.get_filters(&FilterType::Value);

    for day in days
    {
        if filter::filter_day(day, date_filters, value_filters)
        {
            let mut filtered_entries: IndexMap<String, String> = IndexMap::new();

            for (key, value) in &day.entries
            {
                if filter_key(day, &key, &key_filters)
                {
                    filtered_entries.insert(key.title.clone(), value.string_value.clone());
                }
            }

            filtered_days.insert(day.date.date_string.clone(), filtered_entries);
        }
    }

    let json_data = serde_json::to_string_pretty(&filtered_days).unwrap();
    file.write_all(json_data.as_bytes()).expect("Failed to save to file");
}