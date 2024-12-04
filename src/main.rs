use std::{fs, io::Write};
use std::path::Path;

use std::error;
use reqwest::{self, header::{HeaderMap,  HeaderValue, COOKIE}};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
// https://github.com/OliverFlecke/advent-of-code-rust/blob/main/advent-of-code-client/src/lib.rs

fn main() -> Result<()> {
    // println!("Hello, world!");
    let file_path = Path::new("input/day01.txt");

    if file_path.exists() {
        println!("Input data is already present, skipping request.");
        return Ok(())
    }

    let url: String = String::from("https://adventofcode.com/2024/day/1/input");
    let token: String = String::from("<session_token>");
    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .default_headers({
            let mut headers = HeaderMap::new();
            headers.insert(
                COOKIE, 
                HeaderValue::from_str(&format!("session={token}"))
                        .expect("Failed to make header value with token"),
                    );
            headers
        })
        .user_agent("jsnlund")
        .build()
        .expect("Failed to create reqwest client");

    let body = client.get(&url).send()?.text()?;
    // let body = reqwest::blocking::get(&url)?.text()?;
    // println!("body = {body:?}");
    println!("reqwest complete, data retrieved");
    
    let mut file = fs::File::create(&file_path)?;
    file.write_all(&body.as_bytes())?;


    Ok(())
}
