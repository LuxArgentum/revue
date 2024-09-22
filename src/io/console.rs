use crate::io::storage::Storage;
use crate::topics::review_topics::{NextReviewGap, ReviewTopic};
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::debug;
use std::io;
use std::ops::Add;
use chrono::Local;

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

    let today_list: Vec<ReviewTopic> = get_today_list(&review_topic_list);

    let review_topic_not_found: bool = today_list.is_empty();

    if review_topic_not_found {
        writeln!(writer, "No review topics for today")
            .expect("Console<Today> no topic display failed")
    } else {
        display_table_today(&today_list, writer);
    }

    // review_topic_list.iter().for_each(|topic| {
    //     if topic.is_time_to_review() {
    //         if review_topic_not_found {
    //             review_topic_not_found = false
    //         }
    //         writeln!(writer, "{}", topic.topic_name).expect("Console<Today> topic display failed")
    //     }
    // });
}

fn get_today_list(review_list: &[ReviewTopic]) -> Vec<ReviewTopic> {
    let mut today_list: Vec<ReviewTopic> = Vec::new();

    review_list.iter().for_each(|review_topic| {
        if review_topic.is_time_to_review() {
            today_list.push(review_topic.clone());
        }
    });

    debug!("Today list: {:#?}", today_list);
    today_list
}

pub fn display_all(storage: &Storage, mut writer: impl io::Write) {
    writeln!(writer, "\nAll Review Topics:").expect("Console<All> header display failed");
    let list = storage.get_review_topic_list();

    display_table_all(&list, writer);

    // list.iter().for_each(|topic| {
    //     writeln!(writer, "{}", topic.topic_name).expect("Console<All> topic display failed")
    // });
}

use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Tabled, Debug)]
struct TableTopicToday {
    name: String,
    days_since_last_reviewed: i64,
    review_gap: NextReviewGap,
}

impl TableTopicToday {
    fn new(topic: &ReviewTopic) -> TableTopicToday {
        let days_since_review = Local::now()
            .signed_duration_since(topic.last_reviewed)
            .num_days();
        TableTopicToday {
            name: topic.topic_name.clone(),
            days_since_last_reviewed: days_since_review,
            review_gap: topic.next_review_gap.clone(),
        }
    }
}

fn display_table_today(list: &[ReviewTopic], mut writer: impl io::Write) {
    let table_list: Vec<TableTopicToday> = convert_topic_to_table_today(list);
    let table_string = Table::new(table_list).with(Style::sharp()).to_string();

    writeln!(writer, "{}", table_string).expect("Writing the table failed");
}

fn convert_topic_to_table_today(list: &[ReviewTopic]) -> Vec<TableTopicToday> {
    let mut table_list: Vec<TableTopicToday> = Vec::new();
    list.iter().for_each(|topic| {
        table_list.push(TableTopicToday::new(topic));
    });
    debug!("Table list: {:#?}", table_list);
    table_list
}

#[derive(Tabled, Debug)]
struct TableTopicAll {
    name: String,
    review_in_days: String,
    review_gap: NextReviewGap,
}

impl TableTopicAll {
    fn new(topic: &ReviewTopic) -> TableTopicAll {
        let mut review_in_days: String = "Today".to_string();

        if !topic.is_time_to_review() {
            let num_of_days = topic.days_until_review();
            let mut day_string = " Days";
            if num_of_days == 1 {
                day_string = " Day";
            }
            review_in_days = num_of_days.to_string().add(day_string);
        }

        TableTopicAll {
            name: topic.topic_name.clone(),
            review_in_days,
            review_gap: topic.next_review_gap.clone(),
        }
    }
}

pub fn display_table_all(list: &[ReviewTopic], mut writer: impl io::Write) {
    let table_list: Vec<TableTopicAll> = convert_topic_to_table_all(list);
    let table_string = Table::new(table_list).with(Style::sharp()).to_string();

    writeln!(writer, "{}", table_string).expect("Writing the table failed");
}

fn convert_topic_to_table_all(list: &[ReviewTopic]) -> Vec<TableTopicAll> {
    let mut table_list: Vec<TableTopicAll> = Vec::new();
    list.iter().for_each(|topic| {
        table_list.push(TableTopicAll::new(topic));
    });
    debug!("Table list: {:#?}", table_list);
    table_list
}

#[cfg(test)]
mod tests {}
