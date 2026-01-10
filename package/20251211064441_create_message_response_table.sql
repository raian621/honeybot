CREATE TABLE message_responses (
  guild_id   INTEGER NOT NULL,
  channel_id INTEGER NOT NULL,
  response   INTEGER NOT NULL,
  PRIMARY KEY(guild_id, channel_id)
);
