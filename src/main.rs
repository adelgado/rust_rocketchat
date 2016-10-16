//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]

#[macro_use] extern crate hyper;
extern crate clap;
extern crate serde_json;

use hyper::*;
use hyper::header::{Headers, ContentType};
use std::io::Read;
use clap::{Arg, App};
use serde_json::Value;

fn get_api_endpoint(server_url: &str, method: &str) -> String {
    return server_url.to_string() + "api/" + method;
}

struct ChatAuth {
    auth_token: String,
    user_id: String,
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
        let endpoint = get_api_endpoint(&self.url, "version");

        let client = Client::new();
        let mut res = client.get(&endpoint).send().unwrap();
        assert_eq!(res.status, hyper::Ok);
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();
        println!("{}", s);
    }

    fn login(&self) -> ChatAuth {
        let endpoint = get_api_endpoint(&self.url, "login");

        let client = Client::new();
        let body = format!("user={}&password={}", self.username, self.password);
        let mut headers = Headers::new();
        headers.set(ContentType::form_url_encoded());
        let mut res = client.post(&endpoint)
            .headers(headers)
            .body(&body).send().unwrap();
        assert_eq!(res.status, hyper::Ok);
        let mut response_content = String::new();
        res.read_to_string(&mut response_content).unwrap();
        println!("{}", response_content);

        let response_json: Value =
            serde_json::from_str(&response_content).unwrap();
         
        let data = response_json.as_object().unwrap()
            .get("data").unwrap()
            .as_object().unwrap();
         
        return ChatAuth {
            auth_token: String::from(data.get("authToken").unwrap().to_string().trim_matches('"')),
            user_id: String::from(data.get("userId").unwrap().to_string().trim_matches('"')),
        };

    }

    fn get_public_rooms(&self) {
        let endpoint = get_api_endpoint(&self.url, "publicRooms");
        let chat_auth = self.login();
        let client = Client::new();

        header! { (XAuthToken, "X-Auth-Token") => [String] }
        header! { (XUserId, "X-User-Id") => [String] }

        let mut headers = Headers::new();
        headers.set(XAuthToken(chat_auth.auth_token.to_string().to_owned()));
        headers.set(XUserId(chat_auth.user_id.to_string().to_owned()));

        println!("{}", headers);

        let mut res = client.get(&endpoint)
            .headers(headers)
            .send()
            .unwrap();
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();
        println!("{}", s);
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
    client.login();
    client.get_public_rooms();

}