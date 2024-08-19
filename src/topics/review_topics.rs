use chrono::{DateTime, Days, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
enum NextReviewGap {
    #[default]
    Day,
    Week,
    Month,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReviewTopic {
    pub topic_name: String,
    pub last_reviewed: DateTime<Utc>,
    next_review_gap: NextReviewGap,
}

impl ReviewTopic {
    pub fn new(topic_name: String) -> Self {
        ReviewTopic {
            topic_name,
            last_reviewed: Utc::now(),
            next_review_gap: Default::default(),
        }
    }

    pub fn review(&mut self) {
        match self.next_review_gap {
            NextReviewGap::Day => self.next_review_gap = NextReviewGap::Week,
            NextReviewGap::Week => self.next_review_gap = NextReviewGap::Month,
            NextReviewGap::Month => {}
        }
        self.last_reviewed = Utc::now();
    }

    pub fn is_time_to_review(&self) -> bool {
        let current_time = Utc::now();
        let last_reviewed_time = self.last_reviewed;
        let delta_days = current_time
            .signed_duration_since(last_reviewed_time)
            .num_days();

        match self.next_review_gap {
            NextReviewGap::Day => delta_days >= 1,
            NextReviewGap::Week => delta_days >= 7,
            NextReviewGap::Month => delta_days >= 30,
        }
    }

    #[allow(dead_code)]
    pub fn add_days(&mut self, num: u64) {
        match self.last_reviewed.checked_add_days(Days::new(num)) {
            None => {}
            Some(new_time) => {
                self.last_reviewed = new_time;
            }
        }
    }

    #[allow(dead_code)]
    pub fn sub_days(&mut self, num: u64) {
        match self.last_reviewed.checked_sub_days(Days::new(num)) {
            None => {}
            Some(new_time) => {
                self.last_reviewed = new_time;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::topics::review_topics::{NextReviewGap, ReviewTopic};
    use chrono::Utc;

    #[test]
    fn test_default_topic() {
        let default_review_topic: ReviewTopic = ReviewTopic::new(String::from("test"));

        let topic_name_is_not_empty = !default_review_topic.topic_name.is_empty();
        assert!(topic_name_is_not_empty);

        let is_not_time_to_review = !default_review_topic.is_time_to_review();
        assert!(is_not_time_to_review);
    }

    #[test]
    fn test_updating_review_gap() {
        let mut review_topic: ReviewTopic = ReviewTopic::new(String::from("Review Topic Name"));

        assert_eq!(review_topic.next_review_gap, NextReviewGap::Day);
        review_topic.review();
        assert_eq!(review_topic.next_review_gap, NextReviewGap::Week);
        review_topic.review();
        assert_eq!(review_topic.next_review_gap, NextReviewGap::Month);
    }

    #[test]
    fn test_changing_days() {
        let mut review_topic: ReviewTopic = ReviewTopic::new("test_topic".to_string());

        assert_eq!(
            chrono::Utc::now().date_naive(),
            review_topic.last_reviewed.date_naive()
        );

        review_topic.sub_days(1);

        let delta_time = Utc::now().signed_duration_since(review_topic.last_reviewed);
        assert_eq!(1, delta_time.num_days());

        review_topic.add_days(1);

        let delta_time = Utc::now().signed_duration_since(review_topic.last_reviewed);
        assert_eq!(0, delta_time.num_days());
    }
}
