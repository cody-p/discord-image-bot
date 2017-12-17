use std::fs;
use std::path::Path;
use serenity::model::id::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use global_data::*;
use serenity::CACHE;
use std::io::Read;
use std::result::Result;
use std::sync::Arc;

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

pub fn parse_channel_from_file(path: &str) -> Result<Arc<RwLock<GuildChannel>>, String> {
    match fs::File::open(path) {
        Err(why) => {
            return Err(format!("Error opening channel file: {}", why));
        },
        Ok(mut file) => {
            let mut buffer = String::new();
            if let Err(why) = file.read_to_string(&mut buffer) {
                return Err(format!("Failed to read channel file to string: {}", why));
            } else {
                if let Ok(channel_int) = buffer.parse::<u64>() {
                    let channel_id = ChannelId(channel_int);
                    if let Some(channel) = CACHE.read().guild_channel(channel_id) {
                        return Ok(channel);
                    } else {
                        return Err(format!("Could not find specified channel in cache."));
                    }
                } else {
                    return Err(format!("Could not parse integer '{}' into a channel string.", buffer));
                }
            }
        }
    }
}
pub fn send_to_status_channel(msg: &str) {
    match parse_channel_from_file(STATUS_CHANNEL) {
        Ok(channel) => {
            let _ = channel.read().send_message(|m| m
                .content(msg));
        },
        Err(_why) => {
            println!("Not sending status messages. The following error occured: {}", _why);
        }
    }
}

pub fn status_mirror(msg: &str) {
	println!("{}",msg);
	send_to_status_channel(msg);
}



