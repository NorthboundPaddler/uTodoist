use std::u8;

use clap::{ArgAction, Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};

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
    #[arg(short = 'j', long, action=ArgAction::SetTrue)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List tasks that match a filter querystring
    List {
        /// Task filter querystring
        #[arg(short = 'f', long)]
        filter: String,

        /// Show the task description
        #[arg(short = 'd', long,action=ArgAction::SetTrue)]
        show_description: bool,
    },

    /// Add a task using Todoist's natural language processing
    Add {},

    /// Complete a task by it's ID
    Complete {},
}

fn build_task_query_uri(filter: &String) -> String {
    format!("{}?filter={}", GET_ACTIVE_TASKS_URL, filter)
}

#[derive(Deserialize, Serialize, Debug)]
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

fn build_description_newline(desc_str: String) -> String {
    if desc_str.is_empty() {
        "".to_string()
    } else {
        "\n\t".to_string()
    }
}

fn list_tasks(
    filter_string: &String,
    show_description: &bool,
    api_key: String,
    json: bool,
) -> Result<(), anyhow::Error> {
    let uri = build_task_query_uri(filter_string);
    let res = reqwest::blocking::Client::new()
        .get(uri)
        .bearer_auth(api_key)
        .send()?;
    let res_json = res.json::<Vec<Task>>()?;
    //println!("{:?}", res_json);
    if json {
        let json_str = serde_json::to_string(&res_json)?;
        println!("{}", json_str);
        Ok(())
    } else {
        for t in res_json {
            let priority_str = colorize_priority(t.priority);
            let id_str = t.id;
            let content_str = t.content.bold();
            let desc_str = t.description;
            if *show_description {
                let desc_str_value = desc_str.clone();
                let newline_str = build_description_newline(desc_str_value);
                let print_str = format!(
                    "({})[{}] {}{}{}",
                    id_str,
                    priority_str,
                    content_str,
                    newline_str,
                    desc_str.italic()
                );
                println!("{}", print_str);
            } else {
                let print_str = format!("({})[{}] {}", id_str, priority_str, content_str);
                println!("{}", print_str);
            }
        }
        Ok(())
    }
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
            show_description,
            cli.api_key.expect("API Key is required"),
            cli.json,
        )?,
        Some(Commands::Add {}) => {}
        Some(Commands::Complete {}) => {}
        &None => {
            todo!();
        }
    }

    Ok(())
}
