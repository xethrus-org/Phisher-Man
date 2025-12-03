# Manual Email Testing Guide

## Prerequisites

Make sure your server is running:
```bash
# Check if server is up
curl http://localhost:3000/health
# Should return: ok
```

## Step-by-Step Manual Test

### Step 1: Create a Company

```bash
curl -X POST http://localhost:3000/api/companies \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Company","domain":"test.com"}'
```

**Save the `id` from the response** - you'll need it for the next steps.

Example response:
```json
{"id":"123e4567-e89b-12d3-a456-426614174000","name":"Test Company",...}
```

---

### Step 2: Create an Employee (with your email)

Replace `YOUR_COMPANY_ID` with the ID from Step 1:

```bash
curl -X POST http://localhost:3000/api/employees \
  -H "Content-Type: application/json" \
  -d '{
    "company_id":"YOUR_COMPANY_ID",
    "email":"hsiehjoshua424@gmail.com",
    "first_name":"Josh",
    "last_name":"Hsieh",
    "department":"Testing"
  }'
```

**Save the `id` from the response** (optional, but good to have).

---

### Step 3: Create an Email Template

```bash
curl -X POST http://localhost:3000/api/templates \
  -H "Content-Type: application/json" \
  -d '{
    "name":"Test Email Template",
    "subject":"PhisherMan Test Email",
    "body":"<h1>Hello from PhisherMan!</h1><p>This is a test email to verify the email sending functionality.</p><p>If you received this email, the SMTP configuration is working correctly!</p><p>Sent from: PhisherMan Security Platform</p>",
    "template_type":"test"
  }'
```

**Save the `id` from the response** - you'll need it to send the campaign.

---

### Step 4: Create a Campaign

Replace `YOUR_COMPANY_ID` with the ID from Step 1:

```bash
curl -X POST http://localhost:3000/api/campaigns \
  -H "Content-Type: application/json" \
  -d '{
    "company_id":"YOUR_COMPANY_ID",
    "name":"Email Test Campaign",
    "description":"Testing email delivery to Gmail"
  }'
```

**Save the `id` from the response** - you'll need it to send the campaign.

---

### Step 5: Send the Campaign (This sends the email!)

Replace `YOUR_CAMPAIGN_ID` and `YOUR_TEMPLATE_ID` with the IDs from previous steps:

```bash
curl -X POST http://localhost:3000/api/campaigns/YOUR_CAMPAIGN_ID/send \
  -H "Content-Type: application/json" \
  -d '{"template_id":"YOUR_TEMPLATE_ID"}'
```

**Expected response:**
```json
{
  "sent_count": 1,
  "failed_count": 0,
  "message": "Campaign sent: 1 successful, 0 failed"
}
```

---

## Quick Test Script (All-in-One)

If you want to do it all at once, here's a script that does everything:

```bash
#!/bin/bash

API="http://localhost:3000/api"

echo "Step 1: Creating company..."
COMPANY=$(curl -s -X POST "$API/companies" \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Company","domain":"test.com"}')
COMPANY_ID=$(echo $COMPANY | grep -o '"id":"[^"]*' | head -1 | cut -d'"' -f4)
echo "Company ID: $COMPANY_ID"

echo "Step 2: Creating employee..."
EMPLOYEE=$(curl -s -X POST "$API/employees" \
  -H "Content-Type: application/json" \
  -d "{\"company_id\":\"$COMPANY_ID\",\"email\":\"hsiehjoshua424@gmail.com\",\"first_name\":\"Josh\",\"last_name\":\"Hsieh\"}")
echo "Employee created"

echo "Step 3: Creating template..."
TEMPLATE=$(curl -s -X POST "$API/templates" \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Email","subject":"PhisherMan Test","body":"<h1>Test Email</h1><p>This is a test!</p>"}')
TEMPLATE_ID=$(echo $TEMPLATE | grep -o '"id":"[^"]*' | head -1 | cut -d'"' -f4)
echo "Template ID: $TEMPLATE_ID"

echo "Step 4: Creating campaign..."
CAMPAIGN=$(curl -s -X POST "$API/campaigns" \
  -H "Content-Type: application/json" \
  -d "{\"company_id\":\"$COMPANY_ID\",\"name\":\"Test Campaign\"}")
CAMPAIGN_ID=$(echo $CAMPAIGN | grep -o '"id":"[^"]*' | head -1 | cut -d'"' -f4)
echo "Campaign ID: $CAMPAIGN_ID"

echo "Step 5: Sending email..."
RESULT=$(curl -s -X POST "$API/campaigns/$CAMPAIGN_ID/send" \
  -H "Content-Type: application/json" \
  -d "{\"template_id\":\"$TEMPLATE_ID\"}")
echo "$RESULT" | python3 -m json.tool 2>/dev/null || echo "$RESULT"

echo "Done! Check your email inbox."
```

Save this as `quick_test.sh`, make it executable (`chmod +x quick_test.sh`), and run it.

---

## Troubleshooting

### Check if server is running:
```bash
curl http://localhost:3000/health
```

### Check server logs for email errors:
```bash
docker-compose logs app | grep -i email
# or if running locally:
# Check your terminal where cargo run is executing
```

### Verify SMTP configuration is loaded:
```bash
docker-compose exec app env | grep SMTP
```

### Common Issues:

1. **"No employees found"** - Make sure you created the employee with the correct company_id
2. **SMTP connection failed** - Check that your Gmail app password is correct
3. **Email not received** - Check spam folder, verify SMTP credentials in logs

---

## Using jq for Better Output (Optional)

If you have `jq` installed, you can format JSON responses nicely:

```bash
curl -X POST http://localhost:3000/api/companies \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Company","domain":"test.com"}' | jq
```

This will pretty-print the JSON response.

