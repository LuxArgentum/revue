use crate::topics::review_topics::ReviewTopic;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct Storage {
    review_topic_list: Vec<ReviewTopic>,
}

impl Storage {
    pub fn new() -> Self {
        let mut review_topic_list: Vec<ReviewTopic> = Vec::new();

        let previous_storage_found: bool = find_previous_storage();

        if previous_storage_found {
            review_topic_list = get_previous_storage();
        }

        Storage { review_topic_list }
    }

    fn get_review_topic_list(&self) -> Vec<ReviewTopic> {
        self.review_topic_list.clone()
    }

    fn add_review_topic(&mut self, review_topic: ReviewTopic) {
        if let Some(_duplicate_found) = self
            .review_topic_list
            .iter()
            .find(|topic| topic.topic_name == review_topic.topic_name)
        {
            return;
        }

        self.review_topic_list.push(review_topic);
    }

    fn remove_review_topic(&mut self, review_topic: ReviewTopic) {
        if let Some(topic_at_index) = self
            .review_topic_list
            .iter()
            .position(|topic| topic.topic_name == review_topic.topic_name)
        {
            self.review_topic_list.remove(topic_at_index);
        }
    }

    fn find_review_topic(&mut self, review_topic_name: String) -> Option<&ReviewTopic> {
        if let Some(topic_at_index) = self
            .review_topic_list
            .iter()
            .position(|topic| topic.topic_name == review_topic_name)
        {
            return self.review_topic_list.get(topic_at_index);
        }
        None
    }
}

fn find_previous_storage() -> bool {
    // TODO: Add a config of sorts where storage should be searched for
    todo!()
}

fn get_previous_storage() -> Vec<ReviewTopic> {
    todo!()
}
