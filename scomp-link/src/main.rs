use oauth2::{
    basic::BasicClient, 
    reqwest::async_http_client, 
    AuthUrl, 
    AuthorizationCode, 
    ClientId,
    ClientSecret, 
    CsrfToken,
    RedirectUrl, 
    Scope, 
    TokenResponse, 
    TokenUrl,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{fs, io};
use tokio::{self, task};

#[derive(Deserialize, Debug)]
struct MessageList {
    messages: Vec<Message>,
    next_page_token: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Message {
    id: String,
}

#[derive(Deserialize, Debug)]
struct MessageDetails {
    snippet: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load client credentials from `credentials.json`
    let creds = fs::read_to_string("credentials.json")?;
    let creds_json: serde_json::Value = serde_json::from_str(&creds)?;

    let client_id = creds_json["installed"]["client_id"]
        .as_str()
        .expect("Missing client_id")
        .to_string();
    let client_secret = creds_json["installed"]["client_secret"]
        .as_str()
        .expect("Missing client_secret")
        .to_string();
    let auth_uri = creds_json["installed"]["auth_uri"]
        .as_str()
        .expect("Missing auth_uri")
        .to_string();
    let token_uri = creds_json["installed"]["token_uri"]
        .as_str()
        .expect("Missing token_uri")
        .to_string();
    let redirect_uris = creds_json["installed"]["redirect_uris"]
        .as_array()
        .expect("Missing redirect_uris");
    let redirect_url = redirect_uris[0]
        .as_str()
        .expect("Missing redirect_url")
        .to_string();

    // Configure OAuth2 client
    let oauth_client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_uri)?,
        Some(TokenUrl::new(token_uri)?),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url)?);

    // Generate the authorization URL
    let (auth_url, _) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/gmail.readonly".to_string()))
        .url();

    open::that(auth_url.to_string())?;

    // Get the authorization code from the user
    println!("Enter the authorization code:");
    let mut auth_code = String::new();
    io::stdin().read_line(&mut auth_code)?;
    let auth_code = auth_code.trim().to_string();

    // Exchange the authorization code for an access token
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(auth_code))
        .request_async(async_http_client)
        .await?;
    let access_token = token_result.access_token().secret().to_string();

    let http_client = Client::new();

    // Fetch and print emails
    fetch_all_emails(http_client, &access_token).await?;

    Ok(())
}

async fn fetch_all_emails(client: Client, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut next_page_token: Option<String> = None;

    loop {
        let mut url = "https://www.googleapis.com/gmail/v1/users/me/messages".to_string();
        if let Some(page_token) = &next_page_token {
            url.push_str(&format!("?pageToken={}", page_token));
        }

        let response = client
            .get(&url)
            .bearer_auth(token)
            .send()
            .await?
            .json::<MessageList>()
            .await?;

        // Process messages concurrently
        let tasks: Vec<_> = response
            .messages
            .into_iter()
            .map(|message| {
                let client = client.clone();
                let token = token.to_string();
                task::spawn(async move {
                    match fetch_email_details(&client, &token, &message.id).await {
                        Ok(snippet) => println!("Email ID: {}, Snippet: {}", message.id, snippet),
                        Err(e) => eprintln!("Failed to fetch email ID {}: {}", message.id, e),
                    }
                })
            })
            .collect();

        // Wait for all tasks to complete
        for task in tasks {
            task.await?;
        }

        // Check if there is another page
        if let Some(page_token) = response.next_page_token {
            next_page_token = Some(page_token);
        } else {
            break;
        }
    }

    Ok(())
}

async fn fetch_email_details(
    client: &Client,
    token: &str,
    message_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.googleapis.com/gmail/v1/users/me/messages/{}",
        message_id
    );

    let response = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<MessageDetails>()
        .await?;

    // Print the response snippet as soon as it's fetched
    println!("Fetched Email ID: {}, response: {:?}", message_id, response);

    Ok(response.snippet)
}


