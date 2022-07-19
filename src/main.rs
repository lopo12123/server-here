#[macro_use]
extern crate rocket;

use std::{
    env::current_dir,
    io::{stdin, stdout, Write},
    net::{IpAddr, Ipv4Addr},
};
use rocket::{Config, fs::FileServer};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // port
    let mut port = String::new();
    print!("PORT(default: 8000): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut port).ok();
    if port.trim().len() == 0 {
        port = String::from("8000");
    } else if port.ends_with("\r\n") {
        port = port[0..port.len() - 2].to_string();
    } else if port.ends_with("\n") {
        port = port[0..port.len() - 1].to_string();
    }

    // path string
    let mut root = String::new();
    print!("ROOT PATH(default: .): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut root).ok();

    // absolute path
    let mut root_path = current_dir().unwrap();
    let mut end = root.len();
    if root.ends_with("\r\n") {
        end -= 2;
    } else if root.ends_with("\n") {
        end -= 1;
    }
    root_path.push(&root[..end]);

    // setup server
    if !root_path.exists() {
        panic!("Root Path not exist.");
    } else {
        let configs = Config {
            port: port.parse::<u16>().unwrap(),
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).into(),
            ..Config::debug_default()
        };

        let _rocket = rocket::custom(configs)
            .mount("/", FileServer::from(root_path))
            .launch().await?;

        return Ok(());
    }
}