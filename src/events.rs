
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
        status_mirror(&format!("{} is connected!\nCurrently ver {}", user.name, "0.1.5"));
    }
    
    ///The bot has resumed its connection.
    fn on_resume(&self, _: Context, _: event::ResumedEvent) {
        status_mirror("Resume occured!");
    }
    
    //Created or was added to a guild
    fn on_guild_create(&self, _: Context, guild: Guild, _is_new: bool) {
        status_mirror(&format!("Guild added: {} - {}", guild.name, guild.id));
        confirm_server(guild.id);
    }
    
    // guild was deleted
    fn on_guild_delete(&self, _: Context, guild: PartialGuild, _: Option<Arc<RwLock<Guild>>>) { 
        let first_msg = format!("Guild deleted: {} - {}", guild.name, guild.id);
        let path = server_path(guild.id);
        if let Err(why) = fs::remove_dir(&path) {
            status_mirror(&format!("{}\nFailed to delete path '{}': {}",first_msg, path, why));
        } else {
        	status_mirror(&first_msg);
        }
    }
    
    //message edit
    fn on_message_update(&self, _: Context, message: event::MessageUpdateEvent) {
        let mut embed_count = 0;
        for _e in &message.embeds {
            embed_count+=1;
        }
        let embed_count = embed_count;
        
        let mut attachment_count = 0;
        for _a in &message.attachments {
            attachment_count+=1;
        }
        let attachment_count = attachment_count;
        
        println!("Edit: {} embeds, {} attachments.", embed_count, attachment_count);
    }
}
