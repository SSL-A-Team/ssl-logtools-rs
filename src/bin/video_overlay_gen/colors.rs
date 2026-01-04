use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Colors {
    pub command_blue_team: String,
    pub command_yellow_team: String,
    pub command_start: String,
    pub command_stop: String,
    pub clock_running: String,
    pub clock_stopped: String,
}

impl Default for Colors {
    fn default() -> Colors {
        Colors{
            command_blue_team: "rgb(168, 168, 255)".to_string(),
            command_yellow_team: "rgb(255, 255, 168)".to_string(),
            command_start: "rgb(168, 255, 168)".to_string(),
            command_stop: "rgb(255, 145, 145)".to_string(),
            clock_running: "rgb(255, 255, 255)".to_string(),
            clock_stopped: "rgb(255, 145, 145)".to_string(),
        }
    }
}
