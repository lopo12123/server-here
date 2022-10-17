use std::env::current_dir;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

pub struct IOController {}

impl IOController {
    /// 接收输入作为端口参数
    /// 默认值 `8000`
    pub fn get_port() -> u16 {
        let mut port = String::new();
        println!("PORT(press enter to use default value <8000>): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut port).ok();

        if port.trim().len() == 0 {
            return 8000;
        } else if port.ends_with("\r\n") {
            return port[..port.len() - 2].parse::<u16>().unwrap();
        } else if port.ends_with("\n") {
            return port[..port.len() - 1].parse::<u16>().unwrap();
        }

        return 8000;
    }

    /// 接受输入作为根目录 (提供静态文件服务)
    /// 默认值 `.`
    pub fn get_root() -> PathBuf {
        let mut root = String::new();
        println!("ROOT(press enter to use default value <.>): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut root).ok();

        let mut root_path = current_dir().unwrap();
        let mut end = root.len();
        if root.ends_with("\r\n") {
            end -= 2;
        } else if root.ends_with("\n") {
            end -= 1;
        }

        root_path.push(&root[..end]);

        if !root_path.exists() {
            panic!("ROOT not exist!")
        }

        return root_path;
    }
}