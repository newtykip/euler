use euler::{Problem, Result, BASE_DIR, README, SOLUTIONS};
use num_to_words::integer_to_en_us as num_to_words;
use owo_colors::OwoColorize;
use regex::Regex;
use std::fs;
use std::io::BufReader;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, Write},
};

pub async fn execute(number: Option<u8>) -> Result<()> {
    let problem = Problem::new(number).await?;

    // create a file for the problem
    let mut file = File::create(SOLUTIONS.join(format!("{}.rs", problem.number)))?;
    file.write(problem.file_body().as_bytes())?;
    drop(file);

    // open readme
    let mut readme_file = OpenOptions::new().read(true).open(README.clone())?;

    let readme_content = {
        // read
        let mut content = BufReader::new(&readme_file)
            .lines()
            .map(|s| s.unwrap())
            .collect::<Vec<String>>()
            .join("\n");

        drop(readme_file);

        // mark problem as done on readme
        content = content.replace(
            &format!("\n- [ ] {}", problem.number),
            &format!("\n- [x] {}", problem.number),
        );

        // update completed count
        let completion_regex = Regex::new(r"(\d+) out of")?;
        let problem_count = fs::read_dir(SOLUTIONS.clone())?.count();

        content = completion_regex
            .replace(&content, format!("{} out of", problem_count))
            .to_string();

        content
    };

    // write the new content to the readme
    readme_file = OpenOptions::new().write(true).open(README.clone())?;
    readme_file.write(readme_content.as_bytes())?;
    drop(readme_file);

    // add the problem to the run command
    let mut run_file = OpenOptions::new()
        .read(true)
        .open(BASE_DIR.join("src").join("commands").join("run.rs"))?;

    let run_content = {
        let mut content: String = BufReader::new(&run_file)
            .lines()
            .map(|s| s.unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        drop(run_file);

        // add new mod
        let mod_regex = Regex::new(r#"#\[path = ".+"\]\nmod [A-Za-z_]+;"#)?;
        let final_mod = mod_regex.find_iter(&content).last().unwrap();
        let word_number = num_to_words(problem.number as i64)?.replace(" ", "_");

        content.insert_str(
            final_mod.end(),
            &format!(
                r#"
#[path = "../bin/{}.rs"]
mod {};"#,
                problem.number, word_number
            ),
        );

        // add to match
        let match_regex = Regex::new(r"\d+ => .+\(\),")?;
        let final_branch = match_regex.find_iter(&content).last().unwrap();

        content.insert_str(
            final_branch.end(),
            &format!(
                r#"
        {} => {}::main(),"#,
                problem.number, word_number
            ),
        );

        content
    };

    // Write the new content to the run file
    run_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(BASE_DIR.join("src").join("commands").join("run.rs"))?;
    run_file.write(run_content.as_bytes())?;
    drop(run_file);

    // Announce completion!
    println!(
        "{}",
        "File successfully created! Good luck (:".green().bold()
    );

    Ok(())
}
