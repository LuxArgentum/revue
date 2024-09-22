use crate::io::console::{display_all, display_today, Cli, CliState, Commands};
use crate::io::storage::{PreviousStorage, Storage};
use crate::topics::review_topics::ReviewTopic;
use clap::Parser;
use std::io::stdout;

mod io;
mod topics;

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    let mut storage = Storage::new(PreviousStorage::Yes, None);

    match &cli.command {
        // Edit command allows changing the name of a topic to a new name
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

        // View command allows viewing today's topics or all of the topics
        Commands::View { view } => match view {
            CliState::Today => display_today(&storage, stdout()),
            CliState::All => display_all(&storage, stdout()),
        },

        // Add command allows adding new topics to the storage
        Commands::Add { topic_name } => {
            let new_review_topic = ReviewTopic::new(topic_name.to_string());
            storage.add_review_topic(new_review_topic);
            storage.write_storage();
        }

        // Remove command allows removing topics from the storage
        Commands::Remove { topic_name } => {
            match storage.clone().find_review_topic(topic_name.to_string()) {
                None => {}
                Some(review_topic) => {
                    storage.remove_review_topic(review_topic);
                    storage.write_storage();
                }
            }
        }

        // Review command marks the provided topic as reviewed and recalculates when to review it next
        Commands::Review { topic_name } => {
            storage.review_topic(topic_name.to_owned());
        }
    }
}
