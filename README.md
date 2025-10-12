# Alertmanager Silences Slack Reporter

A command-line tool that fetches silence configurations from Alertmanager and sends a formatted report to Slack.

![Slack Report Example](.github/images/alertmanager-silences-slack-reporter.png)

## Prerequisites

- Access to an Alertmanager instance
- A Slack bot token with appropriate permissions

## Installation & Usage

### Option 1: Docker (Recommended)

```bash
docker run --rm \
  -e ALERTMANAGER_URL="http://alertmanager:9093" \
  -e SLACK_BOT_TOKEN="xoxb-your-token" \
  -e SLACK_CHANNEL_ID="C01234ABCDE" \
  ghcr.io/f9n/alertmanager-silences-slack-reporter:latest
```

Use a specific version:
```bash
docker run --rm \
  -e ALERTMANAGER_URL="http://alertmanager:9093" \
  -e SLACK_BOT_TOKEN="xoxb-your-token" \
  -e SLACK_CHANNEL_ID="C01234ABCDE" \
  ghcr.io/f9n/alertmanager-silences-slack-reporter:1.0.0
```

### Option 2: Pre-built Binary

Download and run:

```bash
# Linux AMD64
wget https://github.com/f9n/alertmanager-silences-slack-reporter/releases/latest/download/alertmanager-silences-slack-reporter-linux-amd64.tar.gz
tar xzf alertmanager-silences-slack-reporter-linux-amd64.tar.gz

# Run with environment variables
export ALERTMANAGER_URL="http://alertmanager:9093"
export SLACK_BOT_TOKEN="xoxb-your-token"
export SLACK_CHANNEL_ID="C01234ABCDE"
./alertmanager-silences-slack-reporter

# Or with command line arguments
./alertmanager-silences-slack-reporter \
  -a http://alertmanager:9093 \
  -t xoxb-your-token \
  -c C01234ABCDE
```

### Option 3: Build from Source

```bash
git clone https://github.com/f9n/alertmanager-silences-slack-reporter.git
cd alertmanager-silences-slack-reporter
cargo build --release
./target/release/alertmanager-silences-slack-reporter --help
```

### Available Options

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
