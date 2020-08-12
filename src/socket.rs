use std::os::unix::net::UnixStream;
use std::io::{Write, Read, BufRead};

pub struct CmdResult {
    command: String,
    success: bool,
    response: String,
}

pub struct BirdSocket {
    // path: &str requires a lifetime, so make it a String first
    path: String,
    socket: UnixStream,
}

impl BirdSocket {

    pub fn send_command(&mut self, command: &str) {
        let mut prepared_command = String::new();
        prepared_command.push_str(command);
        prepared_command.push('\n');
        self.socket.write_all(prepared_command.as_bytes());
    }

    pub fn read_output(&mut self) -> String {
        // Use BufRead read_lines()
        let mut command_output = String::new();
//        let line_buffer = BufRead::new(self.socket.read);
//        match self.socket.read_to_string(&mut command_output) {
//            Ok(no_bytes) => {
//                println!("Read {} bytes from Bird socket", no_bytes);
//            },
//            Err(_) => {
//                println!("Could not read data from bird socket")
//            }
//        }
        command_output
    }
}

pub fn connect(path: String) -> BirdSocket {
    if path.trim() == "" {
        panic!("Empty path was given");
    }

    let unix_sock = match UnixStream::connect(&path) {
        Ok(socket) => socket,
        Err(error) => panic!("Couldn't connect to socket at {}: {}", path, error),
    };

    let bird_socket = BirdSocket {
        path: path,
        socket: unix_sock,
    };
    bird_socket
}

pub fn submit_command(command: String) -> CmdResult {
    let result = CmdResult {
        command: command,
        success: false,
        response: String::from("Mocked response"),
    };
    result
}
