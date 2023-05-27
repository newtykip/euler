use owo_colors::OwoColorize;
use regex::Regex;
use scraper::{Html, Selector};
use std::io::BufReader;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, Write},
};

#[tokio::main]
pub async fn execute(problem: Option<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let code_path = euler::code_path();
    let problem_number = euler::problem_number(
        problem,
        &code_path,
        "Which problem would you like to solve?",
    );

    // Fetch the problem information
    let body = reqwest::get(format!("https://projecteuler.net/problem={problem_number}"))
        .await?
        .text()
        .await?;

    let document = Html::parse_document(body.as_str());
    let title_selector = Selector::parse("h2")?;
    let content_selector = Selector::parse(".problem_content")?;
    let html_tag_regex = Regex::new(r"<[^>]*>")?;

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

pub fn main() {{
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
    let mut file = File::create(code_path.join(format!("{problem_number}.rs")))?;
    file.write(file_body.as_bytes())?;

    drop(file);

    // Read the contents of the readme for editing
    let readme_path = euler::readme_path();
    let mut readme_file = OpenOptions::new().read(true).open(&readme_path)?;

    let mut readme_content = BufReader::new(&readme_file)
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<String>>();

    drop(readme_file);

    // Mark the problem as done on the readme
    let readme_regex = Regex::new(format!(" {problem_number} - (.*)").as_str())?;

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

    let completed_regex = Regex::new("<!-- completed -->([0-9]+)<!-- completed -->")?;

    let new_completed =
        completed_regex.captures(readme_string.as_str()).unwrap()[1].parse::<u8>()? + 1;

    readme_string = completed_regex
        .replace(
            readme_string.as_str(),
            format!("<!-- completed -->{new_completed}<!-- completed -->"),
        )
        .to_string();

    // Write the new content to the readme
    readme_file = OpenOptions::new().write(true).open(&readme_path)?;

    readme_file.write(readme_string.as_bytes())?;

    drop(readme_file);

    // Modify the run command
    let run_path = euler::run_path();
    let mut run_file = OpenOptions::new().read(true).open(&run_path)?;

    let mut run_content = BufReader::new(&run_file)
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<String>>();

    drop(run_file);

    // Import the problem's file
    let import_regex = Regex::new(r#"// #\[path = "\.\./bin/{x}\.rs"]"#.replace("{x}", problem_number.to_string().as_str()).as_str())?;

    for i in 0..run_content.len() {
        let line = run_content[i].as_str();

        if import_regex.is_match(line) {
            run_content[i] = line.replace("// ", "");
            run_content[i + 1] = run_content[i + 1].replace("// ", "");

            break;
        }
    }

    // Add the problem to the match statement
    let match_regex = Regex::new(
        r"// {x} =>"
            .replace("{x}", problem_number.to_string().as_str())
            .as_str(),
    )?;

    for i in 0..run_content.len() {
        let line = run_content[i].as_str();

        if match_regex.is_match(line) {
            run_content[i] = line.replace("// ", "");
            break;
        }
    }

    // Write the new content to the run file
    run_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&run_path)?;

    run_file.write(run_content.join("\n").as_bytes())?;

    drop(run_file);

    // Announce completion!
    println!(
        "{}",
        "File successfully created! Good luck (:".green().bold()
    );

    Ok(())
}
