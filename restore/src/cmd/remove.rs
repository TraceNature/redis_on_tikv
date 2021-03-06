use clap::{App, Arg};

pub fn new_remove_cmd() -> App<'static> {
    clap::App::new("remove")
        .about("remove")
        .args(&[
            Arg::new("key").value_name("key").index(1).required(true),
        ])
}

pub fn new_remove_all_cmd() -> App<'static> {
    clap::App::new("removeall")
        .about("remove all keys")
}

