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
use serenity::model::prelude::*;
use serenity::framework::standard::{Args, CommandOptions, DispatchError, StandardFramework, /*help_commands*/};

fn main() {
	println!("Owner ID = {}", *OWNER);
	confirm_dir(SERVER_PATH);
	confirm_dir(SETTINGS_PATH);
	
	let mut client = Client::new(&TOKEN, Handler).expect("Error creating client");
	client.with_framework(StandardFramework::new()
		.configure(|c| 
			c.prefix("~")
			.on_mention(true)) // set the bot's prefix to "~"
		.group("Standard", |g| g
			.command("about", |c| c
				.desc("Information about this bot.")
				.cmd(about)
			)
		)
		.group("Administrative", |g| g
			.command("set-output", |c| c
				.desc("Sets the channel to receive messages in.")
				.required_permissions(Permissions::MANAGE_CHANNELS)
				.cmd(set_output)
			)
		)
		.group("Standard", |g| g
			.command("submit", |c| c
				.desc("Submit a message to be broadcast.")
				.check(owner_check)
				.cmd(submit)
			)
			.command("set-status", |c| c
				.check(owner_check)
				.cmd(set_status)
			)
			.command("die", |c| c
				.check(owner_check)
				.cmd(die)
			)
		)
		.on_dispatch_error(|_ctx, msg, error| {
			match error {
				/*DispatchError::CheckFailed(_arc_Command) => {
					status_mirror(&format!("Dispatch error: CheckFailed\nmessage: {}", msg.content));
				},*/
				DispatchError::CommandDisabled(str_command) => {
					status_mirror(&format!("Dispatch error: CommandDisabled({})\nmessage: {}", str_command, msg.content));
				},
				DispatchError::BlockedUser => {
					status_mirror(&format!("Dispatch error: BlockedUser({})\nmessage: {}", msg.author.name, msg.content));
				},
				DispatchError::BlockedGuild => {
					let temp = msg.guild();
					status_mirror(&format!("Dispatch error: BlockedGuild ({:?})\nmessage: {}", temp, msg.content));
				},
				DispatchError::LackOfPermissions(_permissions) => {
					status_mirror(&format!("Dispatch error: InsufficientPermissions ({})\nmessage: {}",msg.author.name, msg.content));
				},
				DispatchError::RateLimited(secs) => {
					status_mirror(&format!("Dispatch error: RateLimited ({}s left)\nmessage: {}", secs, msg.content));
				},
				DispatchError::OnlyForDM => {
					status_mirror(&format!("Dispatch error: OnlyForDM\nmessage: {}", msg.content));
				},
				DispatchError::OnlyForGuilds => {
					status_mirror(&format!("Dispatch error: OnlyForGuilds\nmessage: {}", msg.content));
				},
				DispatchError::OnlyForOwners => {
					status_mirror(&format!("Dispatch error: OnlyForOwners\nmessage: {}", msg.content));
				},
				DispatchError::LackingRole => {
					status_mirror(&format!("Dispatch error: LackingRole ({})\nmessage: {}", msg.author.name, msg.content));
				},
				DispatchError::NotEnoughArguments {min,given} => {
						status_mirror(&format!("Dispatch error: NotEnoughArguments (min: {} | given: {})\nmessage: {}", min, given, msg.content));
				},
				DispatchError::TooManyArguments {max,given} => {
					status_mirror(&format!("Dispatch error: TooManyArguments (max: {} | given: {})\nmessage: {}", max, given, msg.content));
				},
				DispatchError::IgnoredBot => {
					status_mirror(&format!("Dispatch error: IgnoredBot\nmessage: {}", msg.content));
				},
				DispatchError::WebhookAuthor => {
					status_mirror(&format!("Dispatch error: WebhookAuthor\nmessage: {}", msg.content));
				},
				_ => {
					status_mirror(&format!("Dispatch error: Unknown Reason\nmessage: {}", msg.content));
				}
			}
		})
	);
	println!("Starting...");
	if let Err(why) = client.start() {
		println!("Client error: {:?}", why);
	}
}

/// A more comprehensive wrapper of is_owner.
fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: & CommandOptions) -> bool {
	return is_owner(msg.author.id);
}
