pub trait FromMarkdownJson<T> {
    fn from_markdown_text(markdown_text: &str) -> Result<T, Box<dyn std::error::Error>>;
}