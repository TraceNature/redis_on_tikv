use nom::bytes::complete::{tag, take_until};
use std::error::Error;

use nom::IResult;

#[derive(Debug)]
pub enum ParserError {
    OptionError(String),
}

pub type Result<T, E = ParserError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum KeyType {
    str,
    list,
    set,
    zset,
    hash,
}

impl KeyType {
    pub fn New(s: &str) -> Result<KeyType> {
        return match s {
            "w" => Ok(KeyType::str),
            "l" => Ok(KeyType::list),
            "s" => Ok(KeyType::set),
            "z" => Ok(KeyType::zset),
            "h" => Ok(KeyType::hash),
            _ => Err(ParserError::OptionError(String::from(
                "key type not exists",
            ))),
        };
    }
}

#[derive(Debug)]
pub struct key_in_tikv {
    instance: String,
    db: usize,
    keytype: KeyType,
    reverse: bool,
    name: String,
    value: String,
    field: String,
    index: usize,
    score: f64,
    number: String,
}

impl key_in_tikv {
    pub fn instance(&self) -> &str {
        &self.instance
    }
    pub fn db(&self) -> usize {
        self.db
    }
    pub fn keytype(&self) -> &KeyType {
        &self.keytype
    }
    pub fn reverse(&self) -> bool {
        self.reverse
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn field(&self) -> &str {
        &self.field
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn number(&self) -> &str {
        &self.number
    }
    pub fn score(&self) -> f64 {
        self.score
    }
}

impl key_in_tikv {
    pub fn default() -> Self {
        Self {
            instance: "".to_string(),
            db: 0,
            keytype: KeyType::str,
            reverse: false,
            index: 0,
            name: "".to_string(),
            value: "".to_string(),
            field: "".to_string(),
            score: 0.0,
            number: "".to_string(),
        }
    }

    pub fn set_value(&mut self, s: String) {
        self.value = s;
    }

    pub fn set_score(&mut self, score: f64) {
        self.score = score;
    }
}


pub fn Key_parser(keystr: &str) -> Result<key_in_tikv> {
    //解析key head
    let (body, _) = header_asterisk(keystr).map_err(|e| {
        return ParserError::OptionError(e.to_string());
    })?;

    //解析instance name
    let (content, instance) = until_slash(body).map_err(|e| {
        return ParserError::OptionError(e.to_string());
    })?;

    //解析 db number
    let (content, _) = header_slash(content).map_err(|e| {
        return ParserError::OptionError(e.to_string());
    })?;
    let (content, db) = until_slash(content).map_err(|e| {
        return ParserError::OptionError(e.to_string());
    })?;

    //解析 key 类型
    let (content, _) = header_slash(content).map_err(|e| {
        return ParserError::OptionError(e.to_string());
    })?;
    let (content, key_type) = until_slash(content).map_err(|e| {
        return ParserError::OptionError(e.to_string());
    })?;


    let (content, _) = header_slash(content).map_err(|e| {
        return ParserError::OptionError(e.to_string());
    })?;

    let mut key = key_in_tikv::default();

    key.instance = String::from(instance);
    key.db = db.parse::<usize>().map_err(|e| {
        return ParserError::OptionError(e.to_string());
    })?;

    key.keytype = KeyType::New(key_type)?;

    match key.keytype {
        KeyType::str => {
            key.name = String::from(content);
        }
        KeyType::set => {
            let (content, key_name) = until_slash(content).map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;
            let (val, _) = header_slash(content).map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;
            key.name = String::from(key_name);
            key.value = String::from(val);
        }

        KeyType::zset => {
            let (content, key_name) = until_slash(content).map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;
            let (mumber, _) = header_slash(content).map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;
            key.name = String::from(key_name);
            key.value = String::from(mumber);
        }
        KeyType::list => {
            //解析key 名称
            let (content, key_name) = until_slash(content).map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;

            //解析 是否为反向序列
            let (content, _) = header_slash(content).map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;
            let (content, reverse) = until_slash(content).map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;
            // 解析 list index
            let (index, _) = header_slash(content).map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;

            key.name = String::from(key_name);
            if reverse.eq("1") {
                key.reverse = true;
            }
            key.index = index.parse::<usize>().map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;
        }

        KeyType::hash => {
            let (content, key_name) = until_slash(content).map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;
            let (field, _) = header_slash(content).map_err(|e| {
                return ParserError::OptionError(e.to_string());
            })?;


            key.name = String::from(key_name);
            key.field = String::from(field);
        }
        _ => {
            return Err(ParserError::OptionError("no key type match".to_string()));
        }
    }

    return Ok(key);
}

fn header_asterisk(s: &str) -> IResult<&str, &str> {
    tag("*")(s)
}

fn header_slash(s: &str) -> IResult<&str, &str> {
    tag("_")(s)
}

fn until_slash(s: &str) -> IResult<&str, &str> {
    take_until("_")(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key_parser() {
        let key_str = "*redis01_0_w_str1";
        let key_list = "*redis02_2_l_list1_0_100";
        let key_set = "*redis03_3_s_set01_setval";
        let key_zset = "*redis04_4_z_zset01_zsetval";
        let key_hash = "*redis05_5_h_hash01_field01";
        let result_str = Key_parser(key_str);
        let result_list = Key_parser(key_list);
        let result_set = Key_parser(key_set);
        let result_zset = Key_parser(key_zset);
        let result_hash = Key_parser(key_hash);
        println!("{:?}", result_str);
        println!("{:?}", result_list);
        println!("{:?}", result_set);
        println!("{:?}", result_zset);
        println!("{:?}", result_hash);
    }
}
