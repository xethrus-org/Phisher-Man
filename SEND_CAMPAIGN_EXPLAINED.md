# What Does "Send Campaign" Do?

## Overview

The "Send Campaign" feature sends phishing simulation emails to all employees associated with a campaign's company.

## Step-by-Step Process

1. **Selects a Campaign**: Finds the campaign by ID
2. **Gets the Email Template**: Retrieves the template you selected
3. **Finds All Employees**: Gets all employees belonging to the campaign's company
4. **Sends Emails**: For each employee:
   - Creates a tracking record in the database
   - Adds a tracking pixel to the email (for future analytics)
   - Sends the email using SMTP
   - Records success/failure
5. **Updates Campaign Status**: Changes campaign status to "active" and records start time
6. **Returns Results**: Shows how many emails were sent successfully vs failed

## What Gets Sent

- **To**: Each employee's email address
- **From**: Configured sender (PhisherMan Security)
- **Subject**: From the selected template
- **Body**: Template body + invisible tracking pixel
- **Tracking**: Each email gets a unique tracking token for analytics

## Example Flow

```
Campaign: "Q4 Security Training"
Company: "Acme Corp"
Employees: ["john@acme.com", "jane@acme.com", "bob@acme.com"]
Template: "Urgent Password Reset"

Result: Sends 3 emails, all with the same template but personalized to each employee
```

## Use Cases

- **Security Training**: Send phishing simulations to test employee awareness
- **Campaign Testing**: Test email templates before full deployment
- **Bulk Sending**: Send to multiple employees at once

