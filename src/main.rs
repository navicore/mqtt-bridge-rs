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

fn new_client(mqtt_settings: settings::Mqtt) -> mqttc::Client {
    let url = mqtt_settings.url;
    let username = mqtt_settings.username;
    let password = mqtt_settings.password;
    let use_tls = mqtt_settings.tls;
    use mqttc::{ClientOptions, ReconnectMethod};
    use netopt::{NetworkOptions, SslContext};

    // Using ssl network connection
    let mut netopt = NetworkOptions::new();
    if use_tls {
        let ssl = SslContext::default();
        netopt.tls(ssl);
    }

    // Using credentials for client
    let mut opts = ClientOptions::new();
    opts.set_username(username.to_string());
    opts.set_password(password.to_string());
    let timeout = time::Duration::from_secs(1_0000);
    opts.set_reconnect(ReconnectMethod::ReconnectAfter(timeout));

    let x = opts.connect(url, netopt).unwrap();
    x
}

fn main() {
    let settings = Settings::new().unwrap();
    let mut in_client = new_client(settings.in_client);

    let in_topic = settings.in_topic;
    in_client.subscribe(in_topic.as_str()).ok();
    let mut out_client = new_client(settings.out_client);

    loop {
        match in_client.await() {
            Ok(Some(message)) => {
                println!("new msg on topic {:?}", &message.topic);
                match Arc::try_unwrap(message.payload) {
                    Ok(text) => {
                        let msg = str::from_utf8(&text).unwrap();
                        let out_topic = settings.out_topic.clone();
                        out_client
                            .publish(out_topic, msg, PubOpt::at_least_once())
                            .ok();
                    }
                    Err(err) => {
                        panic!("not ok = {:?}", &err);
                    }
                }
            }
            Ok(_) => (), // timeout
            Err(_) => (panic!("unknown")),
        }
    }
}
