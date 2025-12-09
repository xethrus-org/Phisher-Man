# Email Testing Guide

## Current Status

The PhisherMan application is set up for email sending, but currently uses **default test credentials** (ethereal.email) which don't actually deliver emails.

## To Send Real Emails to Gmail

You need to configure SMTP credentials. Here are your options:

### Option 1: Use Gmail SMTP (Recommended for Testing)

1. **Enable 2-Factor Authentication** on your Gmail account
2. **Create an App Password**:
   - Go to: https://myaccount.google.com/apppasswords
   - Select "Mail" and "Other (Custom name)"
   - Name it "PhisherMan" and generate the password
   - Copy the 16-character password

3. **Update docker-compose.yml** to add SMTP environment variables:

```yaml
app:
  environment:
    # ... existing vars ...
    SMTP_USERNAME: your-email@gmail.com
    SMTP_PASSWORD: your-16-char-app-password
    SMTP_SERVER: smtp.gmail.com
    SMTP_PORT: 587
    FROM_EMAIL: your-email@gmail.com
    FROM_NAME: PhisherMan Security
```

4. **Restart the containers**:
```bash
docker-compose down
docker-compose up --build
```

### Option 2: Use a Testing Service (Mailtrap, etc.)

For development/testing, services like Mailtrap are great - they capture emails without actually sending them.

### Option 3: Use Default Test Mode (Ethereal.email)

The current default will attempt to send but won't actually deliver. It's useful for testing the code path but not for real delivery.

## Testing Email Sending

Once SMTP is configured, you can test by:

### Method 1: Use the test script
```bash
./test_email.sh
```

### Method 2: Manual API calls

1. **Create a company**:
```bash
curl -X POST http://localhost:3000/api/companies \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Company","domain":"test.com"}'
```

2. **Create an employee** (replace COMPANY_ID):
```bash
curl -X POST http://localhost:3000/api/employees \
  -H "Content-Type: application/json" \
  -d '{
    "company_id":"COMPANY_ID_HERE",
    "email":"hsiehjoshua424@gmail.com",
    "first_name":"Josh",
    "last_name":"Hsieh"
  }'
```

3. **Create a template**:
```bash
curl -X POST http://localhost:3000/api/templates \
  -H "Content-Type: application/json" \
  -d '{
    "name":"Test Email",
    "subject":"PhisherMan Test",
    "body":"<h1>Hello!</h1><p>This is a test email.</p>"
  }'
```

4. **Create a campaign** (replace COMPANY_ID):
```bash
curl -X POST http://localhost:3000/api/campaigns \
  -H "Content-Type: application/json" \
  -d '{
    "company_id":"COMPANY_ID_HERE",
    "name":"Test Campaign"
  }'
```

5. **Send the campaign** (replace CAMPAIGN_ID and TEMPLATE_ID):
```bash
curl -X POST http://localhost:3000/api/campaigns/CAMPAIGN_ID_HERE/send \
  -H "Content-Type: application/json" \
  -d '{"template_id":"TEMPLATE_ID_HERE"}'
```

## Environment Variables Reference

| Variable | Default | Description |
|----------|---------|-------------|
| `SMTP_USERNAME` | `test@ethereal.email` | SMTP username |
| `SMTP_PASSWORD` | `password` | SMTP password |
| `SMTP_SERVER` | `smtp.ethereal.email` | SMTP server hostname |
| `SMTP_PORT` | `587` | SMTP port (587 for TLS) |
| `FROM_EMAIL` | `noreply@phisherman.test` | From email address |
| `FROM_NAME` | `PhisherMan Security` | From name |

## Troubleshooting

- **Email not received?** Check server logs: `docker-compose logs app`
- **SMTP connection failed?** Verify credentials and firewall settings
- **Gmail blocking?** You may need to allow "less secure apps" or use an app password

## Security Note

Never commit SMTP credentials to git! Use environment variables or a `.env` file (which should be in `.gitignore`).

