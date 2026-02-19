/// Example: Classify user intent using the IntentClassifierAgent.
///
/// Requires a running Ollama instance at http://localhost:11434
/// and a model matching the assistant name in config.toml.
///
/// Run with:
///   cargo run --example classify_intent
use ollama_ai_agents_playground::{
    agents::{Agent, classifier::{IntentClassifierAgent, IntentParam}},
    config::Config,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_settings = Config::get().user.settings.clone();

    let input = "Envie um e-mail para Eva informando que não vou poder comparecer à reunião.";

    println!("Input: {input}");
    println!("Assistant: {}", user_settings.assistant);
    println!();

    let agent = IntentClassifierAgent::default();
    let param = IntentParam::new(input.to_string(), user_settings.assistant);

    match agent.process(param).await {
        Ok(result) => {
            println!("Intent:    {}", result.intent);
            println!("Recipient: {}", result.params.recipient().unwrap_or("(none)"));
            println!("Message:   {}", result.params.message().unwrap_or("(none)"));
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    Ok(())
}
