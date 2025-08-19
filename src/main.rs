mod config;
use config::Config;
use ollama_ai_agents_playground::agent::{classifier::{self, ClassifierAgent}, Agent, ClassificationResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::get();
    println!("Database path: {}", config.database.path);
    println!("Ollama URL: {}", config.ollama.api.url);

    // Create a tokio runtime for the async example
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(async {
        let input = "Envie um e-mail para Eva informando que não vou poder comparecer à reunião e que peço desculpas por avisar tão em cima da hora.";
    
        let classifier_agent = ClassifierAgent::new();
        let result = classifier_agent.process(input).await;
    
        match result {
            Ok(classification_result) => {
    
                println!("User intent: {}", classification_result.intent);
            }
            Err(e) => {
                println!("Failed: {}", e);
            }
        }
    });


    Ok(())
}
