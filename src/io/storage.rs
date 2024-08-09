use crate::topics::review_topics::ReviewTopic;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct Storage {
    review_topic_list: Vec<ReviewTopic>,
}

impl Storage {
    pub fn new() -> Self {
        let review_topic_list: Vec<ReviewTopic> = Vec::new();

        let previous_storage_found: bool = find_previous_storage();

        if previous_storage_found {
            return get_previous_storage();
        }

        Storage { review_topic_list }
    }

    pub fn get_review_topic_list(&self) -> Vec<ReviewTopic> {
        self.review_topic_list.clone()
    }

    pub fn add_review_topic(&mut self, review_topic: ReviewTopic) {
        if let Some(_duplicate_found) = self
            .review_topic_list
            .iter()
            .find(|topic| topic.topic_name == review_topic.topic_name)
        {
            return;
        }

        self.review_topic_list.push(review_topic);
    }

    pub fn remove_review_topic(&mut self, review_topic: ReviewTopic) {
        if let Some(topic_at_index) = self
            .review_topic_list
            .iter()
            .position(|topic| topic.topic_name == review_topic.topic_name)
        {
            self.review_topic_list.remove(topic_at_index);
        }
    }

    pub fn find_review_topic(&mut self, review_topic_name: String) -> Option<&ReviewTopic> {
        if let Some(topic_at_index) = self
            .review_topic_list
            .iter()
            .position(|topic| topic.topic_name == review_topic_name)
        {
            return self.review_topic_list.get(topic_at_index);
        }
        None
    }

    pub fn write_storage(&self) {
        let expected_storage_location = "src/data/storage.json";
        let serialized: String = serde_json::to_string_pretty(self).unwrap();
        fs::write(expected_storage_location, serialized)
            .expect("Writing to storage.json went wrong");
    }
}

use std::fs;

fn find_previous_storage() -> bool {
    // TODO: Add a config of sorts where storage should be searched for
    let expected_storage_location = "src/data/storage.json";
    fs::read_to_string(expected_storage_location).is_ok()
}

fn get_previous_storage() -> Storage {
    let expected_storage_location = "src/data/storage.json";
    let serialized = fs::read_to_string(expected_storage_location).unwrap();
    let storage: Storage = serde_json::from_str(&serialized).unwrap();

    storage
}

#[cfg(test)]
mod tests {
    use crate::io::storage::{get_previous_storage, Storage};

    #[test]
    fn test_read_write() {
        let storage = Storage::new();
        storage.write_storage();
        assert_eq!(storage, get_previous_storage());
    }
}
