pub trait PublishedMessage {
    fn index(&self) -> usize
    where
        Self: Sized;
    fn id(&self) -> &str
    where
        Self: Sized;

    fn with_index(index: i64) -> Self
    where
        Self: Sized;
}
