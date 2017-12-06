
use util::*;

use std::fs;
use std::sync::Arc;
use std::sync::RwLock;
use serenity::prelude::*;
use serenity::model::*;

pub struct Handler;
impl EventHandler for Handler {

    fn on_message(&self, _: Context, msg: Message) {
        println!("Message received:
        {author:>width$}
        {message:>width$}\n"
        , author=msg.author, message=msg.content, width=4
        );
    }
    
    ///The bot has successfully connected.
    fn on_ready(&self, _: Context, ready: Ready) {
        let user = &ready.user;
        println!("{} is connected!", user.name);
    }
    
    //Created or was added to a guild
    fn on_guild_create(&self, _: Context, guild: Guild, _is_new: bool) {
        println!("Guild added: {} - {}", guild.name, guild.id);
        confirm_server(guild.id);
    }
    
    // guild was deleted
    fn on_guild_delete(&self, _: Context, guild: PartialGuild, _: Option<Arc<RwLock<Guild>>>) { 
        println!("Guild deleted: {} - {}", guild.name, guild.id);
        let path = server_path(guild.id);
        if let Err(why) = fs::remove_dir(&path) {
            println!("Failed to delete path '{}': {}", path, why)
        }
    }
}
