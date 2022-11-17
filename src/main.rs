mod get_user_input;

#[macro_use]
extern crate rocket;

use std::{env, net::{IpAddr, Ipv4Addr}};
use rocket::{Config, fs::FileServer};
use get_user_input::{IOController, ArgsResolver};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let (port, root) = ArgsResolver::try_port_and_root();

    let port = match port {
        Some(v) => v,
        None => IOController::get_port()
    };
    let root = match root {
        Some(v) => v,
        None => IOController::get_root()
    };

    let configs = Config {
        port,
        address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)).into(),
        ..Config::debug_default()
    };

    let _rocket = rocket::custom(configs)
        .mount("/", FileServer::from(root))
        .launch().await?;

    return Ok(());
}