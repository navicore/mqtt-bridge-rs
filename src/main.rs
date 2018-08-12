#[macro_use]
extern crate serde_derive;
extern crate config;
extern crate mqttc;
extern crate netopt;
extern crate serde;
extern crate ws;

use mqttc::{PubOpt, PubSub};
use settings::Settings;
use std::str;
use std::sync::Arc;
use std::time;
mod settings;

fn new_client(cfg: settings::Mqtt) -> mqttc::Client {
    use mqttc::{ClientOptions, ReconnectMethod};
    use netopt::{NetworkOptions, SslContext};

    // Using ssl network connection
    let mut netopt = NetworkOptions::new();
    if cfg.tls {
        netopt.tls(SslContext::default());
    }

    // Using credentials for client
    let mut opts = ClientOptions::new();
    opts.set_username(cfg.username.to_string());
    opts.set_password(cfg.password.to_string());
    let timeout = time::Duration::from_secs(1_0000);
    opts.set_reconnect(ReconnectMethod::ReconnectAfter(timeout));

    let x = opts.connect(cfg.url, netopt).unwrap();
    x
}

fn main() {
    let settings = Settings::new().unwrap();
    let mut in_client = new_client(settings.in_client);

    in_client.subscribe(settings.in_topic.as_str()).ok();
    let mut out_client = new_client(settings.out_client);

    loop {
        match in_client.await() {
            Ok(Some(message)) => {
                println!("new msg on topic {:?}", &message.topic);
                match Arc::try_unwrap(message.payload) {
                    Ok(text) => {
                        let msg = str::from_utf8(&text).unwrap();
                        match out_client.publish(
                            settings.out_topic.clone(),
                            msg,
                            PubOpt::at_least_once(),
                        ) {
                            Ok(_) => (), //sent ok
                            Err(e) => panic!("publish err: {:?}", &e),
                        }
                    }
                    Err(err) => {
                        panic!("not ok = {:?}", &err);
                    }
                }
            }
            Ok(_) => (), // None timeout
            Err(_) => (panic!("unknown")),
        }
    }
}
