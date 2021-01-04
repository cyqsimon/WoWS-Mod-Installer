use std::{fs, str};

const CONF_PATH_STR: &str = "./pref.json";

fn main() {
    // parse config
    let conf_bytes = fs::read(CONF_PATH_STR).unwrap_or_else(|_| {
        exit_with_msg(
            &format!("Cannot read {}. Maybe it does not exist?", CONF_PATH_STR),
            1,
        )
    });
    let conf_json = str::from_utf8(&conf_bytes).unwrap_or_else(|_| {
        exit_with_msg(
            &format!(
                "{} is not a valid UTF8 text file. Maybe it is corrupted?",
                CONF_PATH_STR
            ),
            1,
        )
    });
    let conf_obj = json::parse(conf_json).unwrap_or_else(|_| {
        exit_with_msg(
            &format!(
                "{} failed to parse as JSON. Maybe it contains syntax errors?",
                CONF_PATH_STR
            ),
            1,
        )
    });
    let game_dir_jv = &conf_obj["gameDir"];
    let mods_dir_jv = &conf_obj["modsDir"];
    if game_dir_jv.is_null() || mods_dir_jv.is_null() {
        exit_with_msg(
            &format!("'gameDir' and 'modsDir' must be defined in JSON."),
            1,
        );
    }
    let game_dir = &format!("{}", game_dir_jv);
    let mods_dir = &format!("{}", mods_dir_jv);

    // locate target
    let bin_dir = &format!("{}/bin", game_dir);
    let vers_dirs: Vec<_> = fs::read_dir(bin_dir)
        .unwrap_or_else(|_| {
            exit_with_msg(
                &format!(
                    "Cannot read {}. Maybe it does not exist or is not a directory?",
                    bin_dir
                ),
                1,
            )
        })
        .collect();
    if vers_dirs.iter().any(|r| r.is_err()) {
        exit_with_msg(
            &format!("Error while reading the content of directory {}.", bin_dir),
            1,
        );
    }
    let mut vers_dirs_names: Vec<_> = vers_dirs
        .into_iter()
        .map(|r| r.unwrap().file_name())
        .collect();
    vers_dirs_names.sort();
    let newest_ver_dir_name = vers_dirs_names
        .last()
        .unwrap_or_else(|| {
            exit_with_msg(
                &format!("{} does not contain any version directory.", bin_dir),
                1,
            )
        })
        .to_str()
        .unwrap();
    let target_dir = &format!("{}/{}/res_mods", bin_dir, newest_ver_dir_name);

    // copy files
    println!("Copying all files in {} to {}.", mods_dir, target_dir);
    let cp_opts = fs_extra::dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        copy_inside: true,
        content_only: true,
        ..Default::default()
    };
    let cp_res = fs_extra::dir::copy(&format!("{}", mods_dir), target_dir, &cp_opts);
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
