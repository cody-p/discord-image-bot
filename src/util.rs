use std::fs;
use std::path::Path;
use serenity::model::*;
use global_data::*;

pub fn confirm_dir(path: &str) {
    if !Path::new(path).exists() {
        println!("Directory '{}' didn't exist, creating.", path);
        if let Err(why) = fs::create_dir(path) {
            panic!("Fatal error: {}", why);
        }
    }
}

pub fn server_path(path: GuildId) -> String {
    return format!("{}/{}",SERVER_PATH, path);
}

pub fn confirm_server(path: GuildId) {
    confirm_dir(&server_path(path));
}

pub fn is_owner(id: UserId) -> bool {
    return format!("{}", id) == *OWNER;
}
