# Email Sending Implementation

## Overview

The email sending feature lets you send phishing simulation campaigns to employees via SMTP. Each email includes a tracking pixel for monitoring opens.

## Configuration

Set these environment variables:

```bash
SMTP_SERVER=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
FROM_EMAIL=your-email@gmail.com
FROM_NAME=Security Team
```

For Gmail, you need an app password (not your regular password). Get one at https://myaccount.google.com/apppasswords after enabling 2FA.

## How It Works

When you send a campaign:

1. Fetch the campaign and template from the database
2. Get all employees for that company
3. For each employee:
   - Generate a unique tracking token
   - Insert a record in `sent_emails` table
   - Append tracking pixel to email body: `<img src="http://yourserver.com/track/{token}" width="1" height="1" />`
   - Send via SMTP
   - If sending fails, delete the `sent_emails` record
4. Update campaign status to "active"

The tracking pixel is a 1x1 transparent PNG. When the recipient's email client loads images, it makes a request to the `/track/{token}` endpoint, which records an interaction.

## API Usage

```bash
# Send a campaign
POST /api/campaigns/{campaign_id}/send
{
  "template_id": "uuid-here"
}

# Response
{
  "sent_count": 5,
  "failed_count": 0,
  "message": "Campaign sent: 5 successful, 0 failed"
}
```

## Tracking

The `sent_emails` table stores:
- Which employee received which email
- The tracking token for that email
- Timestamp of when it was sent

The `interactions` table records:
- Email opens (when tracking pixel loads)
- Link clicks (if you use tracking links)
- Timestamp of each interaction

## Testing

For testing without spamming real people, use Ethereal (https://ethereal.email/create). It gives you SMTP credentials and a web inbox to view "sent" emails.

## Limitations

- Sends emails sequentially (one at a time)
- No retry logic if SMTP fails
- Tracking pixel only works if recipient loads images
- Base URL is hardcoded to localhost (needs to be configurable for production)

## Future Improvements

- Batch sending for large campaigns
- Configurable base URL for tracking links
- Retry logic with exponential backoff
- Rate limiting to respect SMTP provider limits
- Queue system for async sending
