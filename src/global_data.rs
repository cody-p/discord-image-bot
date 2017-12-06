use std::env;

pub const SERVER_PATH: &'static str = "./servers";

lazy_static! {
    pub static ref OWNER: String = env::var("DISCORD_OWNER")
        .expect("Expected an owner ID in the environment");
}

lazy_static! {
    pub static ref TOKEN: String = env::var("DISCORD_TOKEN")
            .expect("Expected a token in the environment");
}
