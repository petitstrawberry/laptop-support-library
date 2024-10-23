use std::{io::{Read, Write}, os::unix::net::UnixStream, u8};

pub trait Endpoint {
    fn get_socket_path(&self) -> String;

    fn send_cmd(&self, cmd: u8, value: u8) -> Result<u8, String> {
        // Connect to socket
        let mut stream = match UnixStream::connect(self.get_socket_path()) {
            Ok(stream) => stream,
            Err(e) => return Err(e.to_string()),
        };
        // Send command to socket
        match stream.write_all(&[cmd, value]) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
        // Read response from socket
        let mut response = [0; 1];
        match stream.read(&mut response) {
            Ok(_) => Ok(response[0]),
            Err(e) => return Err(e.to_string()),
        }
    }
}
