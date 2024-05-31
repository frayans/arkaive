use std::{
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use flate2::{write::GzEncoder, Compression};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    archives: Option<Vec<ArchiveConf>>,
}

#[derive(Debug, Deserialize)]
struct ArchiveConf {
    name: String,
    input: String,
    output: String,
}

struct ArchivePath {
    pub input: PathBuf,
    pub output: PathBuf,
}

fn main() -> Result<()> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "arkaive") {
        let config_path = proj_dirs.config_dir().join("config.toml");
        parse_config(&config_path.as_path())?
            .map(|archive| expand_paths(archive))
            .try_for_each(|archive| compress_tar_gz(archive))?;
    }

    Ok(())
}

fn compress_tar_gz(archive_path: ArchivePath) -> Result<()> {
    let file = File::create(&archive_path.output)
        .with_context(|| format!("failed creating file at {}", archive_path.output.display()))?;
    let encode = GzEncoder::new(file, Compression::default());
    let mut tar = tar::Builder::new(encode);
    tar.append_dir_all(".", &archive_path.input)
        .with_context(|| {
            format!(
                "failed creating archive at {}",
                archive_path.input.display()
            )
        })?;
    tar.finish()?;
    Ok(())
}

fn expand_paths(archive: ArchiveConf) -> ArchivePath {
    let input = PathBuf::from(shellexpand::tilde(&archive.input).into_owned());
    let output = Path::new(shellexpand::tilde(&archive.output).as_ref())
        .join("placeholder")
        .with_file_name(archive.name)
        .with_extension("tar.gz");

    ArchivePath { input, output }
}

fn parse_config(path: &Path) -> Result<impl Iterator<Item = ArchiveConf>> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config from {}", path.display()))?;
    let parsed: Config = toml::from_str(contents.as_str())?;
    if let Some(archives) = parsed.archives {
        Ok(archives.into_iter())
    } else {
        Err(anyhow!("failed parsing file from {}", path.display()))
    }
}
