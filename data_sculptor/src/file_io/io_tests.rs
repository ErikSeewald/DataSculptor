#[cfg(test)]
mod tests
{
    mod json_handler_tests
    {
        use crate::file_io::json_handler::{load_data_json, load_data_str};

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
                \"something-else\": \"2\"
            }
        }";

            let result = load_data_str(json_str);
            assert!(result.is_ok());
        }

        #[test]
        fn test_result_err()
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
    }
}