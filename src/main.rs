extern crate hyper;

use hyper::*;
use std::io;
use std::io::Read;

// fn get_api_endpoint(server_url: String) {
//     server_url.to_string() + "/api/";
// }

fn main() {
    println!("Rocket Chat!");

    // println!("Please input the server URL:");
    // let mut server_url = String::new();
    // io::stdin().read_line(&mut server_url)
    //     .expect("Failed to read line");

    let server_url : String = "https://chat.creators.co/".to_string();

    // println!("Please input an username:");
    // let mut user_username = String::new();
    // io::stdin().read_line(&mut user_username)
    //     .expect("Failed to read line");

    // println!("Please input the password:");
    // let mut user_password = String::new();
    // io::stdin().read_line(&mut user_password)
    //     .expect("Failed to read line");

    // println!("URL: {}, user: {}, password: {}", server_url, user_username, user_password)

    let client = Client::new();
    // let endpoint = get_api_endpoint(server_url);
    let endpoint = server_url + "/api/version";
    let mut res = client.get(&endpoint).send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("{}", s);
}