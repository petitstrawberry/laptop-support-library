use super::endpoint::Endpoint;

enum Command {
    SetPassthrough = 0,
    GetPassthrough = 1,
}

pub struct Keyboard {
    socket_path: String,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            socket_path: "/var/run/keyboardd.sock".to_string(),
        }
    }

    // Is the passthrough mode enabled?
    pub fn is_passthrough_enabled(&self) -> Result<bool, String> {
        match self.send_cmd(Command::GetPassthrough as u8, 0) {
            Ok(1) => Ok(true),
            Ok(0) => Ok(false),
            Ok(_) => Err("Invalid response".to_string()),
            Err(e) => Err(e),
        }
    }

    // Enable or disable passthrough mode
    pub fn enable_passthrough(&self) -> Result<(), String> {
        match self.send_cmd(Command::SetPassthrough as u8, 1) {
            Ok(0) => Ok(()),
            Ok(_) => Err("Invalid response".to_string()),
            Err(e) => Err(e),
        }
    }

    // Disable passthrough mode
    pub fn disable_passthrough(&self) -> Result<(), String> {
        match self.send_cmd(Command::SetPassthrough as u8, 0) {
            Ok(0) => Ok(()),
            Ok(_) => Err("Invalid response".to_string()),
            Err(e) => Err(e),
        }
    }
}

impl Endpoint for Keyboard {
    fn get_socket_path(&self) -> String {
        self.socket_path.clone()
    }
}
