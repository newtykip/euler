use std::path::{Path, PathBuf};

fn fetch_dir() -> &'static Path {
    return Path::new(env!("CARGO_MANIFEST_DIR"));
}

pub fn code_path() -> PathBuf {
    return fetch_dir().join("src").join("bin");
}

pub fn resources_path() -> PathBuf {
    return fetch_dir().join("resources");
}

pub fn readme_path() -> PathBuf {
    return fetch_dir().join("readme.md");
}

pub fn run_path() -> PathBuf {
    return fetch_dir().join("src").join("commands").join("run.rs");
}

pub fn problem_number(problem: Option<u8>, code_path: &PathBuf, prompt: &str) -> u8 {
    if let Some(n) = problem {
        return n;
    } else {
        return requestty::prompt_one(
            requestty::Question::int("problemNumber")
                .message(prompt)
                .validate(|n, _| {
                    // All numbers must be positive
                    let mut pass = false;

                    // Ensure that the problem has not already got a file associated with it
                    let files = std::fs::read_dir(code_path).unwrap();

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

                        if n == file_number {
                            pass = true;
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
        .unwrap() as u8;
    }
}
