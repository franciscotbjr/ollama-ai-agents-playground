mod config;

use ollama_ai_agents_playground::{
    agents::{
        assistant::{
            build_assistant_name, check_assistant_agent::CheckParam, create_assistant_agent::CreateParam, load_assistant_agent::{self, LoadParam}, CheckAssistantAgent, CreateAssistantAgent, LoadAssistantAgent
        }, classifier::{IntentClassifierAgent, IntentParam}, Agent
    },
    config::Config,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a tokio runtime for the async example
    println!();
    println!("🚀 Starting asynchronous processing...");
    println!("🚀 Starting classifier...");
    println!();

    println!("🚀 Loading user settings...");
    // Access User Settings
    let user_settings = Config::get().user.settings.clone();

    println!("🚀 Checking if assistant already created...");
    // Check if user assistant exists
    let check_result = CheckAssistantAgent::default()
        .process(CheckParam::new(build_assistant_name(user_settings.assistant.as_str())))
        .await;

    match check_result {
        Ok(check) => {
            if check.exists {
                println!(" Assistant already created...");
                println!("🚀 Loading assistant into system memory...");
                // If user assistant exists, load it
                let load_result = LoadAssistantAgent::default()
                    .process(LoadParam::new(build_assistant_name(user_settings.assistant.as_str())))
                    .await;
                // Just log result
                match load_result {
                    Ok(load) => {
                        println!(" Assistant loaded into system memory: {}", load.success)
                    }
                    Err(e) => {
                        println!(" Failed to load assistant: {e}")
                    }
                }
            } else {
                println!(" Assistant will be created...");
                println!("🚀 Creating assistant...");
                // Else, create an assistante model customized for the user
                let create_param = CreateParam::new(
                    user_settings.name.to_owned(),
                    user_settings.assistant.to_owned(),
                );
                let create_assistant_agent = CreateAssistantAgent::new();
                let create = create_assistant_agent.process(create_param).await;
                match create {
                    Ok(create_result) => {
                        println!();
                        println!("🚀 Creation done!");
                        println!("Success: {}", create_result.success);
                        println!();
                    }
                    Err(e) => {
                        println!("Creation Failed: {e}");
                    }
                }
            }
        }
        Err(e) => {
            println!("Check Failed: {e}");
        }
    }

    println!("🚀 Asking assistant to classify user message...");
    let input = "Envie um e-mail para Eva informando que não vou poder comparecer à reunião e que peço desculpas por avisar tão em cima da hora.";
    let intent_classifier_agent = IntentClassifierAgent::default();
    let result = intent_classifier_agent
        .process(IntentParam::new(input.to_string(), user_settings.assistant))
        .await;
    match result {
        Ok(classification_result) => {
            println!();
            println!("🚀 Classification done!");
            println!("User intent: {}", classification_result.intent);
            println!(
                "User recipient: {}",
                classification_result.params.recipient().unwrap()
            );
            println!();
        }
        Err(e) => {
            println!("Classification Failed: {e}");
        }
    }

    Ok(())
}
