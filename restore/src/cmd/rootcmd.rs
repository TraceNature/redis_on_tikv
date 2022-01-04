use crate::cmd::requestsample::new_requestsample_cmd;
use crate::cmd::{get_ttl_cmd, new_config_cmd, new_get_cmd, new_put_cmd, new_remove_all_cmd, new_remove_cmd};
use crate::commons::CommandCompleter;
use crate::commons::SubCmd;

use crate::configure::set_config_file_path;
use crate::configure::{self, get_config, get_config_file_path};
use crate::request::{req, ReqResult, Request, RequestTaskListAll};
use crate::{configure::set_config, interact};
use clap::{App, AppSettings, Arg, ArgMatches};
use lazy_static::lazy_static;
use log::info;

use std::borrow::Borrow;
use std::{env, fs, thread};

use crate::cmd::loopcmd::new_loop_cmd;
use crate::cmd::restore::new_restore_cmd;
use crate::parser::{Key_parser, KeyType};
use crate::source::{redis_handler, TiKVHandler};
use chrono::prelude::Local;
use fork::{daemon, Fork};
use std::fs::File;
use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{System, SystemExt};

const APP_NAME: &str = "restore";

lazy_static! {
    static ref CLIAPP: clap::App<'static> = App::new(APP_NAME.clone())
        .version("1.0")
        .author("Shiwen Jia. <jiashiwen@gmail.com>")
        .about("command line sample")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                // .about("Sets a custom config file")
                // .takes_value(true)
        )
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .help("run as daemon")

        )
        .arg(
            Arg::new("interact")
                .short('i')
                .long("interact")
                .conflicts_with("daemon")
                .help("interact mod")
           )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .takes_value(true)

        )
        .subcommand(new_requestsample_cmd())
        .subcommand(new_config_cmd())
        .subcommand(new_put_cmd())
        .subcommand(new_remove_cmd())
        .subcommand(new_remove_all_cmd())
        .subcommand(new_get_cmd())
        .subcommand(get_ttl_cmd())
        .subcommand(new_restore_cmd())
        .subcommand(new_loop_cmd());
    static ref SUBCMDS: Vec<SubCmd> = subcommands();
}

pub fn run_app() {
    let matches = CLIAPP.clone().get_matches();
    if let Some(c) = matches.value_of("config") {
        println!("config path is:{}", c);
        set_config_file_path(c.to_string());
    }
    // set_config(&get_config_file_path());
    cmd_match(&matches);
}

pub fn run_from(args: Vec<String>) {
    match App::try_get_matches_from(CLIAPP.to_owned(), args.clone()) {
        Ok(matches) => {
            cmd_match(&matches);
        }
        Err(err) => {
            err.print().expect("Error writing Error");
        }
    };
}

// 获取全部子命令，用于构建commandcompleter
pub fn all_subcommand(app: &App, beginlevel: usize, input: &mut Vec<SubCmd>) {
    let nextlevel = beginlevel + 1;
    let mut subcmds = vec![];
    for iterm in app.get_subcommands() {
        subcmds.push(iterm.get_name().to_string());
        if iterm.has_subcommands() {
            all_subcommand(iterm, nextlevel, input);
        } else {
            if beginlevel == 0 {
                all_subcommand(iterm, nextlevel, input);
            }
        }
    }
    let subcommand = SubCmd {
        level: beginlevel,
        command_name: app.get_name().to_string(),
        subcommands: subcmds,
    };
    input.push(subcommand);
}

pub fn get_command_completer() -> CommandCompleter {
    CommandCompleter::new(SUBCMDS.to_vec())
}

fn subcommands() -> Vec<SubCmd> {
    let mut subcmds = vec![];
    all_subcommand(CLIAPP.clone().borrow(), 0, &mut subcmds);
    subcmds
}

pub fn process_exists(pid: &i32) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (syspid, _) in sys.processes() {
        if syspid == pid {
            return true;
        }
    }
    return false;
}

fn cmd_match(matches: &ArgMatches) {
    if matches.is_present("daemon") {
        let args: Vec<String> = env::args().collect();
        if let Ok(Fork::Child) = daemon(true, true) {
            // 启动子进程
            let mut cmd = Command::new(&args[0]);

            for idx in 1..args.len() {
                let arg = args.get(idx).expect("get cmd arg error!");
                // 去除后台启动参数,避免重复启动
                if arg.eq("-d") || arg.eq("-daemon") {
                    continue;
                }
                cmd.arg(arg);
            }

            let mut child = cmd.spawn().expect("Child process failed to start.");
            fs::write("pid", child.id().to_string());
            println!("process id is:{}", std::process::id());
            println!("child id is:{}", child.id());
        }
        println!("{}", "daemon mod");
        std::process::exit(0);
    }

    if matches.is_present("interact") {
        interact::run(APP_NAME.clone());
        return;
    }

    if let Some(ref matches) = matches.subcommand_matches("loop") {
        let term = Arc::new(AtomicBool::new(false));
        let sigint_2 = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();
        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&sigint_2)).unwrap();
        loop {
            if sigint_2.load(Ordering::Relaxed) {
                println!("{}", "singint signal recived");
                break;
            }
            // i += 1;
            // println!("i: {}", i);

            thread::sleep(Duration::from_millis(1000));
            if term.load(Ordering::Relaxed) {
                println!("{:?}", term);
                break;
            }
            let dt = Local::now();
            fs::write("timestamp", dt.timestamp_millis().to_string());
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("requestsample") {
        if let Some(_) = matches.subcommand_matches("baidu") {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let async_req = async {
                let result = req::get_baidu().await;
                println!("{:?}", result);
            };
            rt.block_on(async_req);
        };
    }

    if let Some(put) = matches.subcommand_matches("put") {
        let key = put.value_of("key").unwrap();
        let val = put.value_of("value").unwrap();

        let pdaddr = vec!["114.67.120.120:2379"];
        let rt = tokio::runtime::Runtime::new().unwrap();
        if !put.value_of("ttl").is_none() {
            let ttl = put.value_of("ttl").unwrap().parse::<u64>().unwrap();
            println!("ttl is {}", ttl);
            let future = async {
                let tikv_handler = TiKVHandler::new(pdaddr).await;
                tikv_handler
                    .tikv_put_with_ttl(key.to_string(), val.to_string(), ttl)
                    .await;
            };
            rt.block_on(future);
            return;
        }
        let future = async {
            let tikv_handler = TiKVHandler::new(pdaddr).await;

            tikv_handler
                .tikv_put(key.to_string(), val.to_string())
                .await;
        };
        rt.block_on(future);
    }

    if let Some(get) = matches.subcommand_matches("get") {
        let key = get.value_of("key").unwrap();
        let pdaddr = vec!["114.67.120.120:2379"];
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = async {
            let tikv_handler = TiKVHandler::new(pdaddr).await;
            let result = tikv_handler.tikv_get(key.to_string()).await;

            if let Some(val) = result.unwrap() {
                println!("get key:{},value is:{:?}", key, String::from_utf8(val));
            }
        };
        rt.block_on(future);
    }

    if let Some(remove) = matches.subcommand_matches("remove") {
        let key = remove.value_of("key").unwrap();
        let pdaddr = vec!["114.67.120.120:2379"];
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = async {
            let tikv_handler = TiKVHandler::new(pdaddr).await;
            tikv_handler.tikv_remove(key.to_string()).await;
        };
        rt.block_on(future);
    }

    if let Some(removeall) = matches.subcommand_matches("removeall") {
        let pdaddr = vec!["114.67.120.120:2379"];
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = async {
            let tikv_handler = TiKVHandler::new(pdaddr).await;
            // tikv_handler.tikv_remove(key.to_string()).await;
            tikv_handler.tikv_remove_all().await;
        };
        rt.block_on(future);
    }

    if let Some(ttl) = matches.subcommand_matches("ttl") {
        let key = ttl.value_of("key").unwrap();
        let pdaddr = vec!["114.67.120.120:2379"];
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = async {
            let tikv_handler = TiKVHandler::new(pdaddr).await;
            let result = tikv_handler.tikv_get_ttl_sec(key.to_string()).await;

            if let Some(val) = result.unwrap() {
                println!("get key:{},value is:{}", key, val);
            }
        };
        rt.block_on(future);
    }

    if let Some(restore) = matches.subcommand_matches("redisrestore") {
        let mut urlstr = "redis://".to_string();
        let mut host = "";
        let mut port = "";
        let mut password = "";
        let mut dbnumber = "";
        if let Some(pass) = restore.value_of("password") {
            urlstr.push_str(":");
            urlstr.push_str(pass);
            urlstr.push_str("@");
        }
        if let Some(addr) = restore.value_of("addr") {
            urlstr.push_str(addr);
        }

        if let Some(p) = restore.value_of("port") {
            urlstr.push_str(":");
            urlstr.push_str(p);
        }

        if let Some(db) = restore.value_of("db") {
            urlstr.push_str("/");
            urlstr.push_str(db);
        }

        println!("redis uri is:{}", urlstr);
        let client = redis::Client::open(urlstr);
        if let Err(e) = client {
            println!("{:?}", e);
            return;
        }

        // let conn = client
        //     .unwrap()
        //     .get_connection_with_timeout(Duration::from_secs(2));
        let conn = client
            .unwrap()
            .get_connection();

        if let Err(e) = conn {
            println!("{:?}", e);
            return;
        }

        let mut redishandler = redis_handler::new(conn.unwrap());
        if let Err(e) = redishandler.ping() {
            println!("{:?}", e);
            return;
        }

        //遍历instance_db
        //解析命令并写入
        let pdaddr = vec!["114.67.120.120:2379"];
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = async {
            let tikv_handler = TiKVHandler::new(pdaddr).await;
            let result = tikv_handler
                .prefix_scan("*redis01_0".to_string(), "*redis01_1".to_string(), 100)
                .await;

            if let Ok(kvs) = result {
                for pair in kvs.iter() {
                    let key = pair.clone().into_key();
                    let val = pair.clone().into_value();
                    let key_str = String::from_utf8(Vec::from(key)).unwrap();
                    println!("key string is {}", key_str);
                    let parsed = Key_parser(key_str.as_str());

                    if let Ok(mut k) = parsed {
                        match k.keytype() {
                            KeyType::str => {
                                k.set_value(String::from_utf8(val).unwrap());
                            }
                            _ => {}
                        }
                        redishandler.send_to_redis(k);
                    };
                    // println!("get key is :{:?} value is:{:?}", parsed, val);
                }
            }
        };
        rt.block_on(future);
    }

    if let Some(config) = matches.subcommand_matches("config") {
        if let Some(show) = config.subcommand_matches("show") {
            match show.subcommand_name() {
                Some("all") => {
                    println!("config show all");
                    info!("log show all");
                    configure::get_config_file_path();
                    println!("{:?}", configure::get_config());
                }
                Some("info") => {
                    println!("config show info");
                }
                _ => {}
            }
        }
    }
}
