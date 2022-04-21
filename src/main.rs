use std::{fmt, io, path::PathBuf};

use fs_err as fs;
use tap::TapFallible;

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
    match main_impl() {
        Ok(_) => prompt_exit::<&str>(None, 0),
        Err(err) => prompt_exit(Some(err.to_string()), 1),
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

    // locate targets
    let res_mods_dirs = fs::read_dir(game_dir.join("bin"))?
        .filter_map(|res| res.tap_err(|e| println!("Failed to read a directory: {:?}", e)).ok())
        .map(
            |d| {
                d.path() // <game_dir>/bin/<ver>
                    .join("res_mods")
            }, // <game_dir>/bin/<ver>/res_mods
        )
        .collect::<Vec<_>>();

    // copy files
    println!("Copying all files in {:?} to {:?}.", mods_dir, res_mods_dirs);
    let cp_opts = fs_extra::dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        copy_inside: true,
        content_only: true,
        ..Default::default()
    };
    for target in res_mods_dirs.iter() {
        // we copy into `res_mods` of all versions (as opposed to just the newest version)
        // because there might be pre-downloads of upcoming updates.
        fs_extra::dir::copy(&mods_dir, target, &cp_opts)?;
    }

    println!(
        "Successfully copied all files from {:?} to {:?}.",
        mods_dir, res_mods_dirs
    );

    Ok(())
}

fn prompt_exit<S>(msg: Option<S>, code: i32) -> !
where
    S: AsRef<str>,
{
    if let Some(msg_inner) = msg {
        println!("{}", msg_inner.as_ref());
    }
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
