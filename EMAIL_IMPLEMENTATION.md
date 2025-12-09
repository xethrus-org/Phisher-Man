# Email Sending Implementation

## Overview

The email sending feature lets you send phishing simulation campaigns to employees via SMTP. Each email includes tracking pixels for monitoring opens and wrapped links for tracking clicks.

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
   - **Replace all links** with tracking URLs: `<a href="http://localhost:3000/click/{token}/{link_id}">`
   - Store original URLs in `tracked_links` table
   - Append tracking pixel to email body: `<img src="http://yourserver.com/track/{token}" width="1" height="1" />`
   - Send via SMTP
   - If sending fails, delete the `sent_emails` record
4. Update campaign status to "active"

**Email Open Tracking:** 1x1 transparent PNG image that records a view when loaded.

**Link Click Tracking:** All `<a href>` tags are automatically replaced with tracking URLs. When clicked, the system records the interaction and redirects to the original URL.

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

The `tracked_links` table stores:
- The original URL before replacement
- Link index (for multiple links in one email)
- Reference to which sent_email it belongs to

The `interactions` table records:
- Email opens (when tracking pixel loads)
- Link clicks (when tracking URL is clicked)
- Timestamp of each interaction
- Metadata (like which link was clicked)

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
