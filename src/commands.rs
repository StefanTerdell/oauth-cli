pub mod auth;
pub mod completions;

use crate::APP_NAME;
use anyhow::*;
use keyring::Entry;
use reqwest::blocking::Client;
use serde_json::Value;

pub fn get_profile() -> Result<()> {
    let keyring =
        Entry::new(APP_NAME, whoami::username().as_str()).context("Getting keyring entry")?;

    let access_token = keyring
        .get_password()
        .unwrap_or_else(|err| panic!("Failed to retrieve token: {err}"));

    let client = Client::new();
    let res = client
        .get(env!("PROFILE_URL"))
        .bearer_auth(access_token)
        .send()
        .context("Sending profile request")?
        .error_for_status()
        .context("Error status returned")?
        .json::<Value>()
        .context("Decoding profile response");

    println!("{res:#?}");

    Ok(())
}
