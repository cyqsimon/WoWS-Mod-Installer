use std::{fmt, io, path::PathBuf};

use fs_err as fs;
use itertools::Itertools;

const CONF_PATH_STR: &str = "pref.json";

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    JsonError(json::Error),
    ConfKeyMissing(String),
    CopyError(fs_extra::error::Error),
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}
impl From<json::Error> for Error {
    fn from(err: json::Error) -> Self {
        Self::JsonError(err)
    }
}
impl From<fs_extra::error::Error> for Error {
    fn from(err: fs_extra::error::Error) -> Self {
        // we only use `fs_extra` for copying step
        Self::CopyError(err)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        let repr = match self {
            IoError(err) => format!("IO Error: {}", err),
            JsonError(err) => format!("Json Error: {}", err),
            ConfKeyMissing(err) => format!("Key missing in {}: \"{}\"", CONF_PATH_STR, err),
            CopyError(err) => format!("Copy Error: {}", err),
        };
        write!(f, "{}", repr)
    }
}

fn main() {
    if let Err(err) = main_impl() {
        exit_with_msg(err.to_string(), 1);
    }
}

fn main_impl() -> Result<(), Error> {
    let conf = json::parse(&fs::read_to_string(CONF_PATH_STR)?)?;

    let game_dir: PathBuf = conf["gameDir"]
        .as_str()
        .ok_or(Error::ConfKeyMissing("gameDir".into()))?
        .into();
    let mods_dir: PathBuf = conf["modsDir"]
        .as_str()
        .ok_or(Error::ConfKeyMissing("modsDir".into()))?
        .into();

    // locate target
    let bin_dir = game_dir.join("bin"); // <game_dir>/bin
    let res_mods_dir = fs::read_dir(&bin_dir)?
        .collect::<Result<Vec<_>, _>>()? // `collect` fails if any subdirectory errors during read
        .into_iter()
        .sorted_by_key(|d| d.file_name()) // sort by version number
        .last() // largest version number is assumed to be newest
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("{:?} does not contain any version directory", bin_dir),
            )
        })?
        .path() // <game_dir>/bin/<newest_ver>
        .join("res_mods"); // <game_dir>/bin/<newest_ver>/res_mods

    // copy files
    println!("Copying all files in {:?} to {:?}.", mods_dir, res_mods_dir);
    let cp_opts = fs_extra::dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        copy_inside: true,
        content_only: true,
        ..Default::default()
    };
    fs_extra::dir::copy(&mods_dir, &res_mods_dir, &cp_opts)?;

    println!(
        "Successfully copied all files from {:?} to {:?}.",
        mods_dir, res_mods_dir
    );

    Ok(())
}

fn exit_with_msg(msg: impl AsRef<str>, code: i32) -> ! {
    println!("{}", msg.as_ref());

    println!("Press any key to continue...");
    // block until a keyboard event is read
    loop {
        let evn = crossterm::event::read().unwrap(); // always returns Ok
        if let crossterm::event::Event::Key(_) = evn {
            break;
        };
    }
    std::process::exit(code);
}
