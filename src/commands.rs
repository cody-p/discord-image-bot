use util::*;
use std::fs;
use std::io::*;
use global_data::*;
use serenity::model::prelude::*;
use serenity::builder::*;
use serenity::CACHE;

//user commands
command!(about(_context, message) {
	let _ = message.reply("This command is a stub");
});

// Admin commands
command!(set_output(_context, message) {
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
});

// Owner commands
command!(die(context, message) {
	if is_owner(message.author.id) {
		let _ = message.reply("Alright then.");
		send_to_status_channel("Shutting off...");
		context.quit();
	} else {
		let _ = message.reply("wow rude");
	}
});

command!(set_status(_context, message) {
	if is_owner(message.author.id) {
		let path = format!("{}", STATUS_CHANNEL);
		
		if let Ok(mut file) = fs::File::create(path) {
			if let Err(why) = write!(file, "{}", message.channel_id) {
				println!("Failed to write status_channel: {}", why);
			} else {
				println!("Made the status_channel channel.");
				let _ = message.reply(&format!("Set the status channel to ``{}``.", message.channel_id));
			}
		} else {
			println!("Couldn't make the status_channel file.");
		}
	} else {
		let _ = message.reply("Only the bot's owner can use this command!");
	}
});

/// Submit a post to be broadcast
command!(submit(_context, message, args) {
	println!("====================
		SUBMISSION: {}
		ARGS: {}"
		, &message.content, args.full());

	let mut message_to_send = CreateMessage::default();
	let mut title = String::new();
	let mut author = String::new();
	let mut description = String::new();
	let mut tags = String::new();
	let mut source = String::new();
	let mut unused_args = String::new();
	let mut error_parsing = false;
	
	while !args.is_empty() {
		if let Ok(arg) = args.single_quoted::<String>() {
			match arg.as_ref() {
				"desc" => {
					match args.single_quoted::<String>() {
						Ok(desc) => {
							description = desc;
						},
						_ => {
							error_parsing = true;
							break;
						}
					}
				},
				"tags" => {
					match args.single_quoted::<String>() {
						Ok(tag_list) => {
							tags = tag_list;
						},
						_ => {
							error_parsing = true;
							break;
						}
					}
				},
				"title" => {
					match args.single_quoted::<String>() {
						Ok(input_title) => {
							title = input_title;
						},
						_ => {
							error_parsing = true;
							break;
						}
					}
				},
				"author" => {
					match args.single_quoted::<String>() {
						Ok(input_author) => {
							author = input_author;
						},
						_ => {
							error_parsing = true;
							break;
						}
					}
				},
				"source" => {
					match args.single_quoted::<String>() {
						Ok(input_source) => {
							source = input_source;
						},
						_ => {
							error_parsing = true;
							break;
						}
					}
				}
				&_ => {
					unused_args = format!("{} {}", unused_args, arg);
				}
			}
		}
	}
	if !error_parsing {
		if !unused_args.trim().is_empty() {
			let _ = message.reply(&format!("Warning, the following arguments were unused:\n{}", unused_args));
		}
		
		for e in &message.embeds {
			message_to_send = message_to_send.embed(|_| CreateEmbed::from(e.clone()));
		}
		
		if source.trim().is_empty() { 
			source = String::from("none provided");
		}
		for att in &message.attachments {
			if let (&Some(x), &Some(y)) = (&att.width, &att.height) {
				message_to_send = message_to_send.embed(|ce| {
					ce.title(title.clone())
					.author(|a| a.name(&author))
					.description(description.clone())
					.field("Dimensions:", &format!("{:?} x {:?}",x,y ), true)
					.field("Source:", source.clone(), true)
					.image(&att.url)
					.footer(|f| f.text(&format!("Tags: {}", tags)))
				});
			}
		}
		
		// Start scanning for recipients
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
												if let Some(channel) = CACHE.read().guild_channel(channel_id) {
													println!("Sending message to {}.", channel.read().name);
													let _ = channel.read().send_message(|_| message_to_send.clone());
													
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
		let _ = message.reply("Failure to parse args.");
	}
});
