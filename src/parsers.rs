pub struct StorageCommand {
    command: String,
    key: String,
    flags: Option<u16>,
    exptime: Option<u32>,
    no_reply: bool,
    data: String,
}

pub struct RetrievalCommand {
    command: String,
    key: String,
}

impl StorageCommand {
    // fn parse(&cmd_line: &String) -> Result<Self, String> {
    //     let args: Vec<String> = cmd_line.split(" ").map(String::from).collect();
    //     let command = match args[0].as_str() {
    //         "set" | "add" | "replace" | "append" | "prepend" => &args[0],
    //         _ => return Err(format!("Command {} cannot be found", args[0])),
    //     };

    //     let key = args[1];
    //     Self
    // }

    fn deserialize(&self) -> String {
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
        String::from(format!(
            "{} {} {} {} {}\r\n{}\r\n",
            self.command, self.key, flags, exptime, noreply, self.data
        ))
    }
}

impl RetrievalCommand {
    fn deserialize(&self) -> String {
        String::from(format!("{} {}\r\n", self.command, self.key))
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
            no_reply: false,
            data: String::from("piss"),
        };
        assert_eq!(
            standard_storage.deserialize(),
            String::from("set dog 10 600 \r\npiss\r\n"),
        );
    }
    #[test]
    fn deserialize_storage_command_nones() {
        let storage = StorageCommand {
            command: String::from("set"),
            key: String::from("dog"),
            flags: None,
            exptime: None,
            no_reply: false,
            data: String::from("piss"),
        };
        assert_eq!(
            storage.deserialize(),
            String::from("set dog 0 0 \r\npiss\r\n")
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
}
