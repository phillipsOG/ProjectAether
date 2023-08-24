pub struct Chat {
    pub chat: [String; 10],
    pub input_counter: usize,
    pub is_repeat_message: bool,
    pub repeat_message_counter: i32,
    pub previous_message: String,
}

impl Chat {
    pub(crate) fn new() -> Self {
        Chat {
            chat: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            input_counter: 0,
            is_repeat_message: false,
            repeat_message_counter: 1,
            previous_message: "".parse().unwrap(),
        }
    }

    pub(crate) fn print_chat(&mut self) {
        for i in 0..self.chat.len() {
            println!("{}", self.chat[i]);
        }
    }

    pub(crate) fn clear_chat(&mut self) {
        for i in 0..self.chat.len() {
            self.chat[i] = "".parse().unwrap();
        }
    }

    pub(crate) fn process_chat_message(&mut self, message: &str) {
        if self.input_counter == self.chat.len() {
            self.input_counter = 0;
        }

        // check if message is the same as previous
        if self.previous_message == message {
            self.repeat_message_counter += 1;
            let repeat_suffix = format!("x{}", self.repeat_message_counter);
            let repeated_message = format!("{} {}", message, repeat_suffix);
            let mut tmp = 0;
            if self.input_counter > 0 {
                tmp = 1;
            }
            self.chat[self.input_counter - tmp] = repeated_message;
            self.is_repeat_message = true;
        } else {
            self.chat[self.input_counter] = message.parse().unwrap();

            self.input_counter += 1;
            self.repeat_message_counter = 1;

            self.is_repeat_message = false;
        }

        // store previous message
        self.previous_message = message.parse().unwrap();
    }

    fn print_processed_input(&mut self) {
        if self.is_repeat_message {
            println!(
                "{} x{}",
                self.chat[self.input_counter], self.repeat_message_counter
            );
        } else {
            println!("{}", self.chat[self.input_counter]);
        }
    }
}
