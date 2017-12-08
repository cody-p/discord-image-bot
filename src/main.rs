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

use serenity::model::*;
use serenity::prelude::*;
use serenity::framework::standard::{Args, Command, StandardFramework, help_commands};
use std::sync::Arc;

fn main() {
    println!("Owner ID = {}", *OWNER);
    confirm_dir(SERVER_PATH);
    confirm_dir(SETTINGS_PATH);
    
    let mut client = Client::new(&TOKEN, Handler);
    client.with_framework(StandardFramework::new()
        .configure(|c| 
            c.prefix("~")
            .on_mention(true)) // set the bot's prefix to "~"
        .group("Standard", |g| g
            .command("help", |c| c.exec_help(help_commands::plain))
            .command("about", |c| c
                .desc("Information about this bot.")
                .exec_str("A bot for broadcasting messages to several servers from a centralized location.")
            )
        )
        .group("Administrative", |g| g
            .command("set-output", |c| c
                .desc("Sets the channel to receive messages in.")
                .required_permissions(Permissions::MANAGE_CHANNELS)
                .exec(set_output)
            )
        )
        .group("Standard", |g| g
            .command("submit", |c| c
                .desc("Submit a message to be broadcast.")
                .check(owner_check)
                .exec(submit)
            )
            .command("set-status", |c| c
                .check(owner_check)
                .exec(set_status)
            )
            .command("die", |c| c
                .check(owner_check)
                .exec(die)
            )
        )
    );
    println!("Starting...");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

/// A more comprehensive wrapper of is_owner.
fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: &Arc<Command>) -> bool {
    return is_owner(msg.author.id);
}
