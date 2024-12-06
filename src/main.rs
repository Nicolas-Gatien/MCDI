use reqwest::{self, blocking::Response};
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
