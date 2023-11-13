A Slack app written in Rust that converts an HTML to PDF and sends it to a user on Slack, i.e.:

* First, given an user email, get the user id (necessary to send the message).

* Second convert an HTML file to PDF.

* Third, send the PDF to the user on Slack.

How to use:

Create a Slack app at https://api.slack.com/apps and navigate to "OAuth & Permissions", then add the following permision are needed on Slack: chat:write, files:write, users:read and users:read.email and the same page look for "install the app on your workspace".

Next, create an `.env` file at the root of the repository (same folder as the `Cargo.toml` file) with:

        SLACK_OAUTH_TOKEN="slack_token_here"
        USER_EMAIL="user_email_here"

and run:

```bash
docker build . -t rust-slackbot-html-to-pdf && docker run --rm -it -v $(pwd)/.env:/app/.env -v path/to/file.html:/file.html rust-slackbot-html-to-pdf
```
