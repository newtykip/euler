use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use regex::Regex;
use scraper::{Html, Selector};
use std::io::BufReader;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, Write},
    path::Path,
};

#[derive(Parser)]
#[clap(about, author, version)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Handles the initialisation of a new Project Euler Problem
    New,
}

#[tokio::main]
async fn new() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let code_path = base_dir.join("src").join("bin");

    let problem_number = requestty::prompt_one(
        requestty::Question::int("problemNumber")
            .message("Which problem would you like to solve?")
            .validate(|n, _| {
                // All numbers must be positive
                let mut pass = n > 0;

                // Ensure that the problem has not already got a file associated with it
                let files = fs::read_dir(&code_path).unwrap();

                for file in files {
                    let file_number = file
                        .unwrap()
                        .file_name()
                        .to_str()
                        .unwrap()
                        .split('.')
                        .collect::<Vec<&str>>()[0]
                        .parse::<i64>()
                        .unwrap();

                    if n == file_number && pass {
                        pass = false;
                    }
                }

                if pass {
                    Ok(())
                } else {
                    Err("Please ensure that your input is valid!".to_owned())
                }
            })
            .build(),
    )
    .unwrap()
    .as_int()
    .unwrap();

    // todo: thoughts documents (?)

    // Fetch the problem information
    let body = reqwest::get(format!("https://projecteuler.net/problem={problem_number}"))
        .await?
        .text()
        .await?;

    let document = Html::parse_document(body.as_str());
    let title_selector = Selector::parse("h2")?;
    let content_selector = Selector::parse(".problem_content")?;
    let html_tag_regex = Regex::new(r"<[^>]*>").unwrap();

    let mut title = document
        .select(&title_selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<&str>>()
        .join("");

    let mut problem = html_tag_regex
        .replace_all(
            document
                .select(&content_selector)
                .next()
                .unwrap()
                .inner_html()
                .as_str(),
            " ",
        )
        .to_string()
        .replace("$$", " ");

    let file_body = format!(
        "/*
Problem {} - {}

{}
*/

fn main() {{
    println!(\"Hello World!\");
}}",
        problem_number,
        html_escape::decode_html_entities(&mut title).to_string(),
        html_escape::decode_html_entities(&mut problem)
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| s != &"")
            .collect::<Vec<&str>>()
            .join("\n")
    );

    // Create the file
    let mut file = File::create(code_path.join(format!("{problem_number}.rs"))).unwrap();
    file.write(file_body.as_bytes()).unwrap();
    drop(file);

    // Read the contents of the readme for editing
    let readme_path = base_dir.join("readme.md");

    let mut readme_file = OpenOptions::new().read(true).open(&readme_path).unwrap();

    let mut readme_content = BufReader::new(&readme_file)
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<String>>();

    drop(readme_file);

    // Mark the problem as done on the readme
    let readme_regex = Regex::new(format!(" {problem_number} - (.*)").as_str()).unwrap();

    for i in 0..readme_content.len() {
        let line = readme_content[i].as_str();

        if readme_regex.is_match(line) {
            let matched = readme_regex.captures(line).unwrap();
            readme_content[i] = format!(
                "- [x] {problem_number} - [{}](src/bin/{problem_number}.rs)",
                &matched[1].trim()
            );
        }
    }

    // Update the summary statistics on the readme
    let mut readme_string = readme_content.join("\n");

    let completed_regex = Regex::new("<!-- completed -->([0-9]+)<!-- completed -->").unwrap();

    let new_completed = completed_regex.captures(readme_string.as_str()).unwrap()[1]
        .parse::<u8>()
        .unwrap()
        + 1;

    readme_string = completed_regex
        .replace(
            readme_string.as_str(),
            format!("<!-- completed -->{new_completed}<!-- completed -->"),
        )
        .to_string();

    // Write the new content to the readme
    readme_file = OpenOptions::new().write(true).open(&readme_path).unwrap();

    readme_file.write(readme_string.as_bytes()).unwrap();

    drop(readme_file);

    // Announce completion!
    println!(
        "{}",
        "File successfully created! Good luck (:".green().bold()
    );

    Ok(())
}

fn main() {
    let value = Value::parse();

    match value.command {
        Commands::New => new().unwrap(),
    }
}
