use std::fmt;

#[derive(Debug)]
pub enum Action {
    SendEmail,
    ScheduleMeeting, 
    NoAction,
}

impl Action {
    pub fn from_str(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            SEND_EMAIL => Action::SendEmail,
            SCHEDULE_MEETING => Action::ScheduleMeeting,
            _ => Action::NoAction,
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

impl fmt::Display for Action {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
          match self {
              Action::SendEmail => write!(f, "{}", SEND_EMAIL),
              Action::ScheduleMeeting => write!(f, "{}", SCHEDULE_MEETING),
              Action::NoAction => write!(f, "{}", NO_ACTION),
          }
      }
}

const SEND_EMAIL: &str = "send_email";
const SCHEDULE_MEETING: &str = "schedule_meeting";
const NO_ACTION: &str = "no_action";
