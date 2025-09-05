use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub database: DatabaseConfig,
    pub ollama: OllamaConfig,
    pub assistant: AssistantConfig,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OllamaConfig {
    pub api: ApiConfig,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ApiConfig {
    pub url: String,
    pub chat: String,
    pub create: String,
    pub model: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AssistantConfig {
    pub root: AssistantRootConfig,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AssistantRootConfig {
    pub name: String,
}

impl AssistantRootConfig {
    pub fn to_name(&self, named_to: &str) -> String {
        format!("{}-{}", self.name, named_to)
    }
}

impl ApiConfig {
    /// Returns the full URL for the chat endpoint
    pub fn chat_url(&self) -> String {
        format!("{}{}", self.url, self.chat)
    }

    /// Returns the full URL for the create endpoint
    pub fn create_url(&self) -> String {
        format!("{}{}", self.url, self.create)
    }

    /// Returns the base URL for the API
    pub fn base_url(&self) -> &str {
        &self.url
    }

    /// Returns a specific endpoint URL by combining base URL with given path
    pub fn endpoint_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.url, endpoint)
    }
}

static CONFIG: Lazy<Config> =
    Lazy::new(|| Config::load_from_file("config.toml").expect("Failed to load config.toml"));

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn get() -> &'static Config {
        &CONFIG
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    fn create_test_config_file(path: &str, content: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn cleanup_test_file(path: &str) {
        if Path::new(path).exists() {
            let _ = fs::remove_file(path);
        }
    }

    #[test]
    fn test_config_load_from_file_valid_config() {
        let test_path = "test_config_valid.toml";
        let test_content = r#"
[database]
path = "/test/database.db"

[ollama.api]
url = "http://localhost:8080/api"
chat = "/chat"
create = "/create"
model = "test-model"

[assistant.root]
name = "test-assistant-"
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.database.path, "/test/database.db");
        assert_eq!(config.ollama.api.url, "http://localhost:8080/api");
        assert_eq!(config.ollama.api.chat, "/chat");
        assert_eq!(config.ollama.api.create, "/create");
        assert_eq!(config.ollama.api.model, "test-model");
        assert_eq!(config.assistant.root.name, "test-assistant-");

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_load_from_file_nonexistent_file() {
        let result = Config::load_from_file("nonexistent_config.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_load_from_file_invalid_toml() {
        let test_path = "test_config_invalid.toml";
        let invalid_content = r#"
[database
path = "/test/database.db"
invalid toml format
"#;

        create_test_config_file(test_path, invalid_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_err());

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_load_from_file_missing_required_fields() {
        let test_path = "test_config_missing_fields.toml";
        let incomplete_content = r#"
[database]
path = "/test/database.db"
"#;

        create_test_config_file(test_path, incomplete_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_err());

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let original_config = Config {
            database: DatabaseConfig {
                path: "/test/path.db".to_string(),
            },
            ollama: OllamaConfig {
                api: ApiConfig {
                    url: "http://test.com/api".to_string(),
                    chat: "/chat".to_string(),
                    create: "/create".to_string(),
                    model: "test-model".to_string(),
                },
            },
            assistant: AssistantConfig {
                root: AssistantRootConfig {
                    name: "test-prefix-".to_string(),
                },
            },
        };

        let serialized = toml::to_string(&original_config).expect("Serialization should succeed");
        let deserialized: Config =
            toml::from_str(&serialized).expect("Deserialization should succeed");

        assert_eq!(original_config, deserialized);
    }

    #[test]
    fn test_config_with_empty_strings() {
        let test_path = "test_config_empty_strings.toml";
        let test_content = r#"
[database]
path = ""

[ollama.api]
url = ""
chat = ""
create = ""
model = ""

[assistant.root]
name = ""
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.database.path, "");
        assert_eq!(config.ollama.api.url, "");
        assert_eq!(config.ollama.api.chat, "");
        assert_eq!(config.ollama.api.create, "");
        assert_eq!(config.ollama.api.model, "");
        assert_eq!(config.assistant.root.name, "");

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_with_special_characters() {
        let test_path = "test_config_special_chars.toml";
        let test_content = r#"
[database]
path = "C:\\Users\\Test User\\database with spaces.db"

[ollama.api]
url = "http://localhost:8080/api"
chat = "/chat?param=value&other=123"
create = "/create-endpoint"
model = "model-with-dashes_and_underscores"

[assistant.root]
name = "assistant-prefix_with-special-chars"
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(
            config.database.path,
            "C:\\Users\\Test User\\database with spaces.db"
        );
        assert_eq!(config.ollama.api.url, "http://localhost:8080/api");
        assert_eq!(config.ollama.api.chat, "/chat?param=value&other=123");
        assert_eq!(config.ollama.api.create, "/create-endpoint");
        assert_eq!(config.ollama.api.model, "model-with-dashes_and_underscores");
        assert_eq!(
            config.assistant.root.name,
            "assistant-prefix_with-special-chars"
        );

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_with_unicode_content() {
        let test_path = "test_config_unicode.toml";
        let test_content = r#"
[database]
path = "/测试/数据库.db"

[ollama.api]
url = "http://localhost:8080/api"
chat = "/café-chat"
create = "/创建-endpoint"
model = "模型-test-ñoño"

[assistant.root]
name = "助理-prefix-café"
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.database.path, "/测试/数据库.db");
        assert_eq!(config.ollama.api.url, "http://localhost:8080/api");
        assert_eq!(config.ollama.api.chat, "/café-chat");
        assert_eq!(config.ollama.api.create, "/创建-endpoint");
        assert_eq!(config.ollama.api.model, "模型-test-ñoño");
        assert_eq!(config.assistant.root.name, "助理-prefix-café");

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_get_method_consistency() {
        let config1 = Config::get();
        let config2 = Config::get();

        // Both calls should return the same reference (same memory address)
        assert!(std::ptr::eq(config1, config2));

        // Content should be the same
        assert_eq!(config1.database.path, config2.database.path);
        assert_eq!(config1.ollama.api.url, config2.ollama.api.url);
        assert_eq!(config1.ollama.api.chat, config2.ollama.api.chat);
        assert_eq!(config1.ollama.api.create, config2.ollama.api.create);
        assert_eq!(config1.ollama.api.model, config2.ollama.api.model);
        assert_eq!(config1.assistant.root.name, config2.assistant.root.name);
    }

    #[test]
    fn test_database_config_creation() {
        let db_config = DatabaseConfig {
            path: "/custom/path.db".to_string(),
        };

        assert_eq!(db_config.path, "/custom/path.db");
    }

    #[test]
    fn test_api_config_creation() {
        let api_config = ApiConfig {
            url: "http://test.com/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            model: "test-model".to_string(),
        };

        assert_eq!(api_config.url, "http://test.com/api");
        assert_eq!(api_config.chat, "/chat");
        assert_eq!(api_config.create, "/create");
        assert_eq!(api_config.model, "test-model");
    }

    #[test]
    fn test_ollama_config_creation() {
        let ollama_config = OllamaConfig {
            api: ApiConfig {
                url: "http://test.com/api".to_string(),
                chat: "/chat".to_string(),
                create: "/create".to_string(),
                model: "test-model".to_string(),
            },
        };

        assert_eq!(ollama_config.api.url, "http://test.com/api");
        assert_eq!(ollama_config.api.chat, "/chat");
        assert_eq!(ollama_config.api.create, "/create");
        assert_eq!(ollama_config.api.model, "test-model");
    }

    #[test]
    fn test_full_config_creation() {
        let config = Config {
            database: DatabaseConfig {
                path: "/test/db.db".to_string(),
            },
            ollama: OllamaConfig {
                api: ApiConfig {
                    url: "http://test.com/api".to_string(),
                    chat: "/chat".to_string(),
                    create: "/create".to_string(),
                    model: "test-model".to_string(),
                },
            },
            assistant: AssistantConfig {
                root: AssistantRootConfig {
                    name: "test-assistant-".to_string(),
                },
            },
        };

        assert_eq!(config.database.path, "/test/db.db");
        assert_eq!(config.ollama.api.url, "http://test.com/api");
        assert_eq!(config.ollama.api.chat, "/chat");
        assert_eq!(config.ollama.api.create, "/create");
        assert_eq!(config.ollama.api.model, "test-model");
        assert_eq!(config.assistant.root.name, "test-assistant-");
    }

    #[test]
    fn test_config_debug_format() {
        let config = Config {
            database: DatabaseConfig {
                path: "/test/db.db".to_string(),
            },
            ollama: OllamaConfig {
                api: ApiConfig {
                    url: "http://test.com/api".to_string(),
                    chat: "/chat".to_string(),
                    create: "/create".to_string(),
                    model: "test-model".to_string(),
                },
            },
            assistant: AssistantConfig {
                root: AssistantRootConfig {
                    name: "debug-assistant-".to_string(),
                },
            },
        };

        let debug_string = format!("{:?}", config);
        assert!(debug_string.contains("Config"));
        assert!(debug_string.contains("/test/db.db"));
        assert!(debug_string.contains("http://test.com/api"));
        assert!(debug_string.contains("/chat"));
        assert!(debug_string.contains("/create"));
        assert!(debug_string.contains("test-model"));
        assert!(debug_string.contains("debug-assistant-"));
    }

    #[test]
    fn test_assistant_config_creation() {
        let assistant_config = AssistantConfig {
            root: AssistantRootConfig {
                name: "custom-assistant-".to_string(),
            },
        };

        assert_eq!(assistant_config.root.name, "custom-assistant-");
    }

    #[test]
    fn test_assistant_root_config_creation() {
        let root_config = AssistantRootConfig {
            name: "prefix-".to_string(),
        };

        assert_eq!(root_config.name, "prefix-");
    }

    #[test]
    fn test_assistant_config_with_empty_name() {
        let assistant_config = AssistantConfig {
            root: AssistantRootConfig {
                name: "".to_string(),
            },
        };

        assert_eq!(assistant_config.root.name, "");
    }

    #[test]
    fn test_assistant_config_with_special_characters() {
        let assistant_config = AssistantConfig {
            root: AssistantRootConfig {
                name: "special-chars_123-".to_string(),
            },
        };

        assert_eq!(assistant_config.root.name, "special-chars_123-");
    }

    #[test]
    fn test_assistant_config_with_unicode() {
        let assistant_config = AssistantConfig {
            root: AssistantRootConfig {
                name: "助理-prefix-café-".to_string(),
            },
        };

        assert_eq!(assistant_config.root.name, "助理-prefix-café-");
    }

    #[test]
    fn test_assistant_config_serialization() {
        let assistant_config = AssistantConfig {
            root: AssistantRootConfig {
                name: "serialization-test-".to_string(),
            },
        };

        let serialized = toml::to_string(&assistant_config).expect("Serialization should succeed");
        let deserialized: AssistantConfig =
            toml::from_str(&serialized).expect("Deserialization should succeed");

        assert_eq!(assistant_config, deserialized);
        assert_eq!(deserialized.root.name, "serialization-test-");
    }

    #[test]
    fn test_config_load_from_file_missing_assistant_section() {
        let test_path = "test_config_no_assistant.toml";
        let test_content = r#"
[database]
path = "/test/database.db"

[ollama.api]
url = "http://localhost:8080/api/chat"
model = "test-model"
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(
            result.is_err(),
            "Config should fail to load without assistant section"
        );

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_load_from_file_missing_assistant_root_name() {
        let test_path = "test_config_no_assistant_name.toml";
        let test_content = r#"
[database]
path = "/test/database.db"

[ollama.api]
url = "http://localhost:8080/api/chat"
model = "test-model"

[assistant.root]
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(
            result.is_err(),
            "Config should fail to load without assistant.root.name"
        );

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_get_includes_assistant() {
        let config = Config::get();

        // Verify that assistant configuration is accessible
        assert!(
            !config.assistant.root.name.is_empty(),
            "Assistant root name should not be empty from config file"
        );
    }

    // New tests for endpoint configuration
    #[test]
    fn test_api_config_chat_url() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            model: "gemma3".to_string(),
        };

        assert_eq!(api_config.chat_url(), "http://localhost:11434/api/chat");
    }

    #[test]
    fn test_api_config_create_url() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            model: "gemma3".to_string(),
        };

        assert_eq!(api_config.create_url(), "http://localhost:11434/api/create");
    }

    #[test]
    fn test_api_config_base_url() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            model: "gemma3".to_string(),
        };

        assert_eq!(api_config.base_url(), "http://localhost:11434/api");
    }

    #[test]
    fn test_api_config_endpoint_url() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            model: "gemma3".to_string(),
        };

        assert_eq!(
            api_config.endpoint_url("/models"),
            "http://localhost:11434/api/models"
        );
        assert_eq!(
            api_config.endpoint_url("/generate"),
            "http://localhost:11434/api/generate"
        );
    }

    #[test]
    fn test_api_config_urls_with_trailing_slash() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api/".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            model: "gemma3".to_string(),
        };

        assert_eq!(api_config.chat_url(), "http://localhost:11434/api//chat");
        assert_eq!(
            api_config.create_url(),
            "http://localhost:11434/api//create"
        );
    }

    #[test]
    fn test_api_config_urls_with_query_parameters() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat?stream=false".to_string(),
            create: "/create?format=json".to_string(),
            model: "gemma3".to_string(),
        };

        assert_eq!(
            api_config.chat_url(),
            "http://localhost:11434/api/chat?stream=false"
        );
        assert_eq!(
            api_config.create_url(),
            "http://localhost:11434/api/create?format=json"
        );
    }

    #[test]
    fn test_api_config_urls_with_unicode() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/聊天".to_string(),
            create: "/创建".to_string(),
            model: "测试模型".to_string(),
        };

        assert_eq!(api_config.chat_url(), "http://localhost:11434/api/聊天");
        assert_eq!(api_config.create_url(), "http://localhost:11434/api/创建");
    }

    #[test]
    fn test_api_config_endpoints_empty_paths() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "".to_string(),
            create: "".to_string(),
            model: "gemma3".to_string(),
        };

        assert_eq!(api_config.chat_url(), "http://localhost:11434/api");
        assert_eq!(api_config.create_url(), "http://localhost:11434/api");
    }

    #[test]
    fn test_config_load_from_file_missing_ollama_endpoints() {
        let test_path = "test_config_no_endpoints.toml";
        let test_content = r#"
[database]
path = "/test/database.db"

[ollama.api]
url = "http://localhost:8080/api"
model = "test-model"

[assistant.root]
name = "test-assistant-"
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(
            result.is_err(),
            "Config should fail to load without chat and create endpoints"
        );

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_api_config_from_actual_config() {
        let config = Config::get();

        // Verify the endpoints are accessible and properly formatted
        let chat_url = config.ollama.api.chat_url();
        let create_url = config.ollama.api.create_url();

        assert!(chat_url.starts_with("http"));
        assert!(create_url.starts_with("http"));
        assert!(chat_url.contains("/chat") || chat_url.ends_with("/api"));
        assert!(create_url.contains("/create") || create_url.ends_with("/api"));
    }

    #[test]
    fn test_assistant_root_config_to_name_method() {
        let root_config = AssistantRootConfig {
            name: "assistant".to_string(),
        };

        assert_eq!(root_config.to_name("personal"), "assistant-personal");
        assert_eq!(
            root_config.to_name("professional"),
            "assistant-professional"
        );
        assert_eq!(root_config.to_name(""), "assistant-");
    }
}
