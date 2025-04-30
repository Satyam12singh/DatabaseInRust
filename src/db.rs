use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader, Write};
use serde::{Serialize, Deserialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct Db<'a> {
    // memory inefficient
    // pub map: HashMap<String, String>,
    pub map:HashMap<Cow<'a, str>, Cow<'a, str>>,   
}

impl<'a> Db<'a> {
    pub fn new() -> Self {
        let mut db = Db {
            map: HashMap::new(),
        };
        db.load().unwrap_or_else(|_| ());
        db
    }

    pub fn set(&mut self, key: String, value: String) {
        // but this code is not memory safe as it takes more memory than required
        // self.map.insert(key.clone(), value.clone());
        self.map.insert(Cow::Owned(key.clone()), Cow::Owned(value));
        // let _ = self.append_log("SET", &key, Some(&value));
        if let Some((k, v)) = self.map.get_key_value(key.as_str()) {
            let _ = self.append_log("SET", k, Some(v));
        }
    }

    pub fn get(&self, key: &str) -> Option<&Cow<str>> {
        self.map.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.map.remove(key);
        let _ = self.append_log("DELETE", key, None);
    }

    fn append_log(&self, cmd: &str, key: &str, value: Option<&str>) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("db.log")?;

        let line = match value {
            Some(val) => format!("{} {} {}\n", cmd, key, val),
            None => format!("{} {}\n", cmd, key),
        };

        file.write_all(line.as_bytes())?;
        Ok(())
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        let file = File::open("db.log")?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            match parts.as_slice() {
                ["SET", key, val] => {
                    // self.map.insert((*key).to_string(), (*val).to_string());
                    self.map.insert(Cow::Owned((*key).to_string()), Cow::Owned((*val).to_string()));
                }
                ["DELETE", key] => {
                    self.map.remove(*key);
                }
                _ => continue,
            }
        }
        Ok(())
    }

    pub fn show_help() {
        println!("Usage:");
        println!("  > SET <key> <value>     - Sets a key-value pair.");
        println!("  > GET <key>            - Retrieves the value of a key.");
        println!("  > DELETE <key>         - Deletes a key-value pair.");
        println!("  > --help               - Displays this help message.");
    }
}
