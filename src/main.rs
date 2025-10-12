use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "alertmanager-silences-slack-reporter")]
#[command(about = "Fetch Alertmanager silences and report them to Slack", long_about = None)]
struct Args {
    #[arg(
        short = 'a',
        long,
        env = "ALERTMANAGER_URL",
        help = "Alertmanager URL"
    )]
    alertmanager_url: String,

    #[arg(
        short = 't',
        long,
        env = "SLACK_BOT_TOKEN",
        help = "Slack bot token"
    )]
    slack_bot_token: String,

    #[arg(
        short = 'c',
        long,
        env = "SLACK_CHANNEL_ID",
        help = "Slack channel ID"
    )]
    slack_channel: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Silence {
    id: String,
    status: SilenceStatus,
    matchers: Vec<Matcher>,
    #[serde(rename = "startsAt")]
    starts_at: String,
    #[serde(rename = "endsAt")]
    ends_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    #[serde(rename = "createdBy")]
    created_by: String,
    comment: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SilenceStatus {
    state: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Matcher {
    name: String,
    value: String,
    #[serde(rename = "isRegex")]
    is_regex: bool,
    #[serde(rename = "isEqual")]
    is_equal: bool,
}

#[derive(Debug, Serialize)]
struct SlackMessage {
    blocks: Vec<SlackBlock>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum SlackBlock {
    #[serde(rename = "header")]
    Header { text: SlackText },
    #[serde(rename = "section")]
    Section { text: SlackText },
    #[serde(rename = "divider")]
    Divider {},
}

#[derive(Debug, Serialize)]
struct SlackText {
    #[serde(rename = "type")]
    text_type: String,
    text: String,
}

fn fetch_silences(alertmanager_url: &str) -> Result<Vec<Silence>> {
    let url = format!("{}/api/v2/silences", alertmanager_url);
    let client = reqwest::blocking::Client::new();
    
    let response = client
        .get(&url)
        .send()
        .context("Failed to send request to Alertmanager")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Alertmanager returned error status: {}",
            response.status()
        );
    }

    let silences: Vec<Silence> = response
        .json()
        .context("Failed to parse JSON response from Alertmanager")?;

    Ok(silences)
}

fn format_slack_message(silences: &[Silence]) -> SlackMessage {
    let mut blocks = vec![
        SlackBlock::Header {
            text: SlackText {
                text_type: "plain_text".to_string(),
                text: "Alertmanager Silences Report".to_string(),
            },
        },
    ];

    let mut active_count = 0;
    let mut expired_count = 0;
    let mut pending_count = 0;

    for silence in silences {
        match silence.status.state.as_str() {
            "active" => active_count += 1,
            "expired" => expired_count += 1,
            "pending" => pending_count += 1,
            _ => {}
        }
    }

    let summary = format!(
        "*Total:* {} | *Active:* {} | *Pending:* {} | *Expired:* {}",
        silences.len(),
        active_count,
        pending_count,
        expired_count
    );

    blocks.push(SlackBlock::Section {
        text: SlackText {
            text_type: "mrkdwn".to_string(),
            text: summary,
        },
    });

    blocks.push(SlackBlock::Divider {});

    for silence in silences {
        let matchers_list = silence
            .matchers
            .iter()
            .map(|m| {
                let operator = if m.is_equal { "=" } else { "!=" };
                let regex_marker = if m.is_regex { "~" } else { "" };
                format!("  • `{}{}{}{}`", m.name, operator, regex_marker, m.value)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut text = format!(
            "*Status:* {}, *CreatedBy:* {}, *Date:* {} → {}\n*Matchers:*\n{}",
            silence.status.state,
            silence.created_by,
            format_timestamp(&silence.starts_at),
            format_timestamp(&silence.ends_at),
            matchers_list
        );

        if !silence.comment.is_empty() && silence.comment != "-" && silence.comment != "." {
            let comment_preview = if silence.comment.len() > 100 {
                format!("{}...", &silence.comment[..100])
            } else {
                silence.comment.clone()
            };
            text.push_str(&format!("\n*Comment:* _{}_", comment_preview));
        }

        blocks.push(SlackBlock::Section {
            text: SlackText {
                text_type: "mrkdwn".to_string(),
                text,
            },
        });

        blocks.push(SlackBlock::Divider {});
    }

    SlackMessage { blocks }
}

fn format_timestamp(timestamp: &str) -> String {
    timestamp
        .replace("T", " ")
        .replace("Z", "")
        .split('.')
        .next()
        .unwrap_or(timestamp)
        .to_string()
}

fn send_to_slack(token: &str, channel: &str, message: &SlackMessage) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    
    #[derive(Serialize)]
    struct SlackApiMessage<'a> {
        channel: &'a str,
        blocks: &'a [SlackBlock],
    }

    let api_message = SlackApiMessage {
        channel,
        blocks: &message.blocks,
    };

    let response = client
        .post("https://slack.com/api/chat.postMessage")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&api_message)
        .send()
        .context("Failed to send message to Slack API")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        anyhow::bail!("Slack API returned error status {}: {}", status, body);
    }

    #[derive(Deserialize)]
    struct SlackApiResponse {
        ok: bool,
        error: Option<String>,
    }

    let slack_response: SlackApiResponse = response
        .json()
        .context("Failed to parse Slack API response")?;

    if !slack_response.ok {
        anyhow::bail!(
            "Slack API returned error: {}",
            slack_response.error.unwrap_or_else(|| "unknown error".to_string())
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!(
        "Fetching silences from Alertmanager: {}",
        args.alertmanager_url
    );

    let silences = fetch_silences(&args.alertmanager_url)?;

    println!("Found {} silence(s)", silences.len());

    let message = format_slack_message(&silences);

    send_to_slack(&args.slack_bot_token, &args.slack_channel, &message)?;

    println!("Report sent to Slack successfully");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_slack_message_empty() {
        let silences = vec![];
        let message = format_slack_message(&silences);
        assert!(message.blocks.len() >= 3);
    }

    #[test]
    fn test_format_slack_message_with_data() {
        let silences = vec![Silence {
            id: "test-id-123".to_string(),
            status: SilenceStatus {
                state: "active".to_string(),
            },
            matchers: vec![Matcher {
                name: "alertname".to_string(),
                value: "TestAlert".to_string(),
                is_regex: false,
                is_equal: true,
            }],
            starts_at: "2024-01-01T00:00:00Z".to_string(),
            ends_at: "2024-01-02T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            created_by: "test-user".to_string(),
            comment: "Test comment".to_string(),
        }];

        let message = format_slack_message(&silences);
        assert!(message.blocks.len() > 3);
    }
}

