[![CI](https://img.shields.io/github/actions/workflow/status/raian621/honeybot/ci.yml?branch=main&label=CI)](https://github.com/raian621/honeybot/actions/workflows/ci.yml)

# 🍯 Honeybot: Discord Honeypot Bot

Blazingly fast Discord server security bot written in Rust, designed to catch
and remove spam bot accounts.

This bot communicates with Discord using the 
[Poise](https://github.com/serenity-rs/poise/) framework.

The logic is simple but quite effective:

1. **The Trap**: You set up a "honeypot" channel that no real server members
should post in.

2. **The Trigger**: An automated bot posts in the "honeypot" channel.

3. **Response**: The bot is immediately banned or kicked from the server,
depending on your configuration.

## 🚀 Hosting Quickstart

1. Create a bot application using the Discord developer portal. The bot should
have permission to "Ban Members" and "Kick Members".

2. Invite the bot to your server using the invite link generated in the Discord
developer portal.

3. Copy a Discord API token for your bot application from the Discord developer
portal.

4. Run the following command, replacing `<discord-token>` with the token you
copied in the previous step:

```sh
export DISCORD_TOKEN=<discord-token>
docker run -it -e DISCORD_TOKEN ghcr.io/raian621/honeybot
```

## 🔧 Commands

### `listen <channel_id> <response>`

Listen to a channel and respond to messages in the channel with the selected
response

**Arguments**:

- `channel_id`: The ID of the channel you want the bot to listen to.
- `response`: The response you want the bot to take for new messages in the
channel (ban, kick, etc.)

### `unlisten <channel_id>`

Stop the bot from listening to a channel that it was previously listening to.

**Arguments**:

- `channel_id`: The ID of the channel you want the bot not to listen to.

### `logging_channel`

Tell the bot where to log actions it has taken on users.

**Arguments**:

- `channel_id`: The ID of the channel the bot will log actions to.
