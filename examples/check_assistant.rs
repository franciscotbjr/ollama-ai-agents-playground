/// Example: Check whether a personal assistant model exists in Ollama.
///
/// Requires a running Ollama instance at http://localhost:11434.
///
/// Run with:
///   cargo run --example check_assistant
use ollama_ai_agents_playground::{
    agents::{Agent, assistant::{CheckAssistantAgent, CheckParam, build_assistant_name}},
    config::Config,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_settings = Config::get().user.settings.clone();
    let assistant_name = build_assistant_name(user_settings.assistant.as_str());

    println!("Checking assistant: {assistant_name}");
    println!();

    let agent = CheckAssistantAgent::default();
    let param = CheckParam::new(assistant_name.clone());

    match agent.process(param).await {
        Ok(result) => {
            if result.exists {
                println!("Assistant '{assistant_name}' exists in Ollama.");
            } else {
                println!("Assistant '{assistant_name}' does NOT exist in Ollama.");
                println!("Hint: run the 'create_assistant' example to create it.");
            }
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    Ok(())
}
