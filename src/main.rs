extern crate hyper;
extern crate clap;

use hyper::*;
use std::io;
use std::io::Read;
use clap::{Arg, App, SubCommand};


// fn get_api_endpoint(server_url: String) {
//     server_url.to_string() + "/api/";
// }

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

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)

    let server_url = matches.value_of("server").unwrap();
    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();
    
    println!("Using server at: {}", &server_url);
    println!("Using username: {}", matches.value_of("username").unwrap());
    println!("Using password: {}", matches.value_of("password").unwrap());

    // println!("URL: {}, user: {}, password: {}", server_url, user_username, user_password)

    let client = Client::new();
    // let endpoint = get_api_endpoint(server_url);
    let endpoint = server_url.to_string() + "/api/version";
    let mut res = client.get(&endpoint).send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("{}", s);
}