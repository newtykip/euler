mod problem;

use once_cell::sync::Lazy;
pub use problem::Problem;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// paths
pub const BASE_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
pub const SOLUTIONS: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("src").join("bin"));
pub const RESOURCES: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("resources"));
pub const README: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("readme.md"));

// panic channel
pub static mut SILENT_PANIC: bool = false;
