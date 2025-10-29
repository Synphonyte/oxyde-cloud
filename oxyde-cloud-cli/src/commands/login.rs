use crate::api_key::api_key_entry;
use crate::commands::logout::logout;
use anyhow::{Context, Result};
use axum::{Router, extract::Query, response::Html, routing::get};
use cliclack::log::remark;
use cliclack::{intro, outro, outro_cancel, spinner};
use oxyde_cloud_client::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

async fn handle_callback(
    Query(params): Query<HashMap<String, String>>,
    tx: Arc<tokio::sync::Mutex<Option<oneshot::Sender<String>>>>,
) -> Html<String> {
    if let Some(api_key) = params.get("api_key") {
        // Send the API key back to the main thread
        if let Ok(mut sender_opt) = tx.try_lock() {
            if let Some(sender) = sender_opt.take() {
                let _ = sender.send(api_key.clone());
            }
        }

        r#"
            <html>
                <head><title>Login Successful</title></head>
                <body>
                    <h1>✅ Login Successful!</h1>
                    <p>You can now close this browser window and return to your terminal.</p>
                    <script>setTimeout(() => window.close(), 2000);</script>
                </body>
            </html>
        "#
        .to_string()
        .into()
    } else {
        let error_msg = params
            .get("error")
            .map(String::as_str)
            .unwrap_or("Unknown error occurred");

        format!(
            r#"
            <html>
                <head><title>Login Failed</title></head>
                <body>
                    <h1>❌ Login Failed</h1>
                    <p>Error: {error_msg}</p>
                    <p>Please return to your terminal and try again.</p>
                </body>
            </html>
        "#
        )
        .into()
    }
}

pub async fn login() -> Result<()> {
    intro("Login").context("Failed to show login intro")?;

    let keyring_entry = api_key_entry().context("Failed to access keyring")?;

    if keyring_entry.get_password().is_ok() {
        outro("You're already logged in.").context("Failed to show outro message")?;
        return Ok(());
    }

    // Start local callback server
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("Failed to bind to local address")?;
    let local_addr = listener
        .local_addr()
        .context("Failed to get local address")?;

    let (tx, rx) = oneshot::channel::<String>();
    let tx = Arc::new(tokio::sync::Mutex::new(Some(tx)));

    // Create the Axum app
    let app = Router::new().route(
        "/callback",
        get({
            let tx = tx.clone();
            move |query| handle_callback(query, tx)
        }),
    );

    // Spawn the server task
    let server_task = tokio::spawn(async move { axum::serve(listener, app).await });

    let callback_url = format!("http://localhost:{}/callback", local_addr.port());
    let auth_url = format!(
        "https://oxyde.cloud/dashboard/authorize-app?callback={}",
        urlencoding::encode(&callback_url)
    );

    remark("Opening browser to authenticate...").context("Failed to show message")?;

    if let Err(err) = opener::open(&auth_url) {
        remark(format!(
            "Failed to open browser automatically.\nIn your browser, please visit: {auth_url}.\n\nError details: {err:#?}\n"

        ))
        .context("Failed to show manual URL message")?;
    }

    let auth_spinner = spinner();
    auth_spinner.start("Waiting for authentication from browser...");

    // Wait for API key with timeout
    let api_key = match tokio::time::timeout(
        tokio::time::Duration::from_secs(300), // 5 minute timeout
        rx,
    )
    .await
    {
        Ok(Ok(key)) => {
            server_task.abort();
            auth_spinner.stop("Successfully authenticated!");
            key
        }
        Ok(Err(_)) => {
            server_task.abort();
            auth_spinner.error("Failed to receive authentication!");
            outro_cancel("Authentication was cancelled")
                .context("Failed to show cancellation message")?;
            return Ok(());
        }
        Err(_) => {
            server_task.abort();
            auth_spinner.error("Timeout!");
            outro_cancel("Timed out waiting for authentication. Please try again.")
                .context("Failed to show timeout message")?;
            return Ok(());
        }
    };

    keyring_entry
        .set_password(&api_key)
        .context("Failed to store authentication key in keyring")?;

    let login_spinner = spinner();
    login_spinner.start("Logging in...");

    match Client::new(api_key).login().await {
        Ok(login_result) => {
            login_spinner.stop("Done!");
            outro(format!(
                "You're now logged in as {}.",
                login_result.username
            ))
            .context("Failed to show login success message")?;
        }
        Err(err) => {
            logout().context("Failed to logout after login error")?;

            login_spinner.error("Failed!");
            outro_cancel(format!("Failed to login: {err}"))
                .context("Failed to show login error message")?;
        }
    }

    Ok(())
}
