// extern crate redis;

use redis::{Commands, RedisResult, ToRedisArgs};
use crate::parser::{key_in_tikv, KeyType, ParserError};

pub struct redis_handler {
    conn: redis::Connection,
}


impl redis_handler {
    pub fn new(redisconn: redis::Connection) -> Self {
        Self { conn: redisconn }
    }
    pub fn ping(&mut self) -> RedisResult<()> {
        self.conn.send_packed_command(&*redis::cmd("PING").get_packed_command())
    }

    pub fn send_to_redis(&mut self, key: key_in_tikv) -> Result<(), ParserError> {
        match key.keytype() {
            KeyType::str => {
                let mut args = vec![];
                args.extend("SET".to_redis_args());
                args.extend(key.name().to_redis_args());
                args.extend(key.value().to_redis_args());
                let cmd = redis::pack_command(&args);
                self.conn.send_packed_command(&*cmd);
            }
            _ => { return Err(ParserError::OptionError("no key type match".to_string())); }
        }
        Ok(())
    }
}

fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://:redistest0102@114.67.76.82:16375/0")?;


    let mut con = client.get_connection()?;

    let _: () = con.set("my_key", 42)?;

    con.get("my_key")
}

#[cfg(test)]
mod test {
    use redis::Commands;
    use crate::source::redishandler::{fetch_an_integer, redis_handler};
    use crate::parser::Key_parser;

    #[test]
    fn test_fetch_an_integer() {
        let key_str = "*redis01_0_w_str1";

        let mut key_struct = Key_parser(key_str).unwrap();

        key_struct.set_value("key12".to_string());


        let client = redis::Client::open("redis://:redistest0102@114.67.76.82:16375/0");

        if let Err(e) = client {
            println!("{:?}", e);
            return;
        }

        let conn = client.unwrap().get_connection();
        if let Err(e) = conn {
            println!("{:?}", e);
            return;
        }
        let mut handler = redis_handler::new(conn.unwrap());
        handler.ping();
        let r = handler.send_to_redis(key_struct);

        println!("{:?}", r);


        let result = fetch_an_integer();
        println!("{:?}", result);
    }
}
