mod tests
{
    mod data_containers_tests
    {
        mod parse_tests
        {
        }

        mod parse_and_sort_tests
        {
            use crate::data::data_containers::{DayDataParsed, DayDataUnparsed, parse_and_sort_by_date};

            #[test]
            fn test_empty_vec()
            {
                let empty: Vec<DayDataUnparsed> = Vec::new();
                let parsed = parse_and_sort_by_date(empty).unwrap();

                assert_eq!(parsed.len(), 0);
            }
        }
    }
}