use serde::{Deserialize, Serialize};

/// The simplified interface of what the robot can do
///
/// TODO: needs to be more verbose and/or allow more options and chaining of commands
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Command {
    Start,
    Stop,
    Dock,
}
