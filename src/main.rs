use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::{fs, path::Path, io::Write};
use scraper::{Html, Selector};
use regex::Regex;

#[derive(Parser)]
#[clap(about, author, version)]
struct Value {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Handles the initialisation of a new Project Euler Problem
    New,
}

#[tokio::main]
async fn new() -> Result<(), Box<dyn std::error::Error>> {
    let code_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("bin");

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
            .build()
    ).unwrap().as_int().unwrap();

    // todo: thoughts documents (?)
    
    // Fetch the problem information
    let body = reqwest::get(format!("https://projecteuler.net/problem={problem_number}")).await?.text().await?;

    let document = Html::parse_document(Box::leak(body.into_boxed_str()));
    let title_selector = Selector::parse("h2")?;
    let content_selector = Selector::parse(".problem_content")?;
    let re = Regex::new(r"<[^>]*>").unwrap();

    let mut title = document
        .select(&title_selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<&str>>()
        .join("");

    let mut problem = re.replace_all(
        Box::leak(
            document
                .select(&content_selector)
                .next()
                .unwrap()
                .inner_html()
                .into_boxed_str()
        ), 
        " "
    ).to_string();

    let file_body = format!(
        "// Problem {} - {}

//{}

fn main() {{
    println!(\"Hello World!\");  
}}", 
        problem_number,
        html_escape::decode_html_entities(&mut title).to_string(),
        html_escape::decode_html_entities(&mut problem).split("\n").filter(|s| s != &"").collect::<Vec<&str>>().join("\n//")
    );

    // Create the file
    let mut file = fs::File::create(code_path.join(format!("{problem_number}.rs"))).unwrap();
    file.write(file_body.as_bytes()).unwrap();
    drop(file);

    // todo: update readme

    // Announce completion!
    println!("{}", "File successfully created! Good luck (:".green());

    Ok(())
}

// todo: runner

fn main() {
    let value = Value::parse();

    match value.command {
        Commands::New => new().unwrap(),
    }
}
