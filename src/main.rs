mod config;
use ollama_ai_agents_playground::agent::{
    Agent,
    assistant::{
        self, CreateAssistantAgent,
        create_assistant_agent::{self, CreateParam},
    },
    classifier::{IntentClassifierAgent, IntentParam},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an assistante model customized for the user
    let user_name = "Ana".to_string();
    let assistant_named = "Tereza".to_string();
    let create_param = CreateParam::new(user_name, assistant_named.clone());
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
            println!("Creation Failed: {}", e);
        }
    }

    // Create a tokio runtime for the async example
    println!();
    println!("🚀 Starting asynchronous processing...");
    println!("🚀 Starting classifier...");
    println!();
    let input = "Envie um e-mail para Eva informando que não vou poder comparecer à reunião e que peço desculpas por avisar tão em cima da hora.";
    let intent_classifier_agent = IntentClassifierAgent::new();
    let result = intent_classifier_agent
        .process(IntentParam::new(input.to_string(), assistant_named))
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
            println!("Classification Failed: {}", e);
        }
    }

    Ok(())
}
