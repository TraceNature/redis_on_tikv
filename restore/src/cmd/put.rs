use clap::{App, Arg};

pub fn new_put_cmd() -> App<'static> {
    clap::App::new("put")
        .about("put")
        .args(&[
            Arg::new("key").value_name("key").index(1).required(true),
            Arg::new("value").value_name("value").index(2).required(true),
            Arg::new("ttl").value_name("ttl").index(3)
        ])
}

