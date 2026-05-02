#!/bin/bash
# Discord webhook notification for pyxel-rust project
# 既存の Nantaraquad Discord Webhook を使用
# Usage: ./scripts/notify-discord.sh "Message" [color]

set -e

# 既存の DISCORD_WEBHOOK を使用（Nantaraquad と同じ）
WEBHOOK_URL="${DISCORD_WEBHOOK}"
PROJECT_NAME="pyxel-rust"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BRANCH=$(cd "$PROJECT_ROOT" && git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
COMMIT=$(cd "$PROJECT_ROOT" && git rev-parse --short HEAD 2>/dev/null || echo "unknown")
AUTHOR=$(cd "$PROJECT_ROOT" && git log -1 --pretty=format:'%an' 2>/dev/null || echo "Unknown")
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

MESSAGE="${1:-No message}"
COLOR="${2:-3447003}"  # Default: blue

# Check if webhook URL is set
if [ -z "$WEBHOOK_URL" ]; then
    echo "⚠️  DISCORD_WEBHOOK not set. Skipping notification."
    echo "   Set with: export DISCORD_WEBHOOK='https://discordapp.com/api/webhooks/...'"
    exit 0
fi

# Create JSON payload
PAYLOAD=$(cat <<EOF
{
  "embeds": [
    {
      "title": "📦 $PROJECT_NAME",
      "description": "$MESSAGE",
      "color": $COLOR,
      "fields": [
        {
          "name": "Branch",
          "value": "\`$BRANCH\`",
          "inline": true
        },
        {
          "name": "Commit",
          "value": "[\`$COMMIT\`](https://github.com/oosawak/pyxel-rust/commit/$COMMIT)",
          "inline": true
        },
        {
          "name": "Author",
          "value": "$AUTHOR",
          "inline": true
        }
      ],
      "timestamp": "$TIMESTAMP"
    }
  ]
}
EOF
)

# Send to Discord
RESPONSE=$(curl -s -X POST \
  -H 'Content-type: application/json' \
  --data "$PAYLOAD" \
  "$WEBHOOK_URL")

if echo "$RESPONSE" | grep -q "error"; then
    echo "❌ Discord notification failed: $RESPONSE"
    exit 1
else
    echo "✅ Discord notification sent"
fi
