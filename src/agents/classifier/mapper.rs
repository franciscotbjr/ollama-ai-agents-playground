/// Generic mapper trait for converting between types.
pub trait Mapper<T, U> {
    type Error;

    fn map(source: T) -> Result<U, Self::Error>;
}
