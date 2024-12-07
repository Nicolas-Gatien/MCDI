use reqwest::{self, blocking::Response};
use std::fs;
use whoami;
use regex::Regex;
use std::env;

fn minecraft_folder() -> String {
    let path: String = format!("/Users/{}/Library/Application Support/minecraft", whoami::username());
    return path;
}

fn get_current_world_name() -> String {
    let logpath: String = format!("{}/logs/latest.log", minecraft_folder());
    let contents: String = fs::read_to_string(&logpath).expect("Should have been able to read file");
    
    let re = Regex::new(r"ServerLevel\[(?<name>[^\]]+)\]").unwrap();
    match re.captures_iter(&contents).last() {
        Some(caps) => caps["name"].to_string(),
        None => "Nothing".to_string()
    }
}

fn fetch_datapack(name: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url: String = format!("http://138.197.135.52/api/get/{}", name);
    let response: Response = reqwest::blocking::get(url)?;

    if response.status().is_success() {
        let bytes = response.bytes()?.to_vec();
        Ok(bytes)
    } else {
        Err(format!("Failed to download file. Status: {}", response.status()).into())
    }
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let datapack_name = &args[2];

    if command != "install" {
        return;
    }

    let world_name = get_current_world_name();
    let destination = format!("{}/saves/{}/datapacks", minecraft_folder(), &world_name);

    println!("Fetching Datapack: {}", datapack_name);

    let datapack = fetch_datapack(datapack_name);

    if datapack.is_ok() {
        let path = format!("{}/datapack.zip", &destination);
        let bytes = datapack.unwrap();
        fs::write(&path, &bytes).expect("Unable to write file");
    } else {
        println!("Couldn't find Datapack: {}", datapack_name)
    }
}