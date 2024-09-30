use crate::APP_NAME;
use anyhow::*;
use keyring::Entry;
use oauth2::ClientId;
use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, CsrfToken,
    PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::str::FromStr;
use tiny_http::{Header, Response, Server};

pub fn login() -> Result<()> {
    let client_id = ClientId::new(env!("CLIENT_ID").to_string());
    let auth_url = AuthUrl::new(env!("AUTH_URL").to_string())
        .context("Failed to create AuthUrl. This should have been checked at build.")?;
    let token_url = TokenUrl::new(env!("TOKEN_URL").to_string())
        .context("Failed to create TokenUrl. This should have been checked at build.")?;

    let client =
        BasicClient::new(client_id, None, auth_url, Some(token_url))
            .set_redirect_uri(RedirectUrl::new(env!("REDIRECT_URL").to_string()).context(
                "Failed to create RedirectUrl. This should have been checked at build.",
            )?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(
            env!("SCOPES")
                .split(',')
                .map(|scope| Scope::new(scope.to_string())),
        )
        .set_pkce_challenge(pkce_challenge)
        .url();

    if webbrowser::open(authorize_url.as_str()).is_err() {
        println!("Please follow this link to login: {authorize_url}");
    }

    let server = Server::http(format!("0.0.0.0:{port}", port = env!("PORT")))
        .map_err(|err| anyhow!("Failed to launch server: {err}"))?;

    println!(
        "Listening on {addr} for the OAuth callback",
        addr = server.server_addr(),
    );

    let request = server
        .incoming_requests()
        .next()
        .context("Awaiting response")?;

    let query = request.url().split('?').nth(1).unwrap_or("");
    let params: std::collections::HashMap<_, _> = query
        .split('&')
        .map(|s| {
            let mut split = s.split('=');
            (split.next().unwrap_or(""), split.next().unwrap_or(""))
        })
        .collect();

    let code = AuthorizationCode::new(
        params
            .get("code")
            .context("Expected code in query")?
            .to_string(),
    );

    let response = Response::from_string(include_str!("../close.html")).with_header(
        Header::from_str("Content-Type: text/html").map_err(|_| anyhow!("Creating header"))?,
    );

    request.respond(response).context("Sending response")?;

    let token = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request(http_client)
        .context("Authentication response")?;

    let keyring =
        Entry::new(APP_NAME, whoami::username().as_str()).context("Creating keychain entry")?;
    keyring
        .set_password(token.access_token().secret())
        .context("Setting keychain entry")?;

    println!("Access token received and securely stored.");

    Ok(())
}

pub fn logout() -> Result<()> {
    let username = whoami::username();
    let keyring = Entry::new(APP_NAME, &username).context("Getting keychain entry")?;

    keyring.delete_credential()?;

    println!("Logged out successfully. Credentials removed.");

    Ok(())
}
