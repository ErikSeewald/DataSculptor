mod data_containers_tests;

mod load_data_str_tests
{
    use indexmap::IndexMap;
    use data_sculptor::file_io::data_loader::load_data_str;

    #[test]
    fn test_result_dates()
    {
        let json_str = "{
            \"2023-10-05\": {
                \"something\": \"1\",
                \"something-else\": \"2\"
            }, \
            \"NO!\": {
                \"something\": \"1\",
                \"something-else\":   \"2\"
            }
        }";

        let days = load_data_str(json_str).unwrap();
        assert_eq!(days.len(), 2);

        let mut contains_both_days: bool = true;

        for day in days
        {
            if day.date != "2023-10-05" && day.date != "NO!"
            {
                contains_both_days = false;
                break;
            }
        }
        assert!(contains_both_days);
    }

    #[test]
    fn test_same_dates_overwrite()
    {
        let json_str = "{
            \"2023-10-05\": {
                \"something\": \"1\",
                \"something-else\": \"2\"
            }, \
            \"2023-10-05\": {
                \"something\": \"3\",
                \"something-else\":   \"4\"
            }
        }";

        let days = load_data_str(json_str).unwrap();
        assert_eq!(days.len(), 1);

        for day in days
        {
            assert_eq!(day.entries.get("something").unwrap().as_str(), "3");
            assert_eq!(day.entries.get("something-else").unwrap().as_str(), "4");
        }
    }

    #[test]
    fn test_large_num_days()
    {
        let amount: usize = 500000;
        let mut json_str: String = String::from("{ ");

        for i in 0..amount-1
        {
            let mut date_str: String = String::from("\"");
            date_str += &*i.to_string();
            date_str += "\": {\"fillerdata\" : \"uuuuuuuuuuuuuuuuuuuuuuuuuuuuu\
            wqddddddwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww\
            wdddddddfwqfqwfqwfqwfqwflqwkdqwkqpwokfpqwofkqopwfkwopqfkpqwkpfwqkfqkwpfkqpwfkqw\
            qwfkopqwjpdsjqwpdjqwpodkqwoslwqüpdkqwüdkqwüpdkqüwpdspwqldpqlwdüpqwldüpqlwdüqlwd\
            kqwdpqkwdpojfpqwmdpqwodmopqwdmqowpdmqwpodmqw,slwq,odjwqopdmqp,sqwdqwjfpoqwpfqpd\
            qjfwdnmkqwmdsqwmdoqwfjnqwoidnqwoidmqwdiomqwdomqwdmqwodmqwodmqwodmqwodmwqodm\"},";
            json_str += date_str.as_str();
        }
        json_str += " \"last-one\": {} }";

        let days = load_data_str(json_str.as_str()).unwrap();
        assert_eq!(days.len(), amount);

    }

    #[test]
    fn test_result_entries()
    {
        let json_str = "{
            \"2023-10-05\": {
                \"something\": \"1\",
                \"something-else\": \"2\"
            }
        }";

        let days = load_data_str(json_str).unwrap();
        for day in days
        {
            let entries: IndexMap<String, String> = day.entries;
            assert_eq!(entries.len(), 2);

            assert_eq!(entries.get("something").unwrap().as_str(), "1");
            assert_eq!(entries.get("something-else").unwrap().as_str(), "2");
        }

    }

    #[test]
    fn test_same_entry_key()
    {
        let json_str = "{
            \"2023-10-05\": {
                \"something\": \"1\",
                \"something\": \"2\"
            }
        }";

        let days = load_data_str(json_str).unwrap();
        for day in days
        {
            assert_eq!(day.entries.get("something").unwrap().as_str(), "2");
        }

    }

    #[test]
    fn test_result_ok()
    {
        let json_str = "{
            \"2023-10-05\": {
                \"something\": \"1\",
                \"something-else\": \"2\"
            }, \
            \"2023-10-06\": {
                \"something\": \"1\",
                \"something-else\":   \"2\"
            }
        }";

        let mut result = load_data_str(json_str);
        assert!(result.is_ok());

        result = load_data_str("{}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_result_ok_empty_day()
    {
        let json_str = "{
            \"2023-10-05\": {}, \
            \"2023-10-06\": {
                \"something\": \"1\",
                \"something-else\": \"2\"
            }
        }";

        let mut result = load_data_str(json_str);
        assert!(result.is_ok());


        result = load_data_str("{}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_result_err_invalid_entries()
    {
        let mut json_str = "{
            \"2023-10-05\": {
                \"something\": {}
            }
        }";

        let mut result = load_data_str(json_str);
        assert!(result.is_err());

        json_str = "{
            \"2023-10-05\": {
                \"something\": []
            }
        }";

        result = load_data_str(json_str);
        assert!(result.is_err());

        json_str = "{
            \"2023-10-05\": {
                \"something\": true
            }
        }";

        result = load_data_str(json_str);
        assert!(result.is_err());

    }

    #[test]
    fn test_result_err_invalid_syntax()
    {
        let json_str = "{
        \"2023-10-05\": {
            \"something\": \"1\",
            \"something-else\": \"2\"
        }, \
        \"2023-10-06\": {
            \"something\": \"1\",
            \"something-else\": \"2\"
        },
    }";

        let result = load_data_str(json_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_result_err_empty()
    {
        let json_str = "";

        let result = load_data_str(json_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_result_err_non_parsable()
    {
        let json_str = "{\"2023-10-05\": \"wrong\"}";

        let result = load_data_str(json_str);
        assert!(result.is_err());
    }
}
