use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    SendEmail,
    ScheduleMeeting,
    NoAction,
}

impl Intent {
    pub fn from_str(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            SEND_EMAIL => Intent::SendEmail,
            SCHEDULE_MEETING => Intent::ScheduleMeeting,
            _ => Intent::NoAction,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::SendEmail => SEND_EMAIL,
            Self::ScheduleMeeting => SCHEDULE_MEETING,
            Self::NoAction => NO_ACTION,
        }
    }
}

impl fmt::Display for Intent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Intent::SendEmail => write!(f, "{}", SEND_EMAIL),
            Intent::ScheduleMeeting => write!(f, "{}", SCHEDULE_MEETING),
            Intent::NoAction => write!(f, "{}", NO_ACTION),
        }
    }
}

const SEND_EMAIL: &str = "send_email";
const SCHEDULE_MEETING: &str = "schedule_meeting";
const NO_ACTION: &str = "no_action";
