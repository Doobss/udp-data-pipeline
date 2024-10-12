use std::collections::HashMap;

use super::PublishedMessage;

pub struct MessageProducer<T>
where
    T: PublishedMessage + Clone,
{
    produced_messages: HashMap<i64, T>,
    index: i64,
}

impl<T> MessageProducer<T>
where
    T: PublishedMessage + Clone,
{
    pub fn next_message(&mut self) -> T {
        let next_index = self.index;
        self.index += 1;
        let next_message = T::with_index(next_index);
        self.produced_messages
            .insert(next_index, next_message.clone());
        next_message
    }

    pub fn get_message(&self, key: &i64) -> Option<&T> {
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
            index: Default::default(),
        }
    }
}
