use crate::io::console::{display_all, display_today, Cli, CliState, Commands};
use crate::io::storage::{PreviousStorage, Storage};
use crate::topics::review_topics::ReviewTopic;
use clap::Parser;
use std::io::stdout;

mod io;
mod scheduling;
mod topics;

fn main() {
    let cli = Cli::parse();

    let mut storage = Storage::new(PreviousStorage::Yes, None);

    match &cli.command {
        Commands::Edit(edit_args) => {
            match storage.rename_review_topic(
                edit_args.topic_name.to_string(),
                edit_args.new_topic_name.to_string(),
            ) {
                Ok(_) => storage.write_storage(),
                Err(_) => {
                    println!("Review topic was not found.")
                }
            }
        }
        Commands::View { view } => match view {
            CliState::Today => display_today(&storage, stdout()),
            CliState::All => display_all(&storage, stdout()),
        },
        Commands::Add { topic_name } => {
            let new_review_topic = ReviewTopic::new(topic_name.to_string());
            storage.add_review_topic(new_review_topic);
            storage.write_storage();
        }
        Commands::Remove { topic_name } => {
            match storage.clone().find_review_topic(topic_name.to_string()) {
                None => {}
                Some(review_topic) => {
                    storage.remove_review_topic(review_topic);
                    storage.write_storage();
                }
            }
        }
        Commands::Review { topic_name } => {
            let mut review_list = storage.get_review_topic_list();
            let topic_index = review_list.iter().position(|topic| topic.topic_name == *topic_name);

            match topic_index {
                None => { println!("Review topic was not found. Did you misspell?") }
                Some(topic_index) => {
                    let mut review_topic = review_list.remove(topic_index);
                    review_topic.review();
                    review_list.push(review_topic);
                }
            }
        }
    }
}
