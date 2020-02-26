mod communicator;
mod controller;

pub use controller::Controller;

/// List of available LEDs to control.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Led {
    /// LED in main action button 1.
    Action1 = 0,

    /// LED in main action button 2.
    Action2,

    /// LED in main action button 3.
    Action3,

    /// LED in main action button 4.
    Action4,

    /// LED in power button.
    PowerButton,
}

/// A LED controller command.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LedCmd {
    LedReset,
    LedSet(Led, bool),
}

impl LedCmd {
    /// Get protocol command for current controller command.
    fn to_protocol_cmd(self) -> String {
        match self {
            Self::LedReset => "led reset".into(),
            Self::LedSet(led, level) => format!("led set {} {}", led as u8, level as u8),
        }
    }
}
