# Oauth CLI

This project uses clap, oauth2 and tinyhttp to create a baseplate for Rust CLIs using OAuth to authenticate.

It stores the fetched access token using the keychain crate, utilizing different secure storages depending on OS.

Oauth CLI needs an .env-file to build as it checks some variables like Auth Urls for consistensy before build.

See the included .env.template-file for an example of a Spotify implementation.

No client secret is needed since it uses the PKCE flow.


