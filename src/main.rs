use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use flate2::{write::GzEncoder, Compression};
use serde::Deserialize;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Deserialize)]
struct Config {
    archives: Option<Vec<Archive>>,
}

#[derive(Debug, Deserialize)]
struct Archive {
    name: String,
    input: String,
    output: String,
}

fn main() -> Result<()> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "arkaive") {
        let config_path = proj_dirs.config_dir().join("config.toml");
        let config = parse_config(&config_path)?;
        parse_archives(config)
            .ok_or_else(|| anyhow!("no paths were configured"))?
            .try_for_each(|a| compress_archive(a))?;
    }

    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn compress(input: PathBuf, output: PathBuf) -> Result<()> {
    let current_dir = env::current_dir()?;

    let file = File::create(&output)
        .with_context(|| format!("failed creating file at {}", &output.display()))?;
    let encode = GzEncoder::new(file, Compression::default());
    let mut tar = tar::Builder::new(encode);

    env::set_current_dir(input.parent().unwrap())?;

    for entry in WalkDir::new(input.file_name().unwrap())
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
    {
        println!("{}", entry.path().display());

        tar.append_path(entry.path())?;
    }

    tar.finish()?;

    env::set_current_dir(current_dir)?;

    Ok(())
}

fn compress_archive(archive: Archive) -> Result<()> {
    let input = expand_path(Path::new(&archive.input));
    let output = expand_path(
        &Path::new(&archive.output)
            .join(archive.name)
            .with_extension("tar.gz"),
    );

    compress(input, output)
}

fn expand_path(path: &Path) -> PathBuf {
    PathBuf::from(shellexpand::tilde(&path.to_str().unwrap()).as_ref())
}

fn parse_config(path: &Path) -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("config file doesn't exist: {}", path.display()))?;

    toml::from_str(contents.as_str()).with_context(|| "failed to parse config file")
}

fn parse_archives(config: Config) -> Option<impl Iterator<Item = Archive>> {
    match config.archives {
        Some(archives) => Some(archives.into_iter()),
        None => None,
    }
}
