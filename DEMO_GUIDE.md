# PhisherMan Complete Demo Guide

This guide walks through a full demo from scratch with exact commands.

## Prerequisites

- Docker Desktop installed and running
- Terminal/command line access
- Web browser

## Part 1: Initial Setup (5 minutes)

### Step 1: Start the Application

```bash
cd PhisherMan
docker-compose up --build
```

Wait for these messages:
```
phisherman-db   | database system is ready to accept connections
phisherman-app  | Server running on http://127.0.0.1:3000
```

Keep this terminal open. The server is now running at **http://localhost:3000**

### Step 2: Configure Email (Choose One Option)

**Option A: Ethereal (Recommended for Demo)**

1. **Open browser** to: https://ethereal.email/create
2. The page will automatically generate test SMTP credentials
3. **Copy the credentials** shown on the page (username and password)

Stop Docker (Ctrl+C in the docker-compose terminal), then edit `.env`:
```bash
SMTP_SERVER=smtp.ethereal.email
SMTP_PORT=587
SMTP_USERNAME=generated.name@ethereal.email
SMTP_PASSWORD=abc123xyz456
FROM_EMAIL=noreply@phisherman.test
FROM_NAME=PhisherMan Security
```

Restart Docker:
```bash
docker-compose up
```

**Important**: Save the Ethereal username and password - you'll need them to login at https://ethereal.email/messages to view sent emails.

**Option B: Gmail (For Real Email Testing)**

1. Enable 2FA on your Gmail account
2. Go to https://myaccount.google.com/apppasswords
3. Create app password named "PhisherMan"
4. Edit `.env`:
```bash
SMTP_SERVER=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-16-char-app-password
FROM_EMAIL=your-email@gmail.com
FROM_NAME=PhisherMan Demo
```
5. Restart: `docker-compose down && docker-compose up`

---

## Part 2: Create Demo Data (10 minutes)

### Step 3: Create a Company

```bash
curl -X POST http://localhost:3000/api/companies \
  -H "Content-Type: application/json" \
  -d '{"name":"Acme Corporation","domain":"acme.com"}'
```

**IMPORTANT**: Copy the `id` from the response. You'll need it for all following steps.

Example response:
```json
{"id":"123e4567-e89b-12d3-a456-426614174000",...}
```

Set it as a variable for convenience:
```bash
export COMPANY_ID="123e4567-e89b-12d3-a456-426614174000"
```

### Step 4: Add Multiple Employees

Add at least 3 employees to different departments so the heatmap is interesting:

**Engineering Employee:**
```bash
curl -X POST http://localhost:3000/api/employees \
  -H "Content-Type: application/json" \
  -d "{
    \"company_id\":\"$COMPANY_ID\",
    \"email\":\"alice.smith@acme.com\",
    \"first_name\":\"Alice\",
    \"last_name\":\"Smith\",
    \"department\":\"Engineering\"
  }"
```

**Sales Employee:**
```bash
curl -X POST http://localhost:3000/api/employees \
  -H "Content-Type: application/json" \
  -d "{
    \"company_id\":\"$COMPANY_ID\",
    \"email\":\"bob.jones@acme.com\",
    \"first_name\":\"Bob\",
    \"last_name\":\"Jones\",
    \"department\":\"Sales\"
  }"
```

**Marketing Employee:**
```bash
curl -X POST http://localhost:3000/api/employees \
  -H "Content-Type: application/json" \
  -d "{
    \"company_id\":\"$COMPANY_ID\",
    \"email\":\"carol.white@acme.com\",
    \"first_name\":\"Carol\",
    \"last_name\":\"White\",
    \"department\":\"Marketing\"
  }"
```

**HR Employee:**
```bash
curl -X POST http://localhost:3000/api/employees \
  -H "Content-Type: application/json" \
  -d "{
    \"company_id\":\"$COMPANY_ID\",
    \"email\":\"dan.brown@acme.com\",
    \"first_name\":\"Dan\",
    \"last_name\":\"Brown\",
    \"department\":\"HR\"
  }"
```

Verify employees were created:
```bash
curl "http://localhost:3000/api/employees?company_id=$COMPANY_ID"
```

### Step 5: Browse and Save a Template

Open browser to: **http://localhost:3000/template.html**

1. Browse the pre-built phishing templates
2. Click **Preview** on "Password Reset - Generic"
3. Review the realistic phishing email
4. Click **Save Template**
5. Select "Acme Corporation" from dropdown
6. Click Save

Get the template ID:
```bash
curl http://localhost:3000/api/templates | grep -A 2 "Password Reset"
```

Copy the `id` field and save it:
```bash
export TEMPLATE_ID="template-uuid-here"
```

### Step 6: Create a Campaign

```bash
curl -X POST http://localhost:3000/api/campaigns \
  -H "Content-Type: application/json" \
  -d "{
    \"company_id\":\"$COMPANY_ID\",
    \"name\":\"Q4 2024 Security Awareness Test\",
    \"description\":\"Testing employee response to password reset phishing\"
  }"
```

Copy the campaign `id`:
```bash
export CAMPAIGN_ID="campaign-uuid-here"
```

---

## Part 3: Run the Phishing Simulation (5 minutes)

### Step 7: Send the Campaign

```bash
curl -X POST "http://localhost:3000/api/campaigns/$CAMPAIGN_ID/send" \
  -H "Content-Type: application/json" \
  -d "{\"template_id\":\"$TEMPLATE_ID\"}"
```

Expected response:
```json
{
  "sent_count": 4,
  "failed_count": 0,
  "message": "Campaign sent: 4 successful, 0 failed"
}
```

If `failed_count > 0`, check Docker logs:
```bash
docker-compose logs app | grep -i email
```

### Step 8: View the Sent Emails

**If using Ethereal:**
1. Go to https://ethereal.email/messages
2. Login with your Ethereal credentials (from Step 2)
3. You'll see 4 emails (one per employee)
4. Open each email and **allow images to load** (this triggers the tracking pixel)

**If using Gmail:**
1. Check the Gmail inbox for each employee email
2. Open the emails and allow images to load

### Step 9: Simulate Employee Clicks (Optional)

To make the demo more realistic, click the links in 1-2 of the emails (not all, so you have variation in the metrics).

---

## Part 4: View Analytics (The Finale!)

### Step 10: View Dashboard

Open browser to: **http://localhost:3000/dashboard.html**

You should see:
- Your campaign listed under "Campaigns"
- Status badge showing "ACTIVE"
- Template showing under "Email Templates"
- All 4 employees listed

### Step 11: View Campaign Metrics

Click the blue **"View Metrics"** button next to your campaign.

You'll see the analytics page with:

**Stats Cards:**
- Total Sent: 4
- Open Rate: (depends on how many you opened)
- Click Rate: (depends on how many links you clicked)

**Department Vulnerability Heatmap:**
- Bar chart showing click rate by department
- Color-coded: Green (<10%), Yellow (10-20%), Red (>20%)

**Department Breakdown Table:**
- Detailed stats per department
- Shows which departments are most vulnerable

### Step 12: Demonstrate the Features

Walk through explaining:

1. **Multi-tenant architecture**: Each company's data is isolated
2. **Template library**: 10+ pre-built realistic phishing scenarios
3. **Tracking**: Automatic pixel tracking for opens, link tracking for clicks
4. **Analytics**: Visual heatmap shows which departments need more training
5. **Campaign workflow**: Draft → Active → track results

---

## Part 5: Reset for Next Demo (2 minutes)

To reset the database between demos:

### Option A: Quick Reset (Keep Structure)

```bash
docker-compose exec db psql -U postgres -d phisherman -c "
TRUNCATE companies, employees, campaigns, templates, sent_emails, interactions CASCADE;
"
```

### Option B: Complete Reset

```bash
docker-compose down -v
docker-compose up --build
```

This deletes everything including Docker volumes. Start from Step 1 again.

---

## Troubleshooting

### "Campaign sent: 0 successful, 4 failed"

**Cause**: SMTP credentials not configured or wrong.

**Fix**:
1. Check `.env` file has correct SMTP credentials
2. For Ethereal: Run the curl command again to get fresh credentials
3. Restart Docker after updating `.env`

### "Error loading analytics"

**Cause**: No tracking data yet.

**Fix**:
1. Make sure you sent a campaign first
2. Open the emails in Ethereal/Gmail and allow images
3. Wait a few seconds for tracking to record
4. Refresh the analytics page

### Database connection fails

**Cause**: PostgreSQL container not ready.

**Fix**: Wait 10 seconds after `docker-compose up` before running commands.

### Port 3000 already in use

**Cause**: Old server still running.

**Fix**:
```bash
docker-compose down
lsof -ti:3000 | xargs kill -9
docker-compose up
```

---

## Demo Script (For Presentations)

**Introduction (1 min):**
"PhisherMan is a phishing simulation platform for security awareness training. It helps organizations test employee vulnerability to phishing attacks."

**Setup Demo (2 min):**
1. Show `docker-compose up` command
2. Explain it starts PostgreSQL + Rust backend
3. Show main dashboard at localhost:3000

**Create Campaign (3 min):**
1. "First, we create a company representing the organization"
2. "Add employees from different departments"
3. "Browse our template library - these are realistic phishing scenarios based on common attacks"
4. "Create a campaign and select a template"

**Send & Track (2 min):**
1. "Send campaign to all employees in the company"
2. "Emails include invisible tracking pixels"
3. Show Ethereal inbox with received emails
4. "When employees open emails, we track it automatically"

**Analytics (2 min):**
1. Click "View Metrics" button
2. "Here's the department vulnerability heatmap"
3. "We can see Sales clicked at 50% rate - high risk"
4. "Engineering only 12% - they're more aware"
5. "This helps identify which departments need more training"

**Wrap Up (1 min):**
"The platform provides complete tracking, realistic templates, and actionable analytics to improve security awareness across the organization."

---

## Advanced Features

### AI-Generated Emails

Go to http://localhost:3000/generated.html to create custom phishing emails:
1. Select phishing type (CEO fraud, package delivery, etc.)
2. Enter target role and company context
3. Set urgency level
4. Generate and save as template

### Custom Templates

Go to http://localhost:3000/custom.html to write completely custom HTML emails.

### Bulk Employee Import

Create `employees.csv`:
```csv
first_name,last_name,email,department
Alice,Smith,alice@acme.com,Engineering
Bob,Jones,bob@acme.com,Sales
```

Import with script:
```bash
while IFS=, read -r first last email dept; do
  curl -X POST http://localhost:3000/api/employees \
    -H "Content-Type: application/json" \
    -d "{\"company_id\":\"$COMPANY_ID\",\"email\":\"$email\",\"first_name\":\"$first\",\"last_name\":\"$last\",\"department\":\"$dept\"}"
done < employees.csv
```

---

## Quick Reference

**Web UI Pages:**
- `/` - Landing page
- `/dashboard.html` - Main dashboard
- `/companies.html` - Manage companies
- `/employees.html` - Manage employees
- `/campaigns.html` - Manage campaigns
- `/template.html` - Browse template library
- `/generated.html` - AI email generator
- `/custom.html` - Custom template creator
- `/analytics.html?campaign_id=X` - Campaign metrics

**Key API Endpoints:**
- `POST /api/companies` - Create company
- `POST /api/employees` - Add employee
- `POST /api/campaigns` - Create campaign
- `POST /api/campaigns/:id/send` - Send campaign
- `GET /api/campaigns/:id/analytics` - Get metrics

**Environment Variables (in .env):**
- `SMTP_SERVER` - SMTP server hostname
- `SMTP_PORT` - SMTP port (587 for TLS)
- `SMTP_USERNAME` - SMTP username
- `SMTP_PASSWORD` - SMTP password
- `FROM_EMAIL` - Sender email address
- `FROM_NAME` - Sender display name

---

**Last Updated**: December 2025
