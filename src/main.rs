mod get_user_input;

#[macro_use]
extern crate rocket;

use std::{env, net::{IpAddr, Ipv4Addr}};
use std::env::current_dir;
use rocket::{Config, fs::FileServer};
use get_user_input::{IOController, ArgsResolver, ResolverResult};

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     let port = IOController::get_port();
//     let root = IOController::get_root();
//
//     let configs = Config {
//         port,
//         address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)).into(),
//         ..Config::debug_default()
//     };
//
//     let _rocket = rocket::custom(configs)
//         .mount("/", FileServer::from(root))
//         .launch().await?;
//
//     return Ok(());
// }

fn main() {
    let mut port = 8000;
    let mut root = current_dir().unwrap();

    match ArgsResolver::try_port_and_root() {
        ResolverResult::Port(v) => {
            port = v;
            root = IOController::get_root();
            println!("1");
        }
        ResolverResult::Root(v) => {
            port = IOController::get_port();
            root = v;
            println!("2");
        }
        ResolverResult::Both(v) => {
            port = v.0;
            root = v.1;
            println!("3");
        }
        ResolverResult::Fail(v) => {
            port = IOController::get_port();
            root = IOController::get_root();
            println!("4");
        }
    }

    println!("final: port: {}, root: {:?}", port, root);
}