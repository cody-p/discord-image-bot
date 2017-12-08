use std::env;

pub const SERVER_PATH: &'static str = "./servers";
pub const SETTINGS_PATH: &'static str = "./settings";
pub const STATUS_CHANNEL: &'static str = "./settings/status_channel";

lazy_static! {
    pub static ref OWNER: String = env::var("DISCORD_NEWSBOT_OWNER")
        .expect("Expected an owner ID in the environment");
}

lazy_static! {
    pub static ref TOKEN: String = env::var("DISCORD_NEWSBOT_TOKEN")
            .expect("Expected a token in the environment");
}
