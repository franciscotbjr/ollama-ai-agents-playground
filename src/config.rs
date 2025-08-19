use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub database: DatabaseConfig,
    pub ollama: OllamaConfig,
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
    pub model: String,
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
url = "http://localhost:8080/api/chat"
model = "test-model"
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.database.path, "/test/database.db");
        assert_eq!(config.ollama.api.url, "http://localhost:8080/api/chat");
        assert_eq!(config.ollama.api.model, "test-model");

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
                    model: "test-model".to_string(),
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
model = ""
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.database.path, "");
        assert_eq!(config.ollama.api.url, "");
        assert_eq!(config.ollama.api.model, "");

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_with_special_characters() {
        let test_path = "test_config_special_chars.toml";
        let test_content = r#"
[database]
path = "C:\\Users\\Test User\\database with spaces.db"

[ollama.api]
url = "http://localhost:8080/api/chat?param=value&other=123"
model = "model-with-dashes_and_underscores"
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(
            config.database.path,
            "C:\\Users\\Test User\\database with spaces.db"
        );
        assert_eq!(
            config.ollama.api.url,
            "http://localhost:8080/api/chat?param=value&other=123"
        );
        assert_eq!(config.ollama.api.model, "model-with-dashes_and_underscores");

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_config_with_unicode_content() {
        let test_path = "test_config_unicode.toml";
        let test_content = r#"
[database]
path = "/测试/数据库.db"

[ollama.api]
url = "http://localhost:8080/api/café"
model = "模型-test-ñoño"
"#;

        create_test_config_file(test_path, test_content).expect("Failed to create test file");

        let result = Config::load_from_file(test_path);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.database.path, "/测试/数据库.db");
        assert_eq!(config.ollama.api.url, "http://localhost:8080/api/café");
        assert_eq!(config.ollama.api.model, "模型-test-ñoño");

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
        assert_eq!(config1.ollama.api.model, config2.ollama.api.model);
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
            url: "http://test.com".to_string(),
            model: "test-model".to_string(),
        };

        assert_eq!(api_config.url, "http://test.com");
        assert_eq!(api_config.model, "test-model");
    }

    #[test]
    fn test_ollama_config_creation() {
        let ollama_config = OllamaConfig {
            api: ApiConfig {
                url: "http://test.com".to_string(),
                model: "test-model".to_string(),
            },
        };

        assert_eq!(ollama_config.api.url, "http://test.com");
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
                    url: "http://test.com".to_string(),
                    model: "test-model".to_string(),
                },
            },
        };

        assert_eq!(config.database.path, "/test/db.db");
        assert_eq!(config.ollama.api.url, "http://test.com");
        assert_eq!(config.ollama.api.model, "test-model");
    }

    #[test]
    fn test_config_debug_format() {
        let config = Config {
            database: DatabaseConfig {
                path: "/test/db.db".to_string(),
            },
            ollama: OllamaConfig {
                api: ApiConfig {
                    url: "http://test.com".to_string(),
                    model: "test-model".to_string(),
                },
            },
        };

        let debug_string = format!("{:?}", config);
        assert!(debug_string.contains("Config"));
        assert!(debug_string.contains("/test/db.db"));
        assert!(debug_string.contains("http://test.com"));
        assert!(debug_string.contains("test-model"));
    }
}
