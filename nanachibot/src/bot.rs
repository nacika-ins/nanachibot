use regex::Regex;
use mammut::Mastodon;
use mammut::StatusBuilder;
use mammut::entities::notification::NotificationType;
use mammut::entities::prelude::*;
use std::{thread, time};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::ops::Shr;
use url::Url;

extern crate curl;
use std::io::{stdout, Write};
use self::curl::easy::Easy;



enum BotCommand {
    Reply,
    Keyword
}

struct BotAction {
    command: BotCommand,
    message: Option<String>,
    from_status: Status,
    from_account: Account,
    get_url: Option<String>

}


extern crate nanachi;
use self::nanachi::application::service::nanachi::NanachiApplicationService;

pub fn exec(mastodon: &Mastodon) -> Result<(), String> {

    let nanachi = NanachiApplicationService::new("nanachi.db");
    nanachi.nanachi.create_table();
    let nanachi_arc = Arc::new(Mutex::new(nanachi));
    let nanachi_1 = nanachi_arc.clone();
    let nanachi_2 = nanachi_arc.clone();

    let re_s = Regex::new(r"/s (?P<text>[^<]*)").expect("re");
    let re_w = Regex::new(r"/w (?P<text>[^<]*)").expect("re");

    let (tx, rx) = channel();
    let tx_1 = tx.clone();
    let tx_2 = tx.clone();

    let (srx, nrx) = mastodon.get_user_streaming();

    let one_sec = time::Duration::from_millis(1000);

    // Status queue
    thread::spawn(move || {
        let nanachi = nanachi_1.clone();
        loop {
            let nanachi = nanachi.clone();
            match srx.recv() {
                Ok(status) => {
                    let nanachi = match nanachi.lock() {
                        Ok(nanachi) => nanachi,
                        Err(err) => err.into_inner(),
                    };
                    let status: Status = status;
                    println!("{:?}", status);
                    let message = status.content.clone();
                    if message.contains("ナナチbot") {
                        let comment = nanachi.post_comment(&message);
                        println!("\nmessage => {:?}", message);
                        println!("comment => {:?}", comment);
                        match comment {
                            Ok(comment) => {
                                tx_2.send(BotAction {
                                    command: BotCommand::Keyword,
                                    message: Some(comment),
                                    from_status: status.clone(),
                                    from_account: status.account.clone(),
                                    get_url: None
                                });
                            }
                            _ => {
                                tx_2.send(BotAction {
                                    command: BotCommand::Keyword,
                                    message: Some("んなぁ〜".to_owned()),
                                    from_status: status.clone(),
                                    from_account: status.account.clone(),
                                    get_url: None
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    });

    // Notification queue
    thread::spawn(move || {
        let nanachi = nanachi_2.clone();
        loop {
            let nanachi = nanachi.clone();
            match nrx.recv() {
                Ok(notification) => {
                    let nanachi = match nanachi.lock() {
                        Ok(nanachi) => nanachi,
                        Err(err) => err.into_inner(),
                    };
                    let notification: Notification = notification;
                    match notification.notification_type {
                        NotificationType::Mention => {
                            if notification.status.as_ref().unwrap().content.contains("んなぁ〜") {
                                tx_1.send(BotAction {
                                    command: BotCommand::Reply,
                                    message: Some("んなぁ〜".to_owned()),
                                    from_status: notification.status.clone().unwrap(),
                                    from_account: notification.account.clone(),
                                    get_url: None
                                });
                            } else {

                                let message = notification.status.as_ref().unwrap().content.clone();
                                match true {
                                    _ if re_s.captures(&message).is_some() => {
                                        let caps_s = re_s.captures(&message).unwrap();
                                        let text = caps_s.name("text").expect("re").as_str();
                                        println!("text => {}", text);
                                        nanachi.sentense_input(text);
                                        tx_1.send(BotAction {
                                            command: BotCommand::Reply,
                                            message: Some("んなぁ！".to_owned()),
                                            from_status: notification.status.clone().unwrap(),
                                            from_account: notification.account.clone(),
                                            get_url: None
                                        });
                                    }
                                    _ if re_w.captures(&message).is_some() => {
                                        let caps_w = re_w.captures(&message).unwrap();
                                        let text = caps_w.name("text").expect("re").as_str();
                                        println!("text => {}", text);
                                        nanachi.word_input(text);
                                        tx_1.send(BotAction {
                                            command: BotCommand::Reply,
                                            message: Some("んなぁ！".to_owned()),
                                            from_status: notification.status.clone().unwrap(),
                                            from_account: notification.account.clone(),
                                            get_url: None
                                        });
                                    }
                                    _ => {
                                        let comment = nanachi.post_comment(&message);
                                        println!("\nmessage => {:?}", message);
                                        println!("comment => {:?}", comment);
                                        match comment {
                                            Ok(comment) => {
                                                tx_1.send(BotAction {
                                                    command: BotCommand::Reply,
                                                    message: Some(comment),
                                                    from_status: notification.status.clone().unwrap(),
                                                    from_account: notification.account.clone(),
                                                    get_url: None
                                                });
                                            }
                                            _ => {
                                                tx_1.send(BotAction {
                                                    command: BotCommand::Reply,
                                                    message: Some("んなぁ〜".to_owned()),
                                                    from_status: notification.status.clone().unwrap(),
                                                    from_account: notification.account.clone(),
                                                    get_url: None
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                        }
                    }

                    println!("{:?}", notification);
                }
                _ => {}
            }
        }
    });

    // Mastodon queue
    loop {
        match rx.recv() {
            Ok(action) => {
                let action: BotAction = action;

                let user_name = action.from_account.username.clone();
                let uri = action.from_account.url.clone();
                let domain = Url::parse(&uri).unwrap();
                let domain = domain.host_str().unwrap();
                let user_id = format!("@{}@{}", user_name, domain);
                let message = action.message.unwrap_or("".to_owned());
                let get_url = action.get_url;

                match action.command {
                    BotCommand::Reply => {
                        let mut status_b = StatusBuilder::new(format!("{} {}", user_id, message));
                        status_b.in_reply_to_id = Some(action.from_status.id);
                        mastodon.new_status(status_b);
                    }
                    BotCommand::Keyword => {
                        let mut status_b = StatusBuilder::new(format!("{}", message));
                        mastodon.new_status(status_b);
                    }
                }

                get_url.and_then( |url| {
                    let mut easy = Easy::new();
                    easy.url(&url).and_then( |_| easy.perform() );
                    Some(true)
                });
            }
            _ => {}
        }
    }
    Ok(())
}
