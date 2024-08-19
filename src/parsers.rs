pub enum Command {
    Storage,
    Retrieval,
    Unknown,
}
pub struct StorageCommand {
    command: String,
    key: String,
    flags: Option<u16>,
    exptime: Option<u32>,
    bytes: Option<u32>,
    no_reply: bool,
    data: String,
}
pub struct RetrievalCommand {
    command: String,
    key: String,
}

impl StorageCommand {
    pub fn deserialize(&self) -> String {
        let flags = match self.flags {
            None => String::from("0"),
            Some(num) => num.to_string(),
        };
        let exptime = match self.exptime {
            None => String::from("0"),
            Some(num) => num.to_string(),
        };
        let noreply = if self.no_reply {
            String::from("noreply")
        } else {
            String::new()
        };
        let bytes = match self.bytes {
            Some(bytes) => bytes.to_string(),
            None => self.data.len().to_string(),
        };

        if self.no_reply {
            String::from(format!(
                "{} {} {} {} {} noreply\r\n{}\r\n",
                self.command, self.key, flags, exptime, bytes, self.data
            ))
        } else {
            String::from(format!(
                "{} {} {} {} {}\r\n{}\r\n",
                self.command, self.key, flags, exptime, bytes, self.data
            ))
        }
    }
    pub fn parse(words: Vec<String>) -> Result<Self, String> {
        if words.len() < 3 {
            return Err(String::from(
                "not enough arguments provided: <command> <key> <flags> <value>",
            ));
        }
        let key: String = words[1].to_string();
        let data: String = words[words.len() - 1].to_string();
        let flags: Option<u16> = None;
        let exptime: Option<u32> = None;
        let bytes: Option<u32> = None;
        let no_reply: bool = false;
        let args = if words.len() > 3 {
            &words[2..words.len() - 2]
        } else {
            &vec![]
        };
        let command: String = match words[0].as_str() {
            "set" | "add" | "replace" => {
                // place to process additional args for set add replace
                for word in args {
                    match word {
                        _ => {
                            return Err(String::from(format!("{} - unexpected argument", word)));
                        }
                    }
                }
                String::from(format!("{}", words[0]))
            }
            "append" | "prepend" => {
                // place to process additional args for append prepend
                for word in args {
                    match word {
                        _ => {
                            return Err(String::from(format!("{} - unexpected argument", word)));
                        }
                    }
                }
                String::from(format!("{}", words[0]))
            }
            _ => {
                return Err(String::from(format!("{} - command not found", words[0])));
            }
        };
        Ok(Self {
            command,
            key,
            flags,
            exptime,
            bytes,
            no_reply,
            data,
        })
    }
}

impl RetrievalCommand {
    pub fn deserialize(&self) -> String {
        String::from(format!("{} {}\r\n", self.command, self.key))
    }
    pub fn parse(words: Vec<String>) -> Result<Self, String> {
        if words.len() < 2 {
            return Err(String::from(
                "not enough arguments provided: <command> <flags> <key>",
            ));
        }
        let command: String = match words[0].as_str() {
            "get" | "gets" | "gat" | "gats" => {
                let args = if words.len() > 3 {
                    &words[2..words.len() - 2]
                } else {
                    &vec![]
                };
                for word in args {
                    match word {
                        _ => return Err(String::from(format!("{} - unexpected argument", word))),
                    }
                }
                words[0].to_string()
            }
            _ => return Err(String::from(format!("{} - command not found", words[0]))),
        };
        let key: String = words[words.len() - 1].to_string();
        Ok(Self { command, key })
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    #[test]
    fn deserialize_standard_storage_command() {
        let standard_storage = StorageCommand {
            command: String::from("set"),
            key: String::from("dog"),
            flags: Some(10),
            exptime: Some(600),
            bytes: Some(5),
            no_reply: false,
            data: String::from("piss"),
        };
        assert_eq!(
            standard_storage.deserialize(),
            String::from("set dog 10 600 5\r\npiss\r\n"),
        );
    }
    #[test]
    fn deserialize_storage_command_nones() {
        let storage = StorageCommand {
            command: String::from("set"),
            key: String::from("dog"),
            flags: None,
            exptime: None,
            bytes: None,
            no_reply: false,
            data: String::from("piss"),
        };
        println!("{}", storage.deserialize());
        assert_eq!(
            storage.deserialize(),
            String::from("set dog 0 0 4\r\npiss\r\n")
        );
    }

    #[test]
    fn deserialize_get_command() {
        let getter = RetrievalCommand {
            command: String::from("get"),
            key: String::from("dog"),
        };
        assert_eq!(getter.deserialize(), String::from("get dog\r\n"));
    }

    #[test]
    fn parse_standard_storage() {
        let command = vec![
            String::from("set"),
            String::from("dog"),
            String::from("piss"),
        ];
        let getter = {
            match StorageCommand::parse(command) {
                Ok(command) => command,
                Err(string) => panic!("getter returned an error {}", string),
            }
        };
        assert_eq!(
            getter.deserialize(),
            String::from("set dog 0 0 4\r\npiss\r\n")
        );
    }
    #[test]
    fn parse_wrong_storage() {
        let command = vec![
            String::from("set"),
            String::from("bro"),
            String::from("-b"),
            String::from("4"),
            String::from("same"),
        ];
        let getter = {
            match StorageCommand::parse(command) {
                Ok(_command) => panic!("wrongly parsed getter got through"),
                Err(string) => string,
            }
        };
        assert_eq!(getter, String::from("-b - unexpected argument"));
    }

    #[test]
    fn parse_retrieval() {
        let command = vec![String::from("get"), String::from("dog")];
        let getter = {
            match RetrievalCommand::parse(command) {
                Ok(command) => command,
                Err(string) => panic!("command should not panic: {}", string),
            }
        };
        assert_eq!(getter.deserialize(), String::from("get dog\r\n"));
    }
}
