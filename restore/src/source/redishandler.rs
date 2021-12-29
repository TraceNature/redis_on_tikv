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
                self.conn.set(key.name(), key.value()).map_err(|e| {
                    return ParserError::OptionError(e.to_string());
                })?;
            }
            KeyType::set => {
                let mut args = vec![];
                args.extend("SADD".to_redis_args());
                args.extend(key.name().to_redis_args());
                args.extend(key.value().to_redis_args());
                let cmd = redis::pack_command(&args);
                self.conn.send_packed_command(&*cmd).map_err(|e| {
                    return ParserError::OptionError(e.to_string());
                })?;
            }
            KeyType::zset => {
                let mut args = vec![];
                args.extend("ZADD".to_redis_args());
                args.extend(key.name().to_redis_args());
                args.extend(key.score().to_redis_args());
                args.extend(key.number().to_redis_args());
                let cmd = redis::pack_command(&args);
                self.conn.send_packed_command(&*cmd).map_err(|e| {
                    return ParserError::OptionError(e.to_string());
                })?;
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
    use std::time::Duration;
    use redis::Commands;
    use crate::source::redishandler::{fetch_an_integer, redis_handler};
    use crate::parser::Key_parser;

    #[test]
    fn test_fetch_an_integer() {
        let client = redis::Client::open("redis://:redistes0102@114.67.76.82:16375/0");
        if let Err(e) = client {
            println!("{:?}", e);
            return;
        }
        // let conn = client.unwrap().get_connection();
        let conn = client.unwrap().get_connection_with_timeout(Duration::from_secs(2));
        if let Err(e) = conn {
            println!("{:?}", e);
            return;
        }
        let mut handler = redis_handler::new(conn.unwrap());
        handler.ping();
        let key_str = "*redis01_0_w_str1";
        let key_set = "*redis03_3_s_set01_setval";
        let key_zset = "*redis04_4_z_zset01_zsetval";

        //解析string类型的key
        let mut key_struct_str = Key_parser(key_str).unwrap();
        //设置key value
        key_struct_str.set_value("key12".to_string());

        //解析set类型key
        let mut key_struct_set = Key_parser(key_set).unwrap();

        //解析zset key
        let mut key_struct_zset = Key_parser(key_zset).unwrap();
        key_struct_zset.set_score(6.66_f64);


        let r = handler.send_to_redis(key_struct_str);
        println!("put str: {:?}", r);
        let r = handler.send_to_redis(key_struct_set);
        println!("put set: {:?}", r);
        let r = handler.send_to_redis(key_struct_zset);
        println!("put zset: {:?}", r);

        let result = fetch_an_integer();
        println!("{:?}", result);
    }
}
