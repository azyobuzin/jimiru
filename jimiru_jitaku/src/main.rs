#[macro_use]
extern crate clap;
extern crate jimiru;
extern crate libc;
extern crate native_tls;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod config;
mod udp_socket;

use clap::{App, Arg, SubCommand};
use config::*;

fn main() {
    let app_m = App::new("jimiru_jitaku")
        .version(crate_version!())
        .arg(Arg::with_name("config")
            .long("config")
            .help("設定ファイル")
            .takes_value(true)
            .use_delimiter(false)
            .global(true)
            .env("JIMIRU_JITAKU_CONFIG"))
        .subcommand(SubCommand::with_name("wake")
            .arg(Arg::with_name("machines")
                .help("起動するマシン名")
                .multiple(true))
            .arg(Arg::with_name("machines_all")
                .help("すべてのマシンを対象にする")
                .long("all")))
        .get_matches();

    let config =
        match app_m.value_of_os("config") {
            Some(config_file) => {
                match read_config_file(config_file) {
                    Ok(x) => x,
                    Err(x) => {
                        eprintln!("設定ファイルを読み込めませんでした。\n{}", x);
                        return;
                    }
                }
            },
            None => {
                eprintln!("設定ファイルが指定されていません。");
                return;
            }
        };

    match app_m.subcommand() {
        ("", None) => {
            // サーバー接続
            println!("これから実装します");
            println!("{:?}", config);
        }
        ("wake", Some(sub_m)) => {
            // マジックパケット送信
        }
        _ => unreachable!(),
    }
}
