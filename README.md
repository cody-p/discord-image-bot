# discord-image-bot
Discord bot made to distribute content to multiple servers from a single source.

This is my first project written in Rust.

### Usage
Requires two environment vars, DISCORD_NEWSBOT_TOKEN (your bot token) and DISCORD_NEWSBOT_OWNER (the owner's ID)
The owner can use the following command to send pics:
``~submit title "My title" desc "A description of this post" author "Name of author" source "link to the source of this image"``
Currently only the bot's owner can do this, nobody else can. All the args are optional.
  
Also has the command ``~set-status`` to set an output channel to print debug info such as startup messages and errors, and ``~set-output`` to set which channel to post into on any given server the bot is in (this command can be used by anyone with the Manage Channels permission).  
This bot is still in alpha so handle with care. You're recommended to put exactly one image in each post at this time.
