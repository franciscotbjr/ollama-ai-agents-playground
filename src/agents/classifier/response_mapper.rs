// Este arquivo foi refatorado seguindo o princípio "um tipo por arquivo" (Design Source).
// Os tipos foram distribuídos para arquivos individuais:
//   - MapperError           → mapper_error.rs
//   - Mapper<T, U>          → mapper.rs
//   - OllamaToClassificationMapper + map_ollama_to_classification → ollama_to_classification_mapper.rs
//   - ToClassificationResult → to_classification_result.rs
//
// Re-exports mantidos para compatibilidade via classifier/mod.rs.
