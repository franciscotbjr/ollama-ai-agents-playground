# Ollama AI Agents Playground

A Rust-based AI agent system that leverages Ollama for intent classification and task automation. This project demonstrates a modular architecture for building AI-powered agents that can classify user intents and perform automated actions.

## Features

- **Intent Classification**: Uses Ollama with qwen3:0.6b model to classify user intents
- **Modular Agent Architecture**: Extensible system for creating specialized AI agents
- **HTTP Client Infrastructure**: Robust HTTP communication layer
- **Configuration Management**: Lazy-loaded configuration system
- **Comprehensive Testing**: Unit tests for all major components

## Supported Intents

- **SendEmail**: Classifies requests to send emails
- **ScheduleMeeting**: Identifies meeting scheduling requests
- **NoAction**: Fallback for unrecognized intents

## Prerequisites

### Ollama Installation

You must have Ollama installed and running locally on your system.

1. **Install Ollama**: Visit [Ollama's official website](https://ollama.ai/) and follow the installation instructions for your platform.

2. **Pull the qwen3:0.6b model**:
   ```bash
   ollama pull qwen3:0.6b
   ```

3. **Verify Ollama is running**:
   ```bash
   ollama list
   ```
   You should see `qwen3:0.6b` in the model list.

### Model Selection: qwen3:0.6b

The `qwen3:0.6b` model has been selected after testing against several alternatives including `deepseek-r1:1.5b`, `llama3.1`, `mistral-nemo`, and `gemma3`. For the specific use case of intent classification in this project, `qwen3:0.6b` demonstrated:

- **Consistent Results**: Most reliable intent classification accuracy
- **Optimal Response Time**: Best balance between processing speed and accuracy
- **Enhanced User Experience**: Faster interaction cycles for real-time applications
- **System Performance**: Lower resource consumption while maintaining quality

**Note**: While `qwen3:0.6b` excels for this specific intent classification scenario, it doesn't mean it's the optimal choice for all AI use cases. For small-scale classification tasks like this project, it provides an excellent tool that balances performance, accuracy, and resource efficiency.

### Optimization Settings

Through extensive testing, the following configuration optimizations were discovered:

- **Temperature = 0.0**: Setting temperature to zero eliminates randomness, ensuring deterministic and consistent classification results across multiple requests with identical input.

- **Two-Message Structure**: Splitting the classification prompt into separate `system` and `user` role messages significantly improved result consistency:
  - **System message**: Contains classification instructions and examples
  - **User message**: Contains the actual user input to classify
  
  This separation makes the model's responses highly idempotent, providing identical results for the same input across many requests, which is crucial for reliable intent classification in production environments.

For detailed implementation of these optimization techniques, refer to `src/agents/classifier/intent_classifier_agent.rs`.

### Rust Environment

- Rust 1.70+ (2024 edition)
- Cargo package manager

## Quick Start

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd ollama-ai-agents-playground
   ```

2. **Configure the application**:
   Edit `config.toml` to match your environment:
   ```toml
   [database]
   path = "path/to/your/database.db"

   [ollama.api]
   url = "http://localhost:11434/api/chat"
   model = "qwen3:0.6b"
   ```

3. **Build and run**:
   ```bash
   cargo build
   cargo run
   ```

## Project Structure

```
src/
├── agent/                    # Agent system
│   ├── classifier/          # Intent classification agent
│   │   ├── classifier_agent.rs    # Main classifier implementation
│   │   ├── classification_result.rs # Classification result types
│   │   ├── params.rs              # Parameter extraction
│   │   └── response_mapper.rs     # Response mapping utilities
│   ├── email/               # Email agent (future implementation)
│   └── intent.rs            # Intent enumeration
├── infra/                   # Infrastructure layer
│   ├── http/                # HTTP client infrastructure
│   ├── ollama/              # Ollama API integration
│   └── contacts/            # Contact management (future)
├── config.rs                # Configuration management
└── main.rs                  # Demo application
```

## Usage Example

The demo application showcases the ClassifierAgent in action:

```rust
use ollama_ai_agents_playground::agent::{Agent, classifier::ClassifierAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "Send an email to Eva informing that I won't be able to attend the meeting";
    
    let classifier_agent = ClassifierAgent::new();
    let result = classifier_agent.process(input).await;
    
    match result {
        Ok(classification_result) => {
            println!("User intent: {}", classification_result.intent);
        }
        Err(e) => {
            println!("Classification failed: {}", e);
        }
    }
    
    Ok(())
}
```

## Configuration

The application uses a TOML configuration file (`config.toml`) with lazy loading for optimal performance:

- **Database**: SQLite database path for future persistence features
- **Ollama API**: URL and model configuration for AI processing

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test config::
cargo test ollama_request::
cargo test ollama_message::
```

The project includes 100+ unit tests covering:
- Configuration loading and validation
- HTTP client functionality
- Ollama request/response handling
- Intent classification components
- Serialization/deserialization

## Development

### Adding New Agents

1. Create a new module in `src/agent/`
2. Implement the `Agent` trait:
   ```rust
   impl Agent for YourAgent {
       fn process(&self, input: &str) -> impl Future<Output = Result<ClassificationResult, AgentError>> + Send {
           // Your implementation
       }
   }
   ```

### Extending Intent Types

Add new variants to the `Intent` enum in `src/agent/intent.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    SendEmail,
    ScheduleMeeting,
    YourNewIntent,  // Add here
    NoAction,
}
```

## Useful Links

- [Ollama Official Website](https://ollama.ai/)
- [Ollama GitHub Repository](https://github.com/ollama/ollama)
- [qwen3:0.6b Model Documentation](https://ollama.ai/library/qwen3:0.6b)
- [Ollama API Documentation](https://github.com/ollama/ollama/blob/main/docs/api.md)

## Dependencies

- **serde**: Serialization framework
- **tokio**: Async runtime
- **reqwest**: HTTP client
- **once_cell**: Lazy static initialization
- **toml**: Configuration file parsing

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Submit a pull request

## Troubleshooting

### Common Issues

1. **Ollama not responding**: Ensure Ollama is running (`ollama serve`)
2. **Model not found**: Pull the qwen3:0.6b model (`ollama pull qwen3:0.6b`)
3. **Connection refused**: Check if the URL in `config.toml` matches your Ollama installation
4. **Build errors**: Ensure you're using Rust 2024 edition (1.70+)

### Debug Mode

Run with detailed logging:
```bash
RUST_LOG=debug cargo run
```