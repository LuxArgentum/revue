use crate::io::storage::Storage;
use crate::topics::review_topics::ReviewTopic;
use std::io;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliState {
    /// View today's review topics
    Today,
    /// View all the review topics
    All,
}

#[derive(Subcommand)]
pub enum Commands {
    View { view: CliState },
    Edit(EditArgs),
    Add { topic_name: String },
    Remove { topic_name: String },
    Review { topic_name: String },
}

#[derive(Args)]
pub struct EditArgs {
    pub topic_name: String,
    pub new_topic_name: String,
}

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub fn display_today(storage: &Storage, mut writer: impl io::Write) {
    writeln!(writer, "\nToday's Review Topics:").expect("Console<Today> header display failed");

    let review_topic_list: Vec<ReviewTopic> = storage.get_review_topic_list();
    let mut review_topic_not_found: bool = true;
    review_topic_list.iter().for_each(|topic| {
        if topic.is_time_to_review() {
            if review_topic_not_found {
                review_topic_not_found = false
            }
            writeln!(writer, "{}", topic.topic_name).expect("Console<Today> topic display failed")
        }
    });

    if review_topic_not_found {
        writeln!(writer, "No review topics for today")
            .expect("Console<Today> no topic display failed")
    }
}

pub fn display_all(storage: &Storage, mut writer: impl io::Write) {
    writeln!(writer, "\nAll Review Topics:").expect("Console<All> header display failed");
    let list = storage.get_review_topic_list();
    list.iter().for_each(|topic| {
        writeln!(writer, "{}", topic.topic_name).expect("Console<All> topic display failed")
    });
}

#[cfg(test)]
mod tests {
}
