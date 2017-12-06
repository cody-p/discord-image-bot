use util::*;
use std::fs;
use std::io::*;
use global_data::*;
use serenity::model::*;
use serenity::CACHE;

// non owner commands
command!(set_output(_context, message) {
    if is_owner(message.author.id) {
        match message.guild_id() {
            None => {
                println!("Guild not found.");
            },
            Some(guild) => {
                let path = format!("{}/output_channel", server_path(guild));
                
                if let Ok(mut file) = fs::File::create(path) {
                    if let Err(why) = write!(file, "{}", message.channel_id) {
                        println!("Failed to write channel: {}", why);
                    } else {
                        println!("Made the channel.");
                        let _ = message.reply(&format!("Set the output channel to ``{}``.", message.channel_id));
                    }
                } else {
                    println!("Couldn't make the channel file.");
                }
            }
        }
    } else {
        let _ = message.reply("Only the bot's owner can use this command!");
    }
});

// Owner commands
command!(submit(_context, message) {
    if is_owner(message.author.id) {
        if let Ok(servers) = fs::read_dir(SERVER_PATH) {
            for entry in servers {
                if let Ok(s) = entry {
                    let path = s.path();
                    if let Ok(files) = fs::read_dir(&path) {
                        for f_entry in files {
                            if let Ok(f) = f_entry {
                                let path = f.path();
                                println!("Located channel file: {:?}",path);
                                match fs::File::open(path) {
                                    Err(why) => {
                                        println!("Error opening channel file: {}", why);
                                    },
                                    Ok(mut file) => {
                                        let mut buffer = String::new();
                                        if let Err(why) = file.read_to_string(&mut buffer) {
                                            println!("Error reading channel file: {}", why);
                                        } else {
                                            
                                            if let Ok(channel_int) = buffer.parse::<u64>() {
                                                let channel_id = ChannelId(channel_int);
                                                if let Some(channel) = CACHE.read().unwrap().guild_channel(channel_id) {
                                                    println!("Sending message to {}.", channel.read().unwrap().name);
                                                    let _ = channel.read().unwrap().send_message(|m| m
                                                        .content("Gottem")
                                                        .tts(false)
                                                        .embed(|e| e
                                                            .title("This is an embed")
                                                            .description("With a description")));
                                                    
                                                } else {
                                                    let _ = message.reply(&format!("Channel ``{}`` doesn't seem to exist.", channel_id));
                                                }
                                            } else {
                                                println!("Failed to parse {} into a u64.", buffer);
                                            }
                                        }
                                    }
                                }
                            } else {
                                println!("Error reading entry in {:?}", path);
                            }
                        }
                    } else {
                        println!("Error reading {:?}", path);
                    }
                } else {
                    println!("Error reading entry in directory.");
                }
                
            }
        } else {
            println!("Error reading {}", SERVER_PATH);
        }
    } else {
        let _ = message.reply("Only the bot's owner can use this command!'");
    }
});

