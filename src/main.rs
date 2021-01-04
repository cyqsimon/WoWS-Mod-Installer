use std::{fs, str};

use itertools::Itertools;
use json::JsonValue;

const CONF_PATH_STR: &str = "./pref.json";

fn main() {
    // read and parse conf
    let conf = read_conf(CONF_PATH_STR).unwrap_or_else(|e_str| exit_with_msg(&e_str, 1));

    // get values from conf
    let game_dir = conf["gameDir"]
        .as_str()
        .unwrap_or_else(|| exit_with_msg("\"gameDir\" is not a string.", 1));
    let mods_dir = conf["modsDir"]
        .as_str()
        .unwrap_or_else(|| exit_with_msg("\"modsDir\" is not a string.", 1));

    // locate target
    let target_dir = locate_target_dir(game_dir).unwrap_or_else(|e_str| exit_with_msg(&e_str, 1));

    // copy files
    println!("Copying all files in {} to {}.", mods_dir, target_dir);
    let cp_opts = fs_extra::dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        copy_inside: true,
        content_only: true,
        ..Default::default()
    };
    let cp_res = fs_extra::dir::copy(mods_dir, &target_dir, &cp_opts);
    match cp_res {
        Ok(_) => exit_with_msg(
            &format!(
                "Successfully copied all files from {} to {}.",
                mods_dir, target_dir
            ),
            0,
        ),
        Err(_) => exit_with_msg(
            &format!(
                "Copying failed from {} to {} failed. Maybe you do not have enough permission?",
                mods_dir, target_dir
            ),
            1,
        ),
    };
}

fn read_conf(conf_path: &str) -> Result<JsonValue, String> {
    // read file into bytes
    let conf_bytes = fs::read(conf_path)
        .map_err(|e| format!("Cannot read {}.\n{}", conf_path, e.to_string()))?;

    // parse bytes into str
    let conf_json = str::from_utf8(&conf_bytes).map_err(|e| {
        format!(
            "{} is not a valid UTF8 text file.\n{}",
            conf_path,
            e.to_string()
        )
    })?;

    // parse str into json obj
    let conf_obj = json::parse(conf_json)
        .map_err(|e| format!("Cannot parse {} as JSON.\n{}", conf_path, e.to_string()))?;

    return Ok(conf_obj);
}

fn locate_target_dir(game_dir_path: &str) -> Result<String, String> {
    // <gameDir>/bin
    let bin_dir_path = format!("{}/bin", game_dir_path);

    // get target dir
    let target_dir_name = fs::read_dir(&bin_dir_path)
        .map_err(|e| format!("Cannot read {}.\n{}", bin_dir_path, e.to_string()))? // short-circuit return Err
        .collect::<Result<Vec<_>, _>>() // collect() magic; Iter<Result<a, b>> -> Result<Vec<a>, b>
        .map_err(|e| {
            format!(
                "Cannot read a subdirectory of {}.\n{}",
                bin_dir_path,
                e.to_string()
            )
        })? // short-circuit return Err
        .into_iter()
        .sorted_by_key(|d| d.file_name()) // sort by version number
        .last() // take largest version number (newest)
        .ok_or_else(|| format!("{} does not contain any version directory.", bin_dir_path))? // short-circuit return Err
        .file_name()
        .into_string()
        .map_err(|s| format!("{:?} is an unsupported dir name.", s))?;

    // <gameDir>/bin/<newestVer>/res_mods
    return Ok(format!("{}/{}/res_mods", bin_dir_path, target_dir_name));
}

fn prompt_exit(code: i32) -> ! {
    println!("Press any key to continue...");
    // block until a keyboard event is read
    loop {
        let evn = crossterm::event::read().unwrap(); // always returns Ok
        if let crossterm::event::Event::Key(_) = evn {
            break;
        };
    }
    std::process::exit(code)
}

fn exit_with_msg(msg: &str, code: i32) -> ! {
    println!("{}", msg);
    prompt_exit(code)
}
