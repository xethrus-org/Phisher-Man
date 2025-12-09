// Pre-built phishing email templates for security awareness training
const PRE_BUILT_TEMPLATES = [
    {
        name: "Password Reset - Generic",
        subject: "Urgent: Password Reset Required",
        category: "credential_harvesting",
        body: `<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #0066cc; color: white; padding: 20px; text-align: center; }
        .content { padding: 20px; background: #f9f9f9; }
        .button { display: inline-block; padding: 12px 24px; background: #0066cc; color: white; text-decoration: none; border-radius: 4px; margin: 20px 0; }
        .footer { font-size: 12px; color: #666; padding: 20px; text-align: center; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h2>Security Alert</h2>
        </div>
        <div class="content">
            <p>Dear User,</p>
            <p>We have detected unusual activity on your account. For your security, please reset your password immediately.</p>
            <p><strong>Action Required:</strong> Your password will expire in 24 hours if not reset.</p>
            <p style="text-align: center;">
                <a href="#" class="button">Reset Password Now</a>
            </p>
            <p>If you did not request this change, please contact IT support immediately.</p>
            <p>Thank you,<br>IT Security Team</p>
        </div>
        <div class="footer">
            <p>This is an automated message. Please do not reply to this email.</p>
        </div>
    </div>
</body>
</html>`
    },
    {
        name: "IT Support Verification",
        subject: "Action Required: Verify Your Account",
        category: "credential_harvesting",
        body: `<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #d32f2f; color: white; padding: 20px; text-align: center; }
        .content { padding: 20px; background: #f9f9f9; border-left: 4px solid #d32f2f; }
        .button { display: inline-block; padding: 12px 24px; background: #d32f2f; color: white; text-decoration: none; border-radius: 4px; margin: 20px 0; }
        .warning { background: #fff3cd; border: 1px solid #ffc107; padding: 10px; margin: 15px 0; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h2>⚠️ Account Verification Required</h2>
        </div>
        <div class="content">
            <div class="warning">
                <strong>WARNING:</strong> Your account access will be suspended in 48 hours.
            </div>
            <p>Hello,</p>
            <p>We are conducting a mandatory security audit and need you to verify your account information.</p>
            <p><strong>Why is this needed?</strong> Recent security policies require all users to confirm their credentials.</p>
            <p style="text-align: center;">
                <a href="#" class="button">Verify Account Now</a>
            </p>
            <p>Failure to verify will result in temporary account suspension.</p>
            <p>Best regards,<br>IT Support Team</p>
        </div>
    </div>
</body>
</html>`
    },
    {
        name: "CEO Urgent Request",
        subject: "RE: Urgent - Need This ASAP",
        category: "business_email_compromise",
        body: `<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .content { padding: 20px; background: white; }
    </style>
</head>
<body>
    <div class="container">
        <div class="content">
            <p>Hi <strong>[Employee Name]</strong> , </p>
            <p>I'm in back-to-back meetings all day and need your help with something urgent.</p>
            <p>In preparation for our client appreciation event happening tomorrow, please go ahead and purchase 5 $250 gift cards and put them on the company card, and send me the card numbers ASAP so I can forward them.</p>
            <p>Please handle this discreetly - it's a surprise for the team that's are finalizing the deal.</p>
            <p>Let me know once it's been taken care of.</p>
            <p>Thanks,<br>
            <strong>[CEO Name]</strong><br>
            <em>Sent from my iPhone</em></p>
        </div>
    </div>
</body>
</html>`
    },
    {
        name: "Payroll Update Required",
        subject: "Action Required: Update Your Direct Deposit Information",
        category: "credential_harvesting",
        body: `<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #2e7d32; color: white; padding: 20px; text-align: center; }
        .content { padding: 20px; background: #f9f9f9; }
        .button { display: inline-block; padding: 12px 24px; background: #2e7d32; color: white; text-decoration: none; border-radius: 4px; margin: 20px 0; }
        .footer { font-size: 12px; color: #666; padding: 20px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h2>Payroll Department</h2>
        </div>
        <div class="content">
            <p>Dear Employee,</p>
            <p>We are updating our payroll system and need all employees to verify their direct deposit information by Friday.</p>
            <p><strong>Important:</strong> Failure to update your information may result in delayed payment.</p>
            <p style="text-align: center;">
                <a href="#" class="button">Update Banking Information</a>
            </p>
            <p>This process takes less than 2 minutes and ensures your next paycheck is deposited correctly.</p>
            <p>Thank you for your prompt attention to this matter.</p>
            <p>Payroll Department</p>
        </div>
        <div class="footer">
            <p>Company Payroll Services | Confidential</p>
        </div>
    </div>
</body>
</html>`
    },
    {
        name: "Package Delivery Notification",
        subject: "Your package delivery was unsuccessful",
        category: "malware",
        body: `<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #ff6f00; color: white; padding: 20px; text-align: center; }
        .content { padding: 20px; background: #f9f9f9; }
        .button { display: inline-block; padding: 12px 24px; background: #ff6f00; color: white; text-decoration: none; border-radius: 4px; margin: 20px 0; }
        .tracking { background: #fff; border: 2px solid #ff6f00; padding: 15px; margin: 15px 0; font-family: monospace; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h2>📦 Delivery Notification</h2>
        </div>
        <div class="content">
            <p>Dear Customer,</p>
            <p>We attempted to deliver your package but no one was available to receive it.</p>
            <div class="tracking">
                <strong>Tracking Number:</strong> 1Z999AA10123456784<br>
                <strong>Delivery Attempt:</strong> Today at 2:43 PM
            </div>
            <p>To reschedule your delivery or pick up your package, please download your delivery notice:</p>
            <p style="text-align: center;">
                <a href="#" class="button">Download Delivery Notice (PDF)</a>
            </p>
            <p>Your package will be held at our facility for 5 business days.</p>
            <p>Shipping Services</p>
        </div>
    </div>
</body>
</html>`
    },
    {
        name: "Microsoft 365 Security Alert",
        subject: "Security Alert: Unusual Sign-in Activity",
        category: "credential_harvesting",
        body: `<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: 'Segoe UI', Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #0078d4; color: white; padding: 20px; }
        .logo { font-size: 24px; font-weight: bold; }
        .content { padding: 20px; background: white; border: 1px solid #ddd; }
        .alert-box { background: #fff4ce; border-left: 4px solid #ffb900; padding: 15px; margin: 15px 0; }
        .button { display: inline-block; padding: 12px 24px; background: #0078d4; color: white; text-decoration: none; border-radius: 2px; margin: 20px 0; }
        .details { background: #f5f5f5; padding: 15px; margin: 15px 0; font-size: 14px; }
        .footer { font-size: 12px; color: #666; padding: 20px; text-align: center; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="logo">Microsoft</div>
        </div>
        <div class="content">
            <h2>Security Alert</h2>
            <div class="alert-box">
                <strong>⚠️ We detected unusual sign-in activity on your account</strong>
            </div>
            <p>Hello,</p>
            <p>We detected a sign-in to your Microsoft account from an unrecognized device or location.</p>
            <div class="details">
                <strong>Sign-in Details:</strong><br>
                Location: Moscow, Russia<br>
                Device: Chrome on Linux<br>
                Time: Today at 3:24 AM EST
            </div>
            <p>If this was you, you can safely ignore this email. If not, please secure your account immediately:</p>
            <p style="text-align: center;">
                <a href="#" class="button">Review Recent Activity</a>
            </p>
            <p>We recommend changing your password and enabling two-factor authentication.</p>
            <p>Thanks,<br>The Microsoft Account Team</p>
        </div>
        <div class="footer">
            <p>Microsoft Corporation | One Microsoft Way | Redmond, WA 98052</p>
        </div>
    </div>
</body>
</html>`
    }
];
