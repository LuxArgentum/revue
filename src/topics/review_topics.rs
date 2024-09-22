use chrono::{DateTime, Days, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use tabled::Tabled;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Ord, PartialOrd, Tabled)]
pub enum NextReviewGap {
    #[default]
    Day,
    Week,
    Month,
}

impl fmt::Display for NextReviewGap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NextReviewGap::Day => {write!(f, "Day")}
            NextReviewGap::Week => {write!(f, "Week")}
            NextReviewGap::Month => {write!(f, "Month")}
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ReviewTopic {
    pub topic_name: String,
    pub last_reviewed: DateTime<Local>,
    pub next_review_gap: NextReviewGap,
}

impl PartialOrd for ReviewTopic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ReviewTopic {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.days_until_review().cmp(&other.days_until_review()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.topic_name.cmp(&other.topic_name),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl ReviewTopic {
    pub fn new(topic_name: String) -> Self {
        ReviewTopic {
            topic_name,
            last_reviewed: Local::now(),
            next_review_gap: Default::default(),
        }
    }

    pub fn review(&mut self) {
        match self.next_review_gap {
            NextReviewGap::Day => self.next_review_gap = NextReviewGap::Week,
            NextReviewGap::Week => self.next_review_gap = NextReviewGap::Month,
            NextReviewGap::Month => {}
        }
        self.last_reviewed = Local::now();
    }

    pub fn is_time_to_review(&self) -> bool {
        let days_until_review = self.days_until_review();
        // Less than 1 rather than 0 because of day offset in days_until_review
        let time_to_review: bool = days_until_review.le(&0);
        time_to_review
    }

    pub fn days_until_review(&self) -> i64 {
        let current_date: NaiveDate = Local::now().date_naive();

        let days_to_add = match self.next_review_gap {
            NextReviewGap::Day => 1,
            NextReviewGap::Week => 7,
            NextReviewGap::Month => 30,
        };
        let review_day: NaiveDate = match self.last_reviewed.date_naive().checked_add_days(Days::new(days_to_add)) {
            None => {
                panic!("Failed to get review day");
            }
            Some(review_day) => review_day,
        };

        review_day.signed_duration_since(current_date).num_days()
    }

    #[cfg(test)]
    pub fn add_days(&mut self, num: u64) {
        match self.last_reviewed.checked_add_days(Days::new(num)) {
            None => {}
            Some(new_time) => {
                self.last_reviewed = new_time;
            }
        }
    }

    #[cfg(test)]
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
    use chrono::Local;

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
            Local::now().date_naive(),
            review_topic.last_reviewed.date_naive()
        );

        review_topic.sub_days(1);

        let delta_time = Local::now().signed_duration_since(review_topic.last_reviewed);
        assert_eq!(1, delta_time.num_days());

        review_topic.add_days(1);

        let delta_time = Local::now().signed_duration_since(review_topic.last_reviewed);
        assert_eq!(0, delta_time.num_days());
    }

    #[test]
    fn test_review() {
        let mut review_topic: ReviewTopic = ReviewTopic::new("test1".to_owned());
        review_topic.review();
        assert_eq!(NextReviewGap::Week, review_topic.next_review_gap);
    }

    #[test]
    fn test_compare_review_gaps() {
        assert!(NextReviewGap::Day < NextReviewGap::Week);
        assert!(NextReviewGap::Week < NextReviewGap::Month);
        assert_eq!(NextReviewGap::Day, NextReviewGap::Day);

        let mut topic1: ReviewTopic = ReviewTopic::new("z".to_owned());
        topic1.review();

        let topic2: ReviewTopic = ReviewTopic::new("a".to_owned());

        assert!(topic1 > topic2);
    }

    #[test]
    fn test_is_review_day() {
        let mut topic = ReviewTopic::new("topic".to_owned());
        assert_eq!(1, topic.days_until_review());
        assert!(!topic.is_time_to_review());

        topic.sub_days(1);
        assert_eq!(0, topic.days_until_review());
        assert!(topic.is_time_to_review());
    }

    #[test]
    fn test_days_until_review() {}
}
