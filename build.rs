use dotenv::var;
use itertools::Itertools;
use oauth2::{AuthUrl, RedirectUrl, TokenUrl};
use url::Url;

pub fn main() {
    if cfg!(target_os = "linux") {
        println!("cargo::rustc-cfg=feature=\"linux\"");
    } else if cfg!(target_os = "macos") {
        println!("cargo::rustc-cfg=feature=\"apple\"");
    } else if cfg!(target_os = "windows") {
        println!("cargo::rustc-cfg=feature=\"windows\"");
    } else {
        panic!("Unsupported OS!");
    }

    let client_id = var("CLIENT_ID").expect("Missing build env CLIENT_ID");
    let auth_url = var("AUTH_URL").expect("Missing build env AUTH_URL");
    let token_url = var("TOKEN_URL").expect("Missing build env TOKEN_URL");
    let port = var("PORT").expect("Missing build env PORT");
    let scopes = var("SCOPES").expect("Missing build env SCOPES");
    let profile_url = var("PROFILE_URL").expect("Missing build env PROFILE_URL");

    let redirect_url = format!("http://localhost:{port}");

    AuthUrl::new(auth_url.clone())
        .unwrap_or_else(|err| panic!("Failed to create AuthUrl from {auth_url}: {err}"));

    TokenUrl::new(token_url.clone())
        .unwrap_or_else(|err| panic!("Failed to create TokenUrl from {token_url}: {err}"));

    RedirectUrl::new(redirect_url.clone())
        .unwrap_or_else(|err| panic!("Failed to create RedirectUrl from {redirect_url}: {err}"));

    Url::parse(&profile_url)
        .unwrap_or_else(|err| panic!("Failed to create Url from {profile_url}: {err}"));

    #[allow(unstable_name_collisions)]
    let scopes: String = scopes
        .split(&[',', ' '])
        .filter(|c| !c.is_empty())
        .intersperse(",")
        .collect();

    println!("cargo:rustc-env=CLIENT_ID={client_id}");
    println!("cargo:rustc-env=AUTH_URL={auth_url}");
    println!("cargo:rustc-env=TOKEN_URL={token_url}");
    println!("cargo:rustc-env=PORT={port}");
    println!("cargo:rustc-env=SCOPES={scopes}");
    println!("cargo:rustc-env=PROFILE_URL={profile_url}");
    println!("cargo:rustc-env=REDIRECT_URL={redirect_url}");
}
