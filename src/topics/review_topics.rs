use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
enum NextReviewGap {
    #[default]
    Day,
    Week,
    Month,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReviewTopic {
    pub topic_name: String,
    last_reviewed: DateTime<Utc>,
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

    fn review(mut self) -> Self {
        match self.next_review_gap {
            NextReviewGap::Day => self.next_review_gap = NextReviewGap::Week,
            NextReviewGap::Week => self.next_review_gap = NextReviewGap::Month,
            NextReviewGap::Month => {}
        }
        self.last_reviewed = Utc::now();

        self
    }

    fn is_time_to_review(&self) -> bool {
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
}

#[cfg(test)]
mod tests {
    use crate::topics::review_topics::{NextReviewGap, ReviewTopic};

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
        let review_topic: ReviewTopic = ReviewTopic::new(String::from("Review Topic Name"));

        assert_eq!(review_topic.next_review_gap, NextReviewGap::Day);
        let review_topic = review_topic.review();
        assert_eq!(review_topic.next_review_gap, NextReviewGap::Week);
        assert_eq!(review_topic.review().next_review_gap, NextReviewGap::Month);
    }
}
