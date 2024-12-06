extern crate reqwest;

use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::get("http://138.197.135.52/api/get/cur")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
