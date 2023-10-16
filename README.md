An Slack bot written in Rust that converts an HTML to PDF and sends it to a user on Slack, i.e.:

* First, given an user email, get the user id (necessary to send the message).

* Second convert an HTML file to PDF.

* Third, send the PDF to the user on Slack.

How to use:

Create an `.env` file at the root of the repository (same folder as the `Cargo.toml` file) with:

        SLACK_OAUTH_TOKEN="slack_token_here"
        USER_EMAIL="user_email_here"

and run:

```bash
docker build . -t rust-slackbot-html-to-pdf && docker run --rm -it -v $(pwd)/.env:/app/.env -v path_to_file:/file.html rust-slackbot-html-to-pdf
```
