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
