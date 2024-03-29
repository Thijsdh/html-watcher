# HTML Watcher

This is a small tool to watch a certain url for changes. You probably want to
use this to check for changes in an HTML document, but other files should work
as well. (As long as they can be reached over HTTP(S)).

When a change is detected, you can set up notifications. Currently, only
notifications over a Discord webhook are supported.

## How to use

First, compile for release:

```bash
cargo build -r
```

The compiled binary can be found in `./target/release/html-watcher`.

After that, make sure you have set the required environment variables in `.env`.
The following variables are available:

| Variable              | Description                                                                                      | Optional | Default     |
| --------------------- | ------------------------------------------------------------------------------------------------ | -------- | ----------- |
| `URL`                 | URL to fetch                                                                                     | ❌       | -           |
| `GROUP_SIZE`          | How many lines of context should be included in the notification diff                            | ✅       | `3`         |
| `OUTPUT_DIR`          | Directory to store the last response                                                             | ✅       | `./outputs` |
| `USER_AGENT`          | User agent used to do the request                                                                | ✅       | `reqwest`   |
| `IGNORED_LINES`       | Comma-separated list of lines to ignore. All lines that contain this string are ignored          | ✅       | -           |
| `DISCORD_WEBHOOK_URL` | [Discord webhook URL](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks) | ✅       | -           |
| `DISCORD_USER_ID`     | If set, the given user is tagged.                                                                | ✅       | -           |
| `DISCORD_MESSAGE`     | If set, this message is included in the discord notification.                                    | ✅       | -           |

### Usage with cronjob

The easiest way to periodically do a check is to create a cronjob. Such a
cronjob might look like this:

```bash
0,30 * * * * cd <path-to-this-project> && ./target/release/html-watcher
```

Changing to the root of this project is required to load the `.env` file.
