extern crate hyper;
extern crate clap;

use hyper::*;
use std::io::Read;
use clap::{Arg, App};

fn get_api_endpoint(server_url: &str, method: &str) -> String {
    return server_url.to_string() + "/api/" + method;
}

struct ChatClient {
    url: String,
    username: String,
    password: String,
}

impl ChatClient {

    fn display_config(&self) {
        println!("Using server at: {}", self.url);
        println!("Using username: {}", self.username);
        println!("Using password: {}", self.password);
    }

    fn get_api_version(&self) {
        let client = Client::new();
        let version_endpoint = get_api_endpoint(&self.url, "version");
        let mut res = client.get(&version_endpoint).send().unwrap();
        assert_eq!(res.status, hyper::Ok);
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();
        println!("{}", s);
    }

}


fn main() {
    let matches =
        App::new("Rocket Chat client")
            .arg(Arg::with_name("server")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("username")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("password")
                .required(true)
                .takes_value(true))
            .get_matches();

    let server_url = matches.value_of("server").unwrap();
    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();
    
    let client = ChatClient {
        url: server_url.to_string(),
        username: username.to_string(),
        password: password.to_string(),
    };

    client.display_config();
    client.get_api_version();

}