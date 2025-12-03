#!/bin/bash
# Script to test email sending to hsiehjoshua424@gmail.com

API_BASE="http://localhost:3000/api"

echo "🧪 Testing Email Sending Setup"
echo "================================"
echo ""

# Check if server is running
echo "1. Checking if server is running..."
if ! curl -s http://localhost:3000/health > /dev/null; then
    echo "   ❌ Server is not running. Please start with: docker-compose up"
    exit 1
fi
echo "   ✅ Server is running"
echo ""

# Step 1: Create or get company
echo "2. Creating/getting company..."
COMPANY_RESPONSE=$(curl -s -X POST "$API_BASE/companies" \
    -H "Content-Type: application/json" \
    -d '{"name":"Test Company","domain":"test.com"}')
COMPANY_ID=$(echo $COMPANY_RESPONSE | grep -o '"id":"[^"]*' | cut -d'"' -f4)
if [ -z "$COMPANY_ID" ]; then
    # Try to get existing company
    COMPANIES=$(curl -s "$API_BASE/companies")
    COMPANY_ID=$(echo $COMPANIES | grep -o '"id":"[^"]*' | head -1 | cut -d'"' -f4)
fi
echo "   Company ID: $COMPANY_ID"
echo ""

# Step 2: Create employee with your email
echo "3. Creating employee with test email..."
EMPLOYEE_RESPONSE=$(curl -s -X POST "$API_BASE/employees" \
    -H "Content-Type: application/json" \
    -d "{\"company_id\":\"$COMPANY_ID\",\"email\":\"hsiehjoshua424@gmail.com\",\"first_name\":\"Josh\",\"last_name\":\"Hsieh\",\"department\":\"Testing\"}")
EMPLOYEE_ID=$(echo $EMPLOYEE_RESPONSE | grep -o '"id":"[^"]*' | cut -d'"' -f4)
echo "   Employee ID: $EMPLOYEE_ID"
echo ""

# Step 3: Create email template
echo "4. Creating test email template..."
TEMPLATE_RESPONSE=$(curl -s -X POST "$API_BASE/templates" \
    -H "Content-Type: application/json" \
    -d '{"name":"Test Email","subject":"PhisherMan Test Email","body":"<h1>Hello from PhisherMan!</h1><p>This is a test email to verify the email sending functionality.</p><p>If you received this, the email system is working correctly!</p>","template_type":"test"}')
TEMPLATE_ID=$(echo $TEMPLATE_RESPONSE | grep -o '"id":"[^"]*' | cut -d'"' -f4)
echo "   Template ID: $TEMPLATE_ID"
echo ""

# Step 4: Create campaign
echo "5. Creating test campaign..."
CAMPAIGN_RESPONSE=$(curl -s -X POST "$API_BASE/campaigns" \
    -H "Content-Type: application/json" \
    -d "{\"company_id\":\"$COMPANY_ID\",\"name\":\"Email Test Campaign\",\"description\":\"Testing email delivery\"}")
CAMPAIGN_ID=$(echo $CAMPAIGN_RESPONSE | grep -o '"id":"[^"]*' | cut -d'"' -f4)
echo "   Campaign ID: $CAMPAIGN_ID"
echo ""

# Step 5: Send the campaign
echo "6. Sending test email..."
SEND_RESPONSE=$(curl -s -X POST "$API_BASE/campaigns/$CAMPAIGN_ID/send" \
    -H "Content-Type: application/json" \
    -d "{\"template_id\":\"$TEMPLATE_ID\"}")

echo "$SEND_RESPONSE" | python3 -m json.tool 2>/dev/null || echo "$SEND_RESPONSE"
echo ""

echo "================================"
echo "✅ Test complete!"
echo ""
echo "📝 Note: By default, PhisherMan uses ethereal.email for testing,"
echo "   which doesn't actually deliver emails. To send real emails,"
echo "   you need to configure SMTP credentials in docker-compose.yml"
echo "   or as environment variables."

