# Event-bot

A Discord bot created to send notifications when certain activities finish. The application will parse the command line
arguments, format the message and send it to the desired channel. Then the bot will be offline again.

Current support:

- torrents (triggered when QBitTorrent finishes a download)

    add the following to the QBitTorrent settings under `Tools>Options...>Downloads>Run on torrent finished:`

    ```bat
    .\path\to\run.bat torrent --name "%N" --category "%L" --tags "%G" --content-path "%F"
    --root-path "%R" --save-path "%D" --files "%C" --byte-size "%Z" --tracker "%T"
    ```

## Setup

To use, ensure the following environment variables are set, or in a `.env` file in the same directory as the current
working directory.

```ini
token=... # required
torrents_channel=... # optional, must be provided if torrents feature is used
```

For the token, go to https://discord.com/developers/applications, select your bot, chose "Bot" from the left, get the
token. For channel IDs, enable developer mode in Discord, right click the channel and select get ID.
