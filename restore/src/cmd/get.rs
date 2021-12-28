use clap::{App, Arg};

pub fn new_get_cmd() -> App<'static> {
    clap::App::new("get")
        .about("get")
        .args(&[
            Arg::new("key").value_name("key").index(1).required(true),
        ])
}

pub fn get_ttl_cmd() -> App<'static> {
    clap::App::new("ttl")
        .about("ttl")
        .args(&[
            Arg::new("key").value_name("key").index(1).required(true),
        ])
}

