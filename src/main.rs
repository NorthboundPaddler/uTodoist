use std::u8;

use clap::{ArgAction, Parser, Subcommand};
use colored::Colorize;
use serde::Deserialize;

const GET_ACTIVE_TASKS_URL: &str = "https://api.todoist.com/rest/v2/tasks";

// Take in command line arguments for filter and API key (this specifically is for the list tasks
// request). Will need to break this out when getting to "add" and "complete" tools for the cli.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // The command to use (List, Add, Complete)
    #[command(subcommand)]
    command: Option<Commands>,

    /// Your Todoist API key
    #[arg(global = true)]
    #[arg(short = 'k', long)]
    api_key: Option<String>,

    /// Return output as JSON
    #[arg(global = true)]
    #[arg(short = 'j', long, action=ArgAction::SetFalse)]
    json: Option<bool>,
}

#[derive(Subcommand)]
enum Commands {
    /// List tasks that match a filter querystring
    List {
        /// Task filter querystring
        #[arg(short = 'f', long)]
        filter: String,

        /// Show the task description
        #[arg(short = 'd', long,action=ArgAction::SetFalse)]
        show_description: Option<bool>,
    },

    /// Add a task using Todoist's natural language processing
    Add {},

    /// Complete a task by it's ID
    Complete {},
}

fn build_task_query_uri(filter: &String) -> String {
    format!("{}?filter={}", GET_ACTIVE_TASKS_URL, filter)
}

#[derive(Deserialize, Debug)]
struct Task {
    content: String,
    description: String,
    id: String,
    priority: u8,
}

fn colorize_priority(priority: u8) -> colored::ColoredString {
    if priority == 1 {
        "p4".normal()
    } else if priority == 2 {
        "p3".blue()
    } else if priority == 3 {
        "p2".yellow()
    } else if priority == 4 {
        "p1".red().bold()
    } else {
        "ERR".cyan().bold()
    }
}

fn list_tasks(
    filter_string: &String,
    show_description: bool,
    api_key: String,
) -> Result<(), anyhow::Error> {
    let uri = build_task_query_uri(filter_string);
    let res = reqwest::blocking::Client::new()
        .get(uri)
        .bearer_auth(api_key)
        .send()?;
    let res_json = res.json::<Vec<Task>>()?;
    //println!("{:?}", res_json);
    for t in res_json {
        let priority_str = colorize_priority(t.priority);
        let id_str = t.id;
        let content_str = t.content.bold();
        let desc_str = t.description.italic();
        println!("{}", show_description);
        if show_description {
            let print_str = format!("({})[{}] {}", id_str, priority_str, content_str);
            println!("{}", print_str);
        } else {
            let print_str = format!(
                "({})[{}] {}\n\t{}",
                id_str, priority_str, content_str, desc_str
            );
            println!("{}", print_str);
        }
    }
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Args::parse();
    //TODO throw error/exit if no API key is given
    match &cli.command {
        Some(Commands::List {
            filter,
            show_description,
        }) => list_tasks(
            filter,
            show_description.is_some(),
            cli.api_key.expect("API Key is required"),
        )?,
        Some(Commands::Add {}) => {}
        Some(Commands::Complete {}) => {}
        &None => {
            todo!();
        }
    }

    Ok(())
}
