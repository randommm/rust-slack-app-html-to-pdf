use dotenvy::var;
use headless_chrome::{types::PrintToPdfOptions, LaunchOptions};
use reqwest::{header::AUTHORIZATION, multipart};
use serde_json::Value;
use std::io::Write;
use tokio::fs::File;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slack_oauth_token = var("SLACK_OAUTH_TOKEN")
        .map_err(|_| "Expected SLACK_OAUTH_TOKEN in the environment or .env file")?;
    let user_email =
        var("USER_EMAIL").map_err(|_| "Expected USER_EMAIL in the environment or .env file")?;

    let client = reqwest::Client::new();

    // Get user ID
    let response = client
        .get(format!(
            "https://slack.com/api/users.lookupByEmail?email={user_email}"
        ))
        .header(AUTHORIZATION, format!("Bearer {slack_oauth_token}"))
        .send()
        .await?;
    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {e}"))?;
    let slack_user: Value =
        serde_json::from_str(&body).map_err(|e| format!("Could not parse response body: {e}"))?;
    let slack_user_id = slack_user["user"]["id"]
        .as_str()
        .ok_or("Could not find user id in response")?
        .to_owned();

    let file_id = Uuid::new_v4().to_string();
    let filename_pdf = format!("/tmp/{file_id}.pdf");

    if std::path::Path::new("/file.html").exists() {
        let filename_pdf = filename_pdf.clone();
        println!("Starting PDF conversion");
        let handler = std::thread::spawn(|| {
            let launch_options = LaunchOptions {
                sandbox: false,
                ..Default::default()
            };
            html2pdf::html_to_pdf(
                "/file.html",
                filename_pdf,
                PrintToPdfOptions::default(),
                launch_options,
                None,
            )
        });
        while !handler.is_finished() {
            print!(".");
            std::io::stdout().flush().unwrap_or_default();
            sleep(Duration::from_millis(200)).await;
        }
        handler.join().map_err(|_| "thread error")??;
        println!("\nFinished PDF conversion");
    } else {
        return Err("HTML file not found".into());
    }

    // Send file to user on Slack
    let file = File::open(filename_pdf).await?;
    let some_file = multipart::Part::stream(file)
        .file_name("document.pdf")
        .mime_str("text/plain")?;
    let form = multipart::Form::new()
        .text("channels", slack_user_id)
        .text("filetype", "pdf")
        .part("file", some_file);
    let response = client
        .post("https://slack.com/api/files.upload")
        .header(AUTHORIZATION, format!("Bearer {slack_oauth_token}"))
        .multipart(form)
        .send()
        .await?;
    response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    Ok(())
}
