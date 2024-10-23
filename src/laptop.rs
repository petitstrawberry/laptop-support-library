use super::*;

pub struct Laptop {
    pub keyboard: keyboard::Keyboard,
    pub mouse: mouse::Mouse,
    pub tabletmode: tabletmode::TabletMode,
}

impl Laptop {
    pub fn new() -> Laptop {
        Laptop {
            keyboard: keyboard::Keyboard::new(),
            mouse: mouse::Mouse::new(),
            tabletmode: tabletmode::TabletMode::new(),
        }
    }
}
