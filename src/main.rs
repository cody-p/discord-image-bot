#[macro_use] extern crate serenity;
#[macro_use] extern crate lazy_static;

mod global_data;
mod util;
mod events;
mod commands;

use util::*;
use events::*;
use commands::*;
use global_data::*;

use serenity::prelude::*;
use serenity::framework::standard::StandardFramework;



fn main() {
    println!("Owner ID = {}", *OWNER);
    confirm_dir(SERVER_PATH);
    
    let mut client = Client::new(&TOKEN, Handler);
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .on("submit", submit)
        .on("set-output", set_output)
        );
    println!("Starting...");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
