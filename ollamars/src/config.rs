use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub ollama: OllamaConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OllamaConfig {
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ApiConfig {
    pub url: String,
    pub chat: String,
    pub create: String,
    pub show: String,
    pub load: String,
    pub model: String,
    pub options: ApiOptions,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ApiOptions {
    pub temperature: f32,
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

    /// Returns the full URL for the show endpoint
    pub fn show_url(&self) -> String {
        format!("{}{}", self.url, self.show)
    }

    /// Returns the full URL for the show endpoint
    pub fn load_url(&self) -> String {
        format!("{}{}", self.url, self.load)
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
[ollama.api]
url = "http://localhost:8080/api"
chat = "/chat"
create = "/create"
show = "/show"
load = "/generate"
model = "test-model"
[ollama.api.options]
temperature = 0
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.ollama.api.url, "http://localhost:8080/api");
        assert_eq!(config.ollama.api.chat, "/chat");
        assert_eq!(config.ollama.api.create, "/create");
        assert_eq!(config.ollama.api.show, "/show");
        assert_eq!(config.ollama.api.model, "test-model");
        assert_eq!(config.ollama.api.options.temperature, 0.0);

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_load_from_file_nonexistent_file() {
        let result = Config::load_from_file("nonexistent_config.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let original_config = Config {
            ollama: OllamaConfig {
                api: ApiConfig {
                    url: "http://test.com/api".to_string(),
                    chat: "/chat".to_string(),
                    create: "/create".to_string(),
                    show: "/show".to_string(),
                    load: "/generate".to_string(),
                    model: "test-model".to_string(),
                    options: ApiOptions { temperature: 0.0 },
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
[ollama.api]
url = ""
chat = ""
create = ""
show = ""
load = ""
model = ""
[ollama.api.options]
temperature = 0
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.ollama.api.url, "");
        assert_eq!(config.ollama.api.chat, "");
        assert_eq!(config.ollama.api.create, "");
        assert_eq!(config.ollama.api.show, "");
        assert_eq!(config.ollama.api.model, "");
        assert_eq!(config.ollama.api.options.temperature, 0.0);

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_with_special_characters() {
        let test_path = "test_config_special_chars.toml";
        let test_content = r#"
[ollama.api]
url = "http://localhost:8080/api"
chat = "/chat?param=value&other=123"
create = "/create-endpoint"
show = "/show"
load = "/generate"
model = "model-with-dashes_and_underscores"
[ollama.api.options]
temperature = 0
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.ollama.api.url, "http://localhost:8080/api");
        assert_eq!(config.ollama.api.chat, "/chat?param=value&other=123");
        assert_eq!(config.ollama.api.create, "/create-endpoint");
        assert_eq!(config.ollama.api.show, "/show");
        assert_eq!(config.ollama.api.load, "/generate");
        assert_eq!(config.ollama.api.options.temperature, 0.0);
        assert_eq!(config.ollama.api.model, "model-with-dashes_and_underscores");

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_with_unicode_content() {
        let test_path = "test_config_unicode.toml";
        let test_content = r#"
[ollama.api]
url = "http://localhost:8080/api"
chat = "/café-chat"
create = "/创建-endpoint"
show = "/show"
load = "/generate"
model = "模型-test-ñoño"
[ollama.api.options]
temperature = 0
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.ollama.api.url, "http://localhost:8080/api");
        assert_eq!(config.ollama.api.chat, "/café-chat");
        assert_eq!(config.ollama.api.create, "/创建-endpoint");
        assert_eq!(config.ollama.api.show, "/show");
        assert_eq!(config.ollama.api.load, "/generate");
        assert_eq!(config.ollama.api.model, "模型-test-ñoño");
        assert_eq!(config.ollama.api.options.temperature, 0.0);

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_get_method_consistency() {
        let config1 = Config::get();
        let config2 = Config::get();

        // Both calls should return the same reference (same memory address)
        assert!(std::ptr::eq(config1, config2));

        // Content should be the same
        assert_eq!(config1.ollama.api.url, config2.ollama.api.url);
        assert_eq!(config1.ollama.api.chat, config2.ollama.api.chat);
        assert_eq!(config1.ollama.api.create, config2.ollama.api.create);
        assert_eq!(config1.ollama.api.show, config2.ollama.api.show);
        assert_eq!(config1.ollama.api.load, config2.ollama.api.load);
        assert_eq!(config1.ollama.api.model, config2.ollama.api.model);
        assert_eq!(
            config1.ollama.api.options.temperature,
            config2.ollama.api.options.temperature
        );
    }

    #[test]
    fn test_api_config_creation() {
        let api_config = ApiConfig {
            url: "http://test.com/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            show: "/show".to_string(),
            load: "/generate".to_string(),
            model: "test-model".to_string(),
            options: ApiOptions { temperature: 0.0 },
        };

        assert_eq!(api_config.url, "http://test.com/api");
        assert_eq!(api_config.chat, "/chat");
        assert_eq!(api_config.create, "/create");
        assert_eq!(api_config.show, "/show");
        assert_eq!(api_config.load, "/generate");
        assert_eq!(api_config.model, "test-model");
        assert_eq!(api_config.options.temperature, 0.0);
    }

    #[test]
    fn test_ollama_config_creation() {
        let ollama_config = OllamaConfig {
            api: ApiConfig {
                url: "http://test.com/api".to_string(),
                chat: "/chat".to_string(),
                create: "/create".to_string(),
                show: "/show".to_string(),
                load: "/generate".to_string(),
                model: "test-model".to_string(),
                options: ApiOptions { temperature: 0.0 },
            },
        };

        assert_eq!(ollama_config.api.url, "http://test.com/api");
        assert_eq!(ollama_config.api.chat, "/chat");
        assert_eq!(ollama_config.api.create, "/create");
        assert_eq!(ollama_config.api.show, "/show");
        assert_eq!(ollama_config.api.load, "/generate");
        assert_eq!(ollama_config.api.model, "test-model");
        assert_eq!(ollama_config.api.options.temperature, 0.0);
    }

    #[test]
    fn test_full_config_creation() {
        let config = Config {
            ollama: OllamaConfig {
                api: ApiConfig {
                    url: "http://test.com/api".to_string(),
                    chat: "/chat".to_string(),
                    create: "/create".to_string(),
                    show: "/show".to_string(),
                    load: "/generate".to_string(),
                    model: "test-model".to_string(),
                    options: ApiOptions { temperature: 0.0 },
                },
            },
        };

        assert_eq!(config.ollama.api.url, "http://test.com/api");
        assert_eq!(config.ollama.api.chat, "/chat");
        assert_eq!(config.ollama.api.create, "/create");
        assert_eq!(config.ollama.api.show, "/show");
        assert_eq!(config.ollama.api.load, "/generate");
        assert_eq!(config.ollama.api.model, "test-model");
        assert_eq!(config.ollama.api.options.temperature, 0.0);
    }

    #[test]
    fn test_config_debug_format() {
        let config = Config {
            ollama: OllamaConfig {
                api: ApiConfig {
                    url: "http://test.com/api".to_string(),
                    chat: "/chat".to_string(),
                    create: "/create".to_string(),
                    show: "/show".to_string(),
                    load: "/generate".to_string(),
                    model: "test-model".to_string(),
                    options: ApiOptions { temperature: 0.0 },
                },
            },
        };

        let debug_string = format!("{:?}", config);
        assert!(debug_string.contains("http://test.com/api"));
        assert!(debug_string.contains("/chat"));
        assert!(debug_string.contains("/create"));
        assert!(debug_string.contains("/show"));
        assert!(debug_string.contains("/generate"));
        assert!(debug_string.contains("0.0"));
        assert!(debug_string.contains("test-model"));
    }

    #[test]
    fn test_config_load_from_file_missing_assistant_section() {
        let test_path = "test_config_no_assistant.toml";
        let test_content = r#"
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
[ollama.api]
url = "http://localhost:8080/api/chat"
model = "test-model"
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(
            result.is_err(),
            "Config should fail to load without assistant.root.name"
        );

        cleanup_test_file(test_path);
    }

    // New tests for endpoint configuration
    #[test]
    fn test_api_config_chat_url() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            show: "/show".to_string(),
            load: "/generate".to_string(),
            model: "qwen3:0.6b".to_string(),
            options: ApiOptions { temperature: 0.0 },
        };

        assert_eq!(api_config.chat_url(), "http://localhost:11434/api/chat");
    }

    #[test]
    fn test_api_config_create_url() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            show: "/show".to_string(),
            load: "/generate".to_string(),
            model: "qwen3:0.6b".to_string(),
            options: ApiOptions { temperature: 0.0 },
        };

        assert_eq!(api_config.create_url(), "http://localhost:11434/api/create");
    }

    #[test]
    fn test_api_config_show_url() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            show: "/show".to_string(),
            load: "/generate".to_string(),
            model: "qwen3:0.6b".to_string(),
            options: ApiOptions { temperature: 0.0 },
        };

        assert_eq!(api_config.show_url(), "http://localhost:11434/api/show");
    }

    #[test]
    fn test_api_config_base_url() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            show: "/show".to_string(),
            load: "/generate".to_string(),
            model: "qwen3:0.6b".to_string(),
            options: ApiOptions { temperature: 0.0 },
        };

        assert_eq!(api_config.base_url(), "http://localhost:11434/api");
    }

    #[test]
    fn test_api_config_endpoint_url() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat".to_string(),
            create: "/create".to_string(),
            show: "/show".to_string(),
            load: "/generate".to_string(),
            model: "qwen3:0.6b".to_string(),
            options: ApiOptions { temperature: 0.0 },
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
            show: "/show".to_string(),
            load: "/generate".to_string(),
            model: "qwen3:0.6b".to_string(),
            options: ApiOptions { temperature: 0.0 },
        };

        assert_eq!(api_config.chat_url(), "http://localhost:11434/api//chat");
        assert_eq!(
            api_config.create_url(),
            "http://localhost:11434/api//create"
        );
        assert_eq!(api_config.show_url(), "http://localhost:11434/api//show");
    }

    #[test]
    fn test_api_config_urls_with_query_parameters() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/chat?stream=false".to_string(),
            create: "/create?format=json".to_string(),
            show: "/show?details=true".to_string(),
            load: "/generate?details=true".to_string(),
            model: "qwen3:0.6b".to_string(),
            options: ApiOptions { temperature: 0.0 },
        };

        assert_eq!(
            api_config.chat_url(),
            "http://localhost:11434/api/chat?stream=false"
        );
        assert_eq!(
            api_config.create_url(),
            "http://localhost:11434/api/create?format=json"
        );
        assert_eq!(
            api_config.show_url(),
            "http://localhost:11434/api/show?details=true"
        );
    }

    #[test]
    fn test_api_config_urls_with_unicode() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "/聊天".to_string(),
            create: "/创建".to_string(),
            show: "/显示".to_string(),
            load: "/显示".to_string(),
            model: "测试模型".to_string(),
            options: ApiOptions { temperature: 0.0 },
        };

        assert_eq!(api_config.chat_url(), "http://localhost:11434/api/聊天");
        assert_eq!(api_config.create_url(), "http://localhost:11434/api/创建");
        assert_eq!(api_config.show_url(), "http://localhost:11434/api/显示");
        assert_eq!(api_config.load_url(), "http://localhost:11434/api/显示");
    }

    #[test]
    fn test_api_config_endpoints_empty_paths() {
        let api_config = ApiConfig {
            url: "http://localhost:11434/api".to_string(),
            chat: "".to_string(),
            create: "".to_string(),
            show: "".to_string(),
            load: "".to_string(),
            model: "qwen3:0.6b".to_string(),
            options: ApiOptions { temperature: 0.0 },
        };

        assert_eq!(api_config.chat_url(), "http://localhost:11434/api");
        assert_eq!(api_config.create_url(), "http://localhost:11434/api");
        assert_eq!(api_config.show_url(), "http://localhost:11434/api");
        assert_eq!(api_config.load_url(), "http://localhost:11434/api");
    }

    #[test]
    fn test_config_load_from_file_missing_ollama_endpoints() {
        let test_path = "test_config_no_endpoints.toml";
        let test_content = r#"
[ollama.api]
url = "http://localhost:8080/api"
model = "test-model"
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
        let show_url = config.ollama.api.show_url();
        let load_url = config.ollama.api.load_url();

        assert!(chat_url.starts_with("http"));
        assert!(create_url.starts_with("http"));
        assert!(show_url.starts_with("http"));
        assert!(chat_url.contains("/chat") || chat_url.ends_with("/api"));
        assert!(create_url.contains("/create") || create_url.ends_with("/api"));
        assert!(show_url.contains("/show") || show_url.ends_with("/api"));
        assert!(load_url.contains("/generate") || load_url.ends_with("/api"));
    }
}
