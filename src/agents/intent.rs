use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    SendEmail,
    ScheduleMeeting,
    NoAction,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntentError;

impl FromStr for Intent {
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_lowercase().as_str() {
            SEND_EMAIL => Ok(Intent::SendEmail),
            SCHEDULE_MEETING => Ok(Intent::ScheduleMeeting),
            _ => Ok(Intent::NoAction),
        }
    }

    type Err = IntentError;
}

impl Intent {
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
            Intent::SendEmail => write!(f, "{SEND_EMAIL}"),
            Intent::ScheduleMeeting => write!(f, "{SCHEDULE_MEETING}"),
            Intent::NoAction => write!(f, "{NO_ACTION}"),
        }
    }
}

const SEND_EMAIL: &str = "send_email";
const SCHEDULE_MEETING: &str = "schedule_meeting";
const NO_ACTION: &str = "no_action";
