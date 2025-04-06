use clap::{ArgAction, Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, u8};

const REST_API_TASK_URL: &str = "https://api.todoist.com/rest/v2/tasks";
const QUICK_ADD_ITEM_URL: &str = "https://api.todoist.com/sync/v9/quick/add";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // The subcommand to use (List, Add, Complete)
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
    Add {
        /// Text content to be processed
        #[arg(short = 't', long)]
        text: String,
    },

    /// Complete a task by it's ID
    Complete {
        /// ID of the task to Complete
        #[arg(short = 'i', long)]
        id: u64,
    },
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    content: String,
    description: String,
    id: String,
    priority: u8,
}

#[derive(Deserialize, Serialize, Debug)]
struct QuickAdd {
    id: String,
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
    api_key: &String,
    json: bool,
) -> Result<(), anyhow::Error> {
    let uri = format!("{}?filter={}", REST_API_TASK_URL, filter_string);
    let res = reqwest::blocking::Client::new()
        .get(uri)
        .bearer_auth(api_key)
        .send()?;
    if json {
        println!("{}", res.text()?);
        Ok(())
    } else {
        let res_json = res.json::<Vec<Task>>()?;
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

fn quick_add_item(text: &String, api_key: &String, json: bool) -> Result<(), anyhow::Error> {
    let uri = QUICK_ADD_ITEM_URL;
    let mut params = HashMap::new();
    params.insert("text", text);
    let res = reqwest::blocking::Client::new()
        .post(uri)
        .bearer_auth(api_key)
        .form(&params)
        .send()?;
    if json {
        println!("{}", res.text()?);
    } else {
        let res_json = res.json::<QuickAdd>()?;
        println!("{}", res_json.id);
    }
    Ok(())
}

fn close_task(id: &u64, api_key: &String) -> Result<(), anyhow::Error> {
    let uri = format!("{}/{}/close", REST_API_TASK_URL, id);
    let res = reqwest::blocking::Client::new()
        .post(uri)
        .bearer_auth(api_key)
        .send()?;
    assert_eq!(res.status().as_u16(), 204); // API returns no content on success
    Ok(())
}

fn run() -> Result<(), anyhow::Error> {
    let cli = Args::parse();
    //TODO throw error/exit if no API key is given
    let api_key = &cli.api_key.expect("API Key is required!");
    match &cli.command {
        Some(Commands::List {
            filter,
            show_description,
        }) => list_tasks(filter, show_description, api_key, cli.json)?,
        Some(Commands::Add { text }) => quick_add_item(text, api_key, cli.json)?,
        Some(Commands::Complete { id }) => close_task(id, api_key)?,
        &None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No subcommand was given",
            )
            .into());
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}{}", "error: ".red().bold(), e);
        std::process::exit(1)
    }
}
