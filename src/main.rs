use std::{
    borrow::Cow,
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
};

use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use flate2::{bufread::GzEncoder, Compression};

use crate::config::{parse_archives, parse_config, Archive};

mod config;

fn main() -> Result<()> {
    let proj_dir = match ProjectDirs::from("", "", "arkaive") {
        Some(dir) => dir,
        None => todo!(),
    };

    let config_dir = proj_dir.config_dir();
    let config_file_path = config_dir.join("config.toml");

    let config = parse_config(&config_file_path)?;
    let archives = match parse_archives(&config) {
        Some(a) => a,
        None => todo!(),
    };

    for archive in archives {
        // iterator already returns &Archive
        let expanded = expand_archive(archive)?;

        let input = Path::new(expanded.get_input());
        let output = Path::new(expanded.get_output())
            // with this the path becomes /foo/bar/file.tar.gz
            // instead of /foo/bar.tar.gz which is the
            // intended behaviour
            .join("placeholder")
            .with_file_name(expanded.get_name())
            .with_extension("tar.gz");

        let compressed = compress_dir_to_vec(input, Compression::default())?;

        let mut file = File::create(output)?;
        file.write_all(&compressed)?;
    }

    Ok(())
}

fn expand_path(path: &Path) -> Result<Cow<str>> {
    let path = path.to_str().ok_or(anyhow!(
        "{} contains non-Unicode characters",
        path.display()
    ))?;
    let expanded = shellexpand::full(path)?;
    Ok(expanded)
}

fn expand_archive(archive: &Archive) -> Result<Archive> {
    let name = archive.get_name();
    let input = Path::new(archive.get_input());
    let output = Path::new(archive.get_output());

    let expanded_input = expand_path(input)?;
    let expanded_output = expand_path(output)?;

    Ok(Archive::new(
        name.to_string(),
        expanded_input.into_owned(),
        expanded_output.into_owned(),
    ))
}

fn compress_dir_to_vec(path: &Path, level: Compression) -> Result<Vec<u8>> {
    let tar = make_tar_from_dir(path)?;
    let compressed = compress_data(&tar, level)?;
    Ok(compressed)
}

fn make_tar_from_dir(path: &Path) -> Result<Vec<u8>> {
    let buf = Vec::new();
    let mut tar = tar::Builder::new(buf);
    tar.append_dir_all(".", path)?;
    Ok(tar.into_inner()?)
}

fn compress_data(data: &[u8], level: Compression) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let read_buf = BufReader::new(data);
    let mut encoder = GzEncoder::new(read_buf, level);
    encoder.read_to_end(&mut buf)?;
    Ok(buf)
}
