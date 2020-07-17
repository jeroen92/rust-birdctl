use std::os::unix::net::UnixStream;

pub struct CmdResult {
    command: String,
    success: bool,
    response: String,
}

pub struct BirdSocket {
    path: String,
    socket: UnixStream,
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
