use super::endpoint::Endpoint;

enum Command {
    SetAutoDetect = 0,
    GetAutoDetect = 1,
    SetTabletMode = 2,
    GetTabletMode = 3,
}

pub struct TabletMode {
    socket_path: String,
}

impl TabletMode {
    pub fn new() -> TabletMode {
        TabletMode {
            socket_path: "/var/run/tabletmoded.sock".to_string(),
        }
    }

    // Is the tablet mode enabled?
    pub fn is_enabled(&self) -> Result<bool, String> {
        match self.send_cmd(Command::GetTabletMode as u8, 0) {
            Ok(1) => Ok(true),
            Ok(0) => Ok(false),
            Ok(_) => Err("Invalid response".to_string()),
            Err(e) => Err(e),
        }
    }

    // Enable or disable tablet mode
    pub fn enable(&self) -> Result<(), String> {
        match self.send_cmd(Command::SetTabletMode as u8, 1) {
            Ok(0) => Ok(()),
            Ok(_) => Err("Invalid response".to_string()),
            Err(e) => Err(e),
        }
    }

    // Disable tablet mode
    pub fn disable(&self) -> Result<(), String> {
        match self.send_cmd(Command::SetTabletMode as u8, 0) {
            Ok(0) => Ok(()),
            Ok(_) => Err("Invalid response".to_string()),
            Err(e) => Err(e),
        }
    }

    // Is the auto detection of tablet mode enabled?
    pub fn is_auto_detect_enabled(&self) -> Result<bool, String> {
        match self.send_cmd(Command::GetAutoDetect as u8, 0) {
            Ok(1) => Ok(true),
            Ok(0) => Ok(false),
            Ok(_) => Err("Invalid response".to_string()),
            Err(e) => Err(e),
        }
    }

    // Enable auto detection of tablet mode
    pub fn enable_auto_detect(&self) -> Result<(), String> {
        match self.send_cmd(Command::SetAutoDetect as u8, 1) {
            Ok(0) => Ok(()),
            Ok(_) => Err("Invalid response".to_string()),
            Err(e) => Err(e),
        }
    }

    // Disable auto detection of tablet mode
    pub fn disable_auto_detect(&self) -> Result<(), String> {
        match self.send_cmd(Command::SetAutoDetect as u8, 0) {
            Ok(0) => Ok(()),
            Ok(_) => Err("Invalid response".to_string()),
            Err(e) => Err(e),
        }
    }
}

impl Endpoint for TabletMode {
    fn get_socket_path(&self) -> String {
        self.socket_path.clone()
    }
}
