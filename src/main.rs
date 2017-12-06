#[macro_use] extern crate serenity;

mod util;
mod events;

use util::*;
use events::*;

use std::env;
use serenity::prelude::*;
use serenity::framework::standard::StandardFramework;

fn init() {
    confirm_dir("./servers");
}

fn main() {
    init();
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler);    
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .on("ping", ping)
        .on("test", test));
    println!("Starting...");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});

command!(test(_context, message) {
    let _ = message.reply("F");
});
