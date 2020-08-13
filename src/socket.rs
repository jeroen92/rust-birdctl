use std::os::unix::net::UnixStream;
use std::io::{Write, BufRead, BufReader};
use std::time::Duration;

pub struct BirdSocket {
    dirty: bool,
    socket: UnixStream,
}

impl BirdSocket {

    pub fn send_command(&mut self, command: &str) {
        if self.dirty {
            panic!("Bird socket is dirty. Read out previously submitted command first!");
        }
        self.dirty = true;
        let mut prepared_command = String::new();
        prepared_command.push_str(command);
        prepared_command.push('\n');
        self.socket.write_all(prepared_command.as_bytes()).expect("Failed to write to socket");
    }

    pub fn read_output(&mut self) -> String {
        let mut command_output = String::new();
        if !self.dirty {
            return command_output;
        };

        let line_buffer = BufReader::new(&self.socket);
        for line in line_buffer.lines() {
            match line {
                Ok(line_str) => {
                    command_output.push_str(&line_str);
                    command_output.push('\n');
                },
                Err(error) => match error.kind() {
                    std::io::ErrorKind::WouldBlock => break,
                    _ => panic!("Uncaught error: {}", error),
                }
            }
        }
        self.dirty = false;
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

    let socket_timeout = Duration::new(0, 250);
    unix_sock.set_read_timeout(Some(socket_timeout))
        .expect("Couldn't set read timeout");

    let bird_socket = BirdSocket {
        socket: unix_sock,
        dirty: false,
    };
    bird_socket
}
