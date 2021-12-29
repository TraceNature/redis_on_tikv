use clap::App;
use clap::Arg;


pub fn new_restore_cmd() -> App<'static> {
    clap::App::new("redisrestore")
        .args(&[
            Arg::new("addr").value_name("addr").index(1).required(true),
            Arg::new("port").value_name("port").index(2).required(true),
            Arg::new("db").value_name("db").index(3).required(false),
            Arg::new("password").value_name("password").index(4).required(false),
        ])
        .about("redisrestore")
}
