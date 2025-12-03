const express = require('express');
const path = require('path');
const nodemailer = require('nodemailer');
const dotenv = require('dotenv');
const OpenAI = require('openai');

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3000;

const client = new OpenAI({
    apiKey: process.env.OPENAI_API_KEY,
});

app.use(express.json());
app.use(express.static(path.join(__dirname, 'public')));

let emailDraft = {};
app.post("/load-email", (req, res) => {
    const { subject, message, sender } = req.body;
    emailDraft = { subject, message, sender: etherealSender };
    res.json({
        success: true,
        draft: emailDraft
    })
})

let transporter;
let etherealSender = null;

async function setupMailer() {
    const testAccount = await nodemailer.createTestAccount();

    transporter = nodemailer.createTransport({
        host: "smtp.ethereal.email",
        port: 587,
        secure: false,
        auth: {
            user: testAccount.user,
            pass: testAccount.pass
        }
    });

    etherealSender = testAccount.user;
    console.log("Ethereal Test Account:");
    console.log("User:", testAccount.user);
    console.log("Pass:", testAccount.pass);
}

app.post("/api/send-emails", async (req, res) => {
    try {
        const { recipients } = req.body;

        if (!Array.isArray(recipients) || recipients.length === 0) {
            return res.status(400).send("No recipients provided.");
        }

        if (!emailDraft.subject || !emailDraft.message || !emailDraft.sender) {
            return res.status(400).send("Subject and body are required.");
        }

        if (!emailDraft.sender) {
            return res.status(400).send("Sender email not loaded. Cannot send.");
        }

        const info = await transporter.sendMail({
            from: emailDraft.sender,
            to: recipients,
            subject : emailDraft.subject,
            html: emailDraft.message,
            text: emailDraft.message.replace(/<[^>]+>/g, ""),
        });

        const previewUrl = nodemailer.getTestMessageUrl(info);
        console.log(previewUrl);

        res.json({
            success: true,
            message: "Email sent to recipients.",
            previewUrl,
        });
    } catch (err) {
        console.error("Error sending emails:", err);
        res.status(500).send("Error sending emails.");
    }
});

setupMailer().then(() => {
    app.listen(PORT, () => console.log(`Server running at http://localhost:${PORT}`));
}).catch(err => {
    console.error("Failed to set up mailer:", err);
});
