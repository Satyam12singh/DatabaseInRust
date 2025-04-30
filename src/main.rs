mod db;
use db::Db;
use std::io::{self, Write};

fn main() {
    let mut db = Db::new();

    println!("📦 Welcome to MyDB (type EXIT to quit)");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens.as_slice() {
            ["EXIT"] => break,
            ["--help"] =>{
                Db::show_help();
                
            }
            ["SET", key, value] => {
                db.set(key.to_string(), value.to_string());
                println!("✅ OK");
            }
            ["GET", key] => {
                match db.get(key) {
                    Some(val) => println!("🔍 {}", val),
                    None => println!("❌ Not Found"),
                }
            }
            ["DELETE", key] => {
                db.delete(key);
                println!("🗑️ Deleted (if existed)");
            }
            _ => println!("❓ Unknown command"),
        }
    }
}
