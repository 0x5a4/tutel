use crate::app;
use anyhow::{bail, Context, Result};
use directories::BaseDirs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::Lines;

/// Search the tutelnav database for a project with the specified name and return its path
pub fn query_nav(name: &str) -> Result<PathBuf> {
    let cache_path = if let Some(dirs) = BaseDirs::new() {
        dirs.data_dir().join("tutelnav")
    } else {
        bail!("unable to retrieve home dir");
    };

    if !cache_path.exists() {
        bail!("no projects added yet");
    }

    let mut cache_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(cache_path)
        .context("unable to open tutelnav file")?;

    let mut cache_file_content = String::new();
    cache_file.read_to_string(&mut cache_file_content)?;

    let entries = cache_file_content.lines();
    if let Some((index, line)) = find_entry(name, entries) {
        let parts: Vec<&str> = line.split(' ').collect();

        if parts.len() < 2 {
            bail!("nav entry is not properly formatted(line {})", index);
        }

        let path = parts[1].parse::<PathBuf>().context(format!(
            "nav entry is not properly formatted(line {})",
            index
        ))?;
        let existence_check = path.join(app::PROJECT_FILE_NAME);

        if !existence_check.exists() {
            delete_entry(index, cache_file_content.as_str(), &mut cache_file)?;
            bail!("no project found with name {}", name);
        }

        return Ok(path);
    }

    bail!("no project found with name {}", name);
}

/// Tries to find the line starting with the given name. 
/// If found, returns the index of the line and the line itself
fn find_entry(name: &str, lines: Lines) -> Option<(usize, String)> {
    for (i, e) in lines.enumerate() {
        let mut tmp = String::from(name);
        tmp.push(' ');
        if e.starts_with(tmp.as_str()) {
            return Some((i, String::from(e)));
        }
    }

    None
}


/// deletes the given line from content and changes to file
fn delete_entry(line_number: usize, content: &str, file: &mut File) -> Result<()> {
    let lines: Vec<&str> = content.lines().collect();
    file.set_len(0)?;
    for (i, s) in lines.iter().enumerate() {
        if i == line_number {
            continue;
        }

        write!(file, "{}\n", s)?;
    }

    Ok(())
}

/// Add a project with name and path to the tutel nav database
/// Each entry has its own line and is in the format "{name} {path}"
pub fn add_to_nav(name: &str, path: &Path) -> Result<()> {
    let cache_path = if let Some(dirs) = BaseDirs::new() {
        dirs.data_dir().join("tutelnav")
    } else {
        bail!("unable to retrieve home dir");
    };

    let mut cache_file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(cache_path)
        .context("unable to open tutelnav file")?;
    
    let mut contents = String::new();
    cache_file.read_to_string(&mut contents).context("unable to read tutelnav file")?;

    if let None = find_entry(name, contents.lines()) {
        cache_file.write_fmt(format_args!("{} {}\n", name, path.display()))?;
    } else {
        bail!("project with name {} already exists", name);
    }

    Ok(())
}

const FISH: &str = include_str!("shells/fish.fish");
const BASH: &str = include_str!("shells/bash.sh");

pub fn init(shell: &str) -> Result<()> {
    match shell {
        "fish" => println!("{}", FISH),
        "bash" => println!("{}", BASH),
        &_ => bail!("no such shell"),
    }

    Ok(())
}
