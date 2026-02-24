use crate::config::Config;

pub fn build_assistant_name(name: &str) -> String {
    let config = Config::get();
    config.assistant.root.to_name(name)
}
