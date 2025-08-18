pub mod classification_result;
pub mod classifier_agent;
pub mod classifier_promp;
pub mod params;
pub mod response_mapper;

pub use classification_result::ClassificationResult;
pub use classifier_agent::ClassifierAgent;
pub use classifier_promp::ClassifierPrompt;
pub use params::Params;
pub use response_mapper::{
    Mapper, MapperError, OllamaToClassificationMapper, ToClassificationResult,
    map_ollama_to_classification,
};
