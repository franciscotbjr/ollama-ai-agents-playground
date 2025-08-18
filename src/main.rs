mod config;
use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    println!("Database path: {}", config.database.path);
    println!("Ollama URL: {}", config.ollama.api.url);

    Ok(())
}
