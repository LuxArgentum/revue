use crate::topics::review_topics::ReviewTopic;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Storage {
    review_topic_list: Vec<ReviewTopic>,
    storage_file_path: String,
}

pub enum PreviousStorage {
    Yes,
    #[allow(dead_code)]
    No,
}

impl Storage {
    /// Create a new Storage struct
    ///
    /// # Arguments
    ///
    /// * `find_prev_storage`: Whether you want to use a previous storage or create a new storage.
    /// * `file_path`: The file path where the previous storage should be found or where to create the new storage. `None` means to use the default location.
    ///
    /// returns: Storage
    ///
    /// # Examples
    ///
    /// This will create a Storage using the previous storage data found in the default location.
    /// ```
    /// let storage = Storage::new(PreviousStorage::Yes, None)
    /// ```
    pub fn new(find_prev_storage: PreviousStorage, file_path: Option<String>) -> Self {
        let review_topic_list: Vec<ReviewTopic> = Vec::new();
        let storage_file_path = file_path.unwrap_or_else(|| "src/data/storage.json".to_string());

        match find_prev_storage {
            PreviousStorage::Yes => {
                let previous_storage_found: bool = find_previous_storage(&storage_file_path);

                if previous_storage_found {
                    return get_previous_storage(&storage_file_path);
                }
            }
            PreviousStorage::No => {}
        }

        Storage {
            review_topic_list,
            storage_file_path,
        }
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

    pub fn remove_review_topic(&mut self, review_topic: &ReviewTopic) {
        if let Some(topic_index) = self
            .review_topic_list
            .iter()
            .position(|topic| topic.topic_name == review_topic.topic_name)
        {
            self.review_topic_list.remove(topic_index);
        }
    }

    pub fn find_review_topic(&mut self, review_topic_name: String) -> Option<&ReviewTopic> {
        if let Some(topic_index) = self
            .review_topic_list
            .iter()
            .position(|topic| topic.topic_name == review_topic_name)
        {
            return self.review_topic_list.get(topic_index);
        }
        None
    }

    pub fn rename_review_topic(
        &mut self,
        review_topic_name: String,
        new_name: String,
    ) -> Result<(), ()> {
        let topic_index = self.review_topic_list
            .iter()
            .position(|topic| topic.topic_name == review_topic_name);
        match topic_index {
            None => Err(()),
            Some(index) => {
                let mut review_topic = self.review_topic_list.remove(index);
                review_topic.topic_name = new_name;
                self.review_topic_list.push(review_topic);
                Ok(())
            }
        }
    }

    pub fn write_storage(&self) {
        let serialized: String = serde_json::to_string_pretty(self).unwrap();
        fs::write(self.storage_file_path.clone(), serialized)
            .expect("Writing to storage.json went wrong");
    }
}

use std::fs;

fn find_previous_storage(storage_file_path: &str) -> bool {
    fs::read_to_string(storage_file_path).is_ok()
}

fn get_previous_storage(storage_file_path: &str) -> Storage {
    let serialized = match fs::read_to_string(storage_file_path) {
        Ok(serialized) => {serialized}
        Err(error) => {
            panic!("Couldn't get serialized from previous storage path: {}", error);
        }
    };
    let storage: Storage = match serde_json::from_str(&serialized) {
        Ok(storage) => storage,
        Err(error) => {
            panic!("Had an error converting json into Storage struct: {}", error);
        }
    };
    
    storage
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use crate::io::storage::{get_previous_storage, PreviousStorage, Storage};
    use crate::topics::review_topics::ReviewTopic;

    static TEST_PATH: &str = "src/data/test_storage.json";

    #[test]
    #[serial]
    fn test_read_write() {
        let storage = Storage::new(PreviousStorage::No, TEST_PATH.to_string().into());
        storage.write_storage();
        assert_eq!(storage, get_previous_storage(TEST_PATH));
    }

    #[test]
    fn update_today_reviews() {
        let test_storage_file_path = TEST_PATH.to_string();
        let mut storage: Storage =
            Storage::new(PreviousStorage::No, test_storage_file_path.into());

        let mut test_1 = ReviewTopic::new("test_1".to_string());
        test_1.sub_days(1);
        storage.add_review_topic(test_1.clone());
        storage.add_review_topic(ReviewTopic::new("test2".to_string()));

        let mut today_topics: Vec<ReviewTopic> = Vec::new();
        let review_list = storage.get_review_topic_list();
        review_list.iter().for_each(|topic| {
            if topic.is_time_to_review() {
                today_topics.push(topic.clone())
            }
        });

        assert_eq!(1, today_topics.len());
        assert_eq!(test_1.topic_name, today_topics.pop().unwrap().topic_name);
    }
    
    #[test]
    #[serial]
    fn test_renaming_topics() {
        let mut storage = Storage::new(PreviousStorage::No, TEST_PATH.to_string().into());
        let review_topic = ReviewTopic::new("Test1".to_string());
        storage.add_review_topic(review_topic);
        assert!(storage.rename_review_topic("Test1".to_string(), "Test1_edited".to_string()).is_ok());
        storage.write_storage();
        assert!(storage.find_review_topic("Test1_edited".to_string()).is_some());
    }
}
