pub struct StorageResponse {
    message: Option<String>,
}

pub struct RetrievalResponse {
    data: Option<Vec<String>>,
    message: Option<String>,
}

impl StorageResponse {
    pub fn serialize(cmd_response: &str) -> Self {
        match cmd_response {
            "STORED\r\n" => Self {
                message: Some(String::from("storage successful")),
            },
            "NOT_STORED\r\n" => Self {
                message: Some(String::from("data was not stored")),
            },
            "EXISTS\r\n" => Self {
                message: Some(String::from(
                    "item that you attempted to fetch has been modified",
                )),
            },
            "NOT_FOUND\r\n" => Self {
                message: Some(String::from(
                    "item that you are trying to store does not exist",
                )),
            },
            _ => {
                panic!("server did not return a parsable response");
            }
        }
    }
    pub fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}

impl RetrievalResponse {
    pub fn serialize(cmd_response: &str) -> Self {
        match cmd_response {
            "NOT_FOUND\r\n" => Self {
                data: None,
                message: Some(String::from("value not found for provided key")),
            },
            response if response.starts_with("VALUE") => {
                let mut data: Vec<String> = Vec::new();
                let lines: Vec<&str> = response.split("\r\n").collect();
                let count = lines.len();
                for (idx, st) in lines.iter().enumerate() {
                    if idx != 0 && idx != count - 1 {
                        data.push(String::from(*st));
                    }
                }
                Self {
                    data: Some(data),
                    message: Some(String::from("data returned successfully")),
                }
            }
            _ => panic!("server did not return a parsable reponse"),
        }
    }
    pub fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}

#[cfg(test)]
mod retrival_tests {
    use super::*;

    #[test]
    fn parse_storage_responses() {
        let stored_response = StorageResponse::serialize("STORED\r\n");
        let not_stored_response = StorageResponse::serialize("NOT_STORED\r\n");
        let exists_response = StorageResponse::serialize("EXISTS\r\n");
        let not_found_response = StorageResponse::serialize("NOT_FOUND\r\n");
        assert_eq!(
            stored_response.message,
            Some(String::from("storage successful"))
        );
        assert_eq!(
            not_stored_response.message,
            Some(String::from("data was not stored"))
        );
        assert_eq!(
            exists_response.message,
            Some(String::from(
                "item that you attempted to fetch has been modified"
            ))
        );
        assert_eq!(
            not_found_response.message,
            Some(String::from(
                "item that you are trying to store does not exist"
            ))
        );
    }
    #[test]
    #[should_panic]
    fn storage_response_panics() {
        let _panic_response_ = StorageResponse::serialize("panic");
    }

    #[test]
    fn get_response() {
        let get_response =
            RetrievalResponse::serialize("VALUE dog 0 0\r\npissing\r\non\r\nthe\r\nbush\r\n");
        let data_output = vec![
            String::from("pissing"),
            String::from("on"),
            String::from("the"),
            String::from("bush"),
        ];
        assert_eq!(
            Some(String::from("data returned successfully")),
            get_response.message
        );
        assert_eq!(Some(data_output), get_response.data);
    }
}
