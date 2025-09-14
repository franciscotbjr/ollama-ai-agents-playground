use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RoleError;

impl FromStr for Role {
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_lowercase().as_str() {
            SYSTEM => Ok(Role::System),
            USER => Ok(Role::User),
            ASSISTANT => Ok(Role::Assistant),
            _ => Err(RoleError)
        }
    }

    type Err = RoleError;
}


impl Role {
    pub fn to_str(&self) -> &str {
        match self {
            Self::System => SYSTEM,
            Self::User => USER,
            Self::Assistant => ASSISTANT,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::System => write!(f, "{SYSTEM}"),
            Role::User => write!(f, "{USER}"),
            Role::Assistant => write!(f, "{ASSISTANT}"),
        }
    }
}

const SYSTEM: &str = "system";
const USER: &str = "user";
const ASSISTANT: &str = "assistant";

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_role_from_str_valid_cases() {
        assert_eq!("system".parse::<Role>().unwrap(), Role::System);
        assert_eq!("user".parse::<Role>().unwrap(), Role::User);
        assert_eq!("assistant".parse::<Role>().unwrap(), Role::Assistant);
    }

    #[test]
    fn test_role_from_str_case_insensitive() {
        assert_eq!("SYSTEM".parse::<Role>().unwrap(), Role::System);
        assert_eq!("USER".parse::<Role>().unwrap(), Role::User);
        assert_eq!("ASSISTANT".parse::<Role>().unwrap(), Role::Assistant);
        assert_eq!("System".parse::<Role>().unwrap(), Role::System);
        assert_eq!("User".parse::<Role>().unwrap(), Role::User);
        assert_eq!("Assistant".parse::<Role>().unwrap(), Role::Assistant);
    }

    #[test]
    fn test_role_from_str_with_whitespace() {
        assert_eq!("  system  ".parse::<Role>().unwrap(), Role::System);
        assert_eq!("\tuser\t".parse::<Role>().unwrap(), Role::User);
        assert_eq!("\nassistant\n".parse::<Role>().unwrap(), Role::Assistant);
    }

    #[test]
    fn test_role_from_str_invalid_cases() {
        assert!("invalid".parse::<Role>().is_err());
        assert!("admin".parse::<Role>().is_err());
        assert!("moderator".parse::<Role>().is_err());
        assert!("".parse::<Role>().is_err());
        assert!("  ".parse::<Role>().is_err());
    }

    #[test]
    fn test_role_to_str() {
        assert_eq!(Role::System.to_str(), "system");
        assert_eq!(Role::User.to_str(), "user");
        assert_eq!(Role::Assistant.to_str(), "assistant");
    }

    #[test]
    fn test_role_display() {
        assert_eq!(format!("{}", Role::System), "system");
        assert_eq!(format!("{}", Role::User), "user");
        assert_eq!(format!("{}", Role::Assistant), "assistant");
    }

    #[test]
    fn test_role_debug() {
        let debug_system = format!("{:?}", Role::System);
        let debug_user = format!("{:?}", Role::User);
        let debug_assistant = format!("{:?}", Role::Assistant);

        assert!(debug_system.contains("System"));
        assert!(debug_user.contains("User"));
        assert!(debug_assistant.contains("Assistant"));
    }

    #[test]
    fn test_role_clone() {
        let original_system = Role::System;
        let cloned_system = original_system.clone();
        assert_eq!(original_system, cloned_system);

        let original_user = Role::User;
        let cloned_user = original_user.clone();
        assert_eq!(original_user, cloned_user);

        let original_assistant = Role::Assistant;
        let cloned_assistant = original_assistant.clone();
        assert_eq!(original_assistant, cloned_assistant);
    }

    #[test]
    fn test_role_partial_eq() {
        assert_eq!(Role::System, Role::System);
        assert_eq!(Role::User, Role::User);
        assert_eq!(Role::Assistant, Role::Assistant);

        assert_ne!(Role::System, Role::User);
        assert_ne!(Role::System, Role::Assistant);
        assert_ne!(Role::User, Role::Assistant);
    }

    #[test]
    fn test_role_serialization() {
        let system_json = serde_json::to_string(&Role::System).unwrap();
        let user_json = serde_json::to_string(&Role::User).unwrap();
        let assistant_json = serde_json::to_string(&Role::Assistant).unwrap();

        assert_eq!(system_json, r#""system""#);
        assert_eq!(user_json, r#""user""#);
        assert_eq!(assistant_json, r#""assistant""#);
    }

    #[test]
    fn test_role_deserialization() {
        let system: Role = serde_json::from_str(r#""system""#).unwrap();
        let user: Role = serde_json::from_str(r#""user""#).unwrap();
        let assistant: Role = serde_json::from_str(r#""assistant""#).unwrap();

        assert_eq!(system, Role::System);
        assert_eq!(user, Role::User);
        assert_eq!(assistant, Role::Assistant);
    }

    #[test]
    fn test_role_deserialization_invalid() {
        let invalid_role_result: Result<Role, _> = serde_json::from_str(r#""invalid""#);
        assert!(invalid_role_result.is_err());

        let empty_role_result: Result<Role, _> = serde_json::from_str(r#""""#);
        assert!(empty_role_result.is_err());
    }

    #[test]
    fn test_role_roundtrip_serialization() {
        let roles = vec![Role::System, Role::User, Role::Assistant];

        for original_role in roles {
            let json = serde_json::to_string(&original_role).unwrap();
            let deserialized_role: Role = serde_json::from_str(&json).unwrap();
            assert_eq!(original_role, deserialized_role);
        }
    }

    #[test]
    fn test_role_error_debug() {
        let error = RoleError;
        let debug_string = format!("{:?}", error);
        assert!(debug_string.contains("RoleError"));
    }

    #[test]
    fn test_role_error_partial_eq() {
        let error1 = RoleError;
        let error2 = RoleError;
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_role_constants() {
        assert_eq!(SYSTEM, "system");
        assert_eq!(USER, "user");
        assert_eq!(ASSISTANT, "assistant");
    }

    #[test]
    fn test_role_from_str_consistency_with_to_str() {
        let roles = vec![Role::System, Role::User, Role::Assistant];

        for role in roles {
            let str_representation = role.to_str();
            let parsed_role = str_representation.parse::<Role>().unwrap();
            assert_eq!(role, parsed_role);
        }
    }

    #[test]
    fn test_role_from_str_consistency_with_display() {
        let roles = vec![Role::System, Role::User, Role::Assistant];

        for role in roles {
            let display_representation = format!("{}", role);
            let parsed_role = display_representation.parse::<Role>().unwrap();
            assert_eq!(role, parsed_role);
        }
    }

    #[test]
    fn test_role_unicode_invalid() {
        assert!("système".parse::<Role>().is_err());
        assert!("用户".parse::<Role>().is_err());
        assert!("助手".parse::<Role>().is_err());
    }

    #[test]
    fn test_role_special_characters_invalid() {
        assert!("system!".parse::<Role>().is_err());
        assert!("user@".parse::<Role>().is_err());
        assert!("assistant#".parse::<Role>().is_err());
        assert!("user-role".parse::<Role>().is_err());
        assert!("system_admin".parse::<Role>().is_err());
    }

    #[test]
    fn test_role_numeric_invalid() {
        assert!("1".parse::<Role>().is_err());
        assert!("123".parse::<Role>().is_err());
        assert!("user1".parse::<Role>().is_err());
        assert!("0system".parse::<Role>().is_err());
    }

    #[test]
    fn test_role_mixed_case_edge_cases() {
        assert_eq!("sYsTeM".parse::<Role>().unwrap(), Role::System);
        assert_eq!("UsEr".parse::<Role>().unwrap(), Role::User);
        assert_eq!("aSSISTANT".parse::<Role>().unwrap(), Role::Assistant);
    }
}