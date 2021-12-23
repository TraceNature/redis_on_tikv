use logger::init_log;

mod cmd;
mod commons;
mod configure;
mod interact;
mod logger;
mod request;
mod parser;
mod source;

fn main() {
    init_log();
    cmd::run_app();
}
