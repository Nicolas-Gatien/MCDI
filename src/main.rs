use reqwest::{self, blocking::Response};
use std::fs::{self, File};
use std::io::Write;
use whoami;
use regex::Regex;

fn minecraft_folder() -> String {
    let path: String = format!("/Users/{}/Library/Application Support/minecraft", whoami::username());
    return path;
}

fn get_path_minecraft_world() -> String {
    let logpath: String = format!("{}/logs/latest.log", minecraft_folder());
    let contents: String = fs::read_to_string(&logpath).expect("Should have been able to read file");
    
    let re = Regex::new(r"ServerLevel\[(?<name>[^\]]+)\]").unwrap();
    match re.captures_iter(&contents).last() {
        Some(caps) => caps["name"].to_string(),
        None => "Nothing".to_string()
    }
}

fn fetch_datapack() -> Result<(), Box<dyn std::error::Error>> {
    let url: &str = "http://138.197.135.52/api/get/cur";
    let response: Response = reqwest::blocking::get(url)?;

    if response.status().is_success() {
        let bytes = response.bytes()?;
        let filename = "downloaded.zip";
        let mut file = File::create(&filename)?;
        file.write_all(&bytes)?;

        println!("File download successful! {}", &filename);
    } else {
        println!("Failed to download file. Status: {}", response.status());
    }

    Ok(())
} 

fn main() {
    let current_world_path = get_path_minecraft_world();
    let destination = format!("{}/saves/{}/datapacks", minecraft_folder(), &current_world_path);

    println!("{}", &destination);
}