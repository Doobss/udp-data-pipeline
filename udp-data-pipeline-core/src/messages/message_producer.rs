use super::PublishedMessage;

pub struct MessageProducer<T>
where
    T: PublishedMessage + Clone,
{
    produced_messages: Vec<T>,
}

impl<T> MessageProducer<T>
where
    T: PublishedMessage + Clone,
{
    pub fn next_message(&mut self) -> T {
        let next_index = self.produced_messages.len() as i64;
        let next_message = T::with_index(next_index);
        self.produced_messages.push(next_message.clone());
        next_message
    }

    pub fn get_message(&self, key: usize) -> Option<&T> {
        self.produced_messages.get(key)
    }
}

impl<T> Default for MessageProducer<T>
where
    T: PublishedMessage + Clone,
{
    fn default() -> Self {
        Self {
            produced_messages: Default::default(),
        }
    }
}
