use crate::io::storage::Storage;
use crate::topics::review_topics::ReviewTopic;

pub fn start_console() {
    println!("|----- SIR-Tracker -----|");

    let mut storage: Storage = Storage::new();

    display_todays_review_topics(&storage);

    display_all_topics(&storage);

    storage.write_storage();
}

fn display_all_topics(storage: &Storage) {
    println!("\nAll Review Topics:");
    let list = storage.get_review_topic_list();
    list.iter()
        .for_each(|topic| println!("{}", topic.topic_name));
}

fn display_todays_review_topics(storage: &Storage) {
    println!("\nToday's Review Topics:");

    let review_topic_list: Vec<ReviewTopic> = storage.get_review_topic_list();
    let mut review_topic_not_found: bool = true;
    review_topic_list.iter().for_each(|topic| {
        if topic.is_time_to_review() {
            if review_topic_not_found {
                review_topic_not_found = false
            }
            println!("{}", topic.topic_name)
        }
    });

    if review_topic_not_found {
        println!("No review topics for today")
    }
}
