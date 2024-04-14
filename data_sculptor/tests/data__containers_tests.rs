mod parse_tests
{
    use std::collections::HashMap;
    use std::hash::Hash;
    use chrono::NaiveDate;
    use data_sculptor::core::data_containers::{DATE_FORMAT, DayDataParsed, DayDataUnparsed, EntryKey, EntryValue, parse, ParseError};

    #[test]
    fn test_parse_invalid_date()
    {
        let invalid_dates: [&str; 5] = ["ew", "", "2024-02-31", "2024--02-02", "2024.02.02"];

        for date in invalid_dates
        {
            let parse = parse(
                DayDataUnparsed{date: date.to_string(), entries: HashMap::new() });

            assert_eq!(parse.unwrap_err(), ParseError::InvalidDate(date.to_string()));
        }
    }

    #[test]
    fn test_parse_valid()
    {
        let date: String = String::from("2024-02-02");
        let mut entries: HashMap<String, String> = HashMap::new();
        entries.insert(String::from("key :)"), String::from("value :)"));

        let parse = parse(DayDataUnparsed{date: date.clone(), entries: entries.clone()});

        assert!(parse.is_ok());
        assert_eq!(parse.as_ref().unwrap().date, NaiveDate::parse_from_str(&date, DATE_FORMAT).unwrap());

        let mut valid_entries: HashMap<EntryKey, EntryValue> = HashMap::new();
        valid_entries.insert(
            EntryKey{title: String::from("key :)")},
            EntryValue{string_value: String::from("value :)")}
        );

        assert_eq!(parse.unwrap().entries, valid_entries);

    }
}

mod parse_and_sort_tests
{
    use std::collections::HashMap;
    use data_sculptor::core::data_containers::{DayDataParsed, DayDataUnparsed, parse, parse_and_sort_by_date, ParseError};

    #[test]
    fn test_empty_vec()
    {
        let empty: Vec<DayDataUnparsed> = Vec::new();
        let parsed = parse_and_sort_by_date(empty).unwrap();

        assert_eq!(parsed.len(), 0);
    }

    #[test]
    fn test_valid_vec()
    {
        let mut entries: HashMap<String, String> = HashMap::new();
        entries.insert(String::from("some"), String::from("thing"));
        entries.insert(String::from("some other"), String::from("thing"));

        let vec: Vec<DayDataUnparsed> = vec!
        [
            DayDataUnparsed{date: String::from("2024-01-08"), entries: entries.clone()},
            DayDataUnparsed{date: String::from("2023-01-09"), entries: entries.clone()}
        ];

        let parsed = parse_and_sort_by_date(vec).unwrap();
        assert_eq!(parsed.len(), 2);

        let manual_parse1: Result<DayDataParsed, ParseError> = parse(
            DayDataUnparsed{date: String::from("2023-01-09"), entries: entries.clone()});

        let manual_parse2: Result<DayDataParsed, ParseError> = parse(
            DayDataUnparsed{date: String::from("2024-01-08"), entries: entries.clone()});

        assert_eq!(*(parsed.get(0).unwrap()), manual_parse1.unwrap());
        assert_eq!(*(parsed.get(1).unwrap()), manual_parse2.unwrap());

    }

    #[test]
    fn test_shared_dates_vec()
    {
        let mut entries: HashMap<String, String> = HashMap::new();
        entries.insert(String::from("some"), String::from("thing"));
        entries.insert(String::from("some other"), String::from("thing"));

        let vec: Vec<DayDataUnparsed> = vec!
        [
            DayDataUnparsed{date: String::from("2024-01-09"), entries: entries.clone()},
            DayDataUnparsed{date: String::from("2024-01-09"), entries: entries.clone()}
        ];

        assert!(parse_and_sort_by_date(vec).is_err());
    }

    #[test]
    fn test_cascades_parse_error()
    {
        let vec: Vec<DayDataUnparsed> = vec!
        [
            DayDataUnparsed{date: String::from("WRONG"), entries: HashMap::new()},
        ];

        let error = parse(
            DayDataUnparsed{date: String::from("WRONG"), entries: HashMap::new()})
            .unwrap_err();
        assert_eq!(parse_and_sort_by_date(vec).unwrap_err(), error);
    }
}
