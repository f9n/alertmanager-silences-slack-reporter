# Alertmanager Silences Slack Reporter

A command-line tool that fetches silence configurations from Alertmanager and sends a formatted report to Slack.

![Slack Report Example](.github/images/alertmanager-silences-slack-reporter.png)

## Prerequisites

- Access to an Alertmanager instance
- A Slack bot token with appropriate permissions

## Installation

### Building from Source

```bash
# Clone the repository
git clone https://github.com/f9n/alertmanager-silences-slack-reporter.git
cd alertmanager-silences-slack-reporter

# Build the project in release mode
cargo build --release

# The binary will be available at
./target/release/alertmanager-silences-slack-reporter
```

## Usage

### Command Line Arguments

```bash
alertmanager-silences-slack-reporter \
  --alertmanager-url http://alertmanager.example.com:9093 \
  --slack-bot-token xoxb-your-bot-token \
  --slack-channel-id C01234ABCDE
```

### Short Form

```bash
alertmanager-silences-slack-reporter \
  -a http://alertmanager.example.com:9093 \
  -t xoxb-your-bot-token \
  -c C01234ABCDE
```

### Environment Variables

```bash
export ALERTMANAGER_URL="http://alertmanager.example.com:9093"
export SLACK_BOT_TOKEN="xoxb-your-bot-token"
export SLACK_CHANNEL_ID="C01234ABCDE"

alertmanager-silences-slack-reporter
```

### Options

- `-a, --alertmanager-url <URL>`: Alertmanager base URL (env: `ALERTMANAGER_URL`)
- `-t, --slack-bot-token <TOKEN>`: Slack bot token (env: `SLACK_BOT_TOKEN`)
- `-c, --slack-channel-id <ID>`: Slack channel ID (env: `SLACK_CHANNEL_ID`)
- `-h, --help`: Display help message

## Setting Up Slack Bot

To use this tool, you need to create a Slack App with bot token permissions.

For detailed instructions on creating a Slack App and obtaining a bot token, see the [Slack API documentation](https://api.slack.com/bot-users).

Required bot token scopes:
- `chat:write`
- `chat:write.public` (optional, if you want to post to channels the bot isn't in)

## Development

### Running Tests

```bash
cargo test
```

### Running in Debug Mode

```bash
cargo run -- \
  --alertmanager-url http://localhost:9093 \
  --slack-bot-token xoxb-your-token \
  --slack-channel-id C01234ABCDE
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
