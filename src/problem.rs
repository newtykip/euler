use crate::{Result, SILENT_PANIC, SOLUTIONS};
use regex::Regex;
use scraper::{Html, Selector};

pub struct Problem {
    pub number: u8,
    title: String,
    content: String,
}

impl Problem {
    pub fn prompt_number(prompt: &str, want_solved: bool) -> Result<u8> {
        unsafe {
            SILENT_PANIC = true;
        }

        let number = requestty::prompt_one(
            requestty::Question::int("problemNumber")
                .message(prompt)
                .validate(|n, _| {
                    // ensure the number is in bound
                    if n <= 0 {
                        Err("Please enter a number greater than 0".to_string())
                    } else if n > 100 {
                        Err("Please enter a number less than or equal to 100".to_string())
                    }
                    // ensure that the problem has not already got a file associated with it
                    else {
                        let has_file = std::fs::read_dir(SOLUTIONS.clone()).unwrap().any(|x| {
                            x.unwrap()
                                .file_name()
                                .to_str()
                                .unwrap()
                                .split(".")
                                .next()
                                .unwrap()
                                .parse::<i64>()
                                .unwrap()
                                == n
                        });

                        if !want_solved && has_file {
                            Err("This problem has already been solved".to_string())
                        } else if want_solved && !has_file {
                            Err("This problem has not been solved yet".to_string())
                        } else {
                            Ok(())
                        }
                    }
                })
                .build(),
        )?
        .as_int()
        .unwrap() as u8;

        unsafe {
            SILENT_PANIC = false;
        }

        Ok(number)
    }

    pub async fn new(number: Option<u8>) -> Result<Self> {
        let number = number
            .unwrap_or_else(|| Self::prompt_number("Please select a problem:", false).unwrap());

        let body = reqwest::get(format!("https://projecteuler.net/minimal={number}"))
            .await?
            .text()
            .await?;

        let document = Html::parse_document(body.as_str());
        let title_selector = Selector::parse("h2")?;
        let content_selector = Selector::parse(".problem_content")?;
        let html_tag_regex = Regex::new(r"<[^>]*>")?;

        let title = document
            .select(&title_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .join("");

        let problem = html_tag_regex
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

        Ok(Self {
            number,
            title: title.trim().to_string(),
            content: problem.trim().to_string(),
        })
    }

    pub fn file_body(&self) -> String {
        format!(
            "/*
Problem {} - {}

{}
*/
    
pub fn main() {{
    println!(\"Hello World!\");
}}",
            self.number,
            html_escape::decode_html_entities(&self.title).to_string(),
            html_escape::decode_html_entities(&self.content)
                .split("\n")
                .map(|s| s.trim())
                .filter(|s| s != &"")
                .collect::<Vec<&str>>()
                .join("\n")
        )
    }
}
