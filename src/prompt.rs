use std::io::{self, Write};

use crate::output;
use crate::socket;

pub struct Prompt {
    bird_socket: socket::BirdSocket,
}

impl Prompt {
    pub fn new(bird_socket: socket::BirdSocket) -> Prompt {
        let prompt = Prompt {
            bird_socket: bird_socket,
        };
        return prompt;
    }

    pub fn start(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let command = self.listen_for_input();
            self.bird_socket.send_command(&command);
            let raw_command_output = self.bird_socket.read_output();
            match output::parse(raw_command_output) {
                Ok(parsed_output) => println!("{}", parsed_output),
                Err(_) => println!("An error occured"),
            }
        }
    }

    fn listen_for_input(&self) -> String {

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read from stdin");
        return user_input;
    }
}
