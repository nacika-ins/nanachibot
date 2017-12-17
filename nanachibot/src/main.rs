extern crate mammut;
use mammut::Registration;
use mammut::apps::{AppBuilder, Scope};
use mammut::Mastodon;
use std::fs::File;
use std::io::prelude::*;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate url;

mod bot;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    global_string: Option<String>,
    global_integer: Option<u64>,
    app: Option<AppConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AppConfig {
    client_id: Option<String>,
    client_secret: Option<String>,
    redirect: Option<String>,
    authorize_code: Option<String>,
    access_token: Option<String>
}

enum MODE{
    None,
    Register,
    GetAuthorizeCode,
    Ready
}

fn main() {
    try().expect("main");
}

fn try() -> mammut::Result<()> {

    let app = get_app();
    let mut config = get_config();
    let mut registration = Registration::new("https://oransns.com");
    #[allow(unused_assignments)]
    let mut mode = MODE::None;

    // Check if config is set, if not configure
    match config.app {
        Some(ref mut app_config) if app_config.client_id.is_none() => {
            println!("---> not found client_id");
            println!("---> generate client_id");
            registration.register(app)?;
            println!("---> get data for save");
            app_config.client_id = registration.client_id.clone();
            app_config.client_secret = registration.client_secret.clone();
            app_config.redirect = registration.redirect.clone();
            let url = registration.authorise()?;
            println!("---> please access to url: '{}'", url);
            println!("After accessing the URL, put the displayed authorize_code in config.toml");
            mode = MODE::Register;
        }
        Some(ref mut app_config) if app_config.access_token.is_some() => {
            println!("---> found access_token");
            mode = MODE::Ready;
        }
        _ => {
            println!("--> found cliend_id");
            mode = MODE::GetAuthorizeCode;
        }
    }

    match mode {

        // When the mode is changed to REGISTER, save processing is performed
        MODE::Register => {
            save_config(&config);
        }
        MODE::GetAuthorizeCode => {
            registration.client_id = config.app.as_ref().expect("client_id").client_id.clone();
            registration.client_secret = config.app.as_ref().expect("client_secret").client_secret.clone();
            registration.redirect = config.app.as_ref().expect("redirect").redirect.clone();
            let authorize_code: String = match config.app.as_ref().expect("authorize_code").authorize_code.clone() {
                Some(ref authorize_code) if authorize_code == "" => { panic!("authorize_code is blank") }
                Some(authorize_code) => { authorize_code }
                None => { panic!("authorize_code is not included in config.toml") }
            };

            // Here you now need to open the url in the browser
            // And handle a the redirect url coming back with the code.
            println!("authorize_code ---> {}", authorize_code);
            let code = authorize_code;

            let mastodon: Mastodon = registration.create_access_token(code)?;
            let access_token = mastodon.data.token;
            match config.app {
                Some(ref mut app_config) => {
                    app_config.access_token = Some(access_token.into_owned());
                }
                _ => { panic!() }
            }
            save_config(&config);
        }
        MODE::Ready => {

            registration.client_id = config.app.as_ref().expect("client_id").client_id.clone();
            registration.client_secret = config.app.as_ref().expect("client_secret").client_secret.clone();
            registration.redirect = config.app.as_ref().expect("redirect").redirect.clone();
            let access_token = config.app.as_ref().expect("access_token").access_token.clone().expect("access_token");
            let mastodon: Mastodon = registration.set_access_token(access_token)?;

            println!("---> ready");
            bot::exec(&mastodon);

        }
        _ => {}
    }

    Ok(())
}


// Get App
fn get_app<'a>() -> AppBuilder<'a> {
    AppBuilder {
        client_name: "Abyss",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scope::All,
        website: None,
    }
}

// Save Config file
fn save_config(config: &Config) -> () {

    let t = toml::to_string(&*config).expect("toml to string");
    let mut config_file: std::fs::File = File::create(&std::path::Path::new("config.toml")).expect("config.toml");
    match config_file.write_all(t.as_bytes()) {
        Ok(_) => {
            println!("success")
        }
        Err(e) => {
            println!("fail: {}", e)
        }
    };
    drop(config_file);
}

// Open Config file
fn get_config() -> Config {

    let mut config_file: std::fs::File = match File::open(&std::path::Path::new("config.toml")) {
        Ok(config_file) => { println!("Opened File"); config_file }
        Err(e) => File::create(&std::path::Path::new("config.toml")).expect("config.toml")
    };
    let mut config_text = String::new();
    #[allow(unused_must_use)]
    config_file.read_to_string(&mut config_text);
    let mut config: Config = toml::from_str(&config_text).expect("config_text");
    if config.app.is_none() {
        config.app = Some(AppConfig {
            client_id: None,
            client_secret: None,
            authorize_code: Some("".to_owned()),
            access_token: None,
            redirect: None
        })
    }
    drop(config_file);
    config
}
