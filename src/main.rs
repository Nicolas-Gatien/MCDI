use reqwest::{self, blocking::Response};
use std::fs;
use std::path::Path;
use std::io;
use whoami;
use regex::Regex;
use std::env;
use zip;


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

fn unzip_datapack(path: &String, destination: String) {
    let datapack_file = fs::File::open(&path).unwrap();
    let mut archive = zip::ZipArchive::new(datapack_file).unwrap();
    let base_destination: &Path = Path::new(&destination);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => base_destination.join(path),
            None => continue,
        };
        
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
        if (*file.name()).ends_with('/'){
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }

    }
}

fn delete_archive(path: &String) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
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
        let path = format!("{}/{}.zip", &destination, &datapack_name);

        let bytes = datapack.unwrap();
        fs::write(&path, &bytes).expect("Unable to write file");

        unzip_datapack(&path, destination);
        delete_archive(&path);
    } else {
        println!("Couldn't find Datapack: {}", datapack_name)
    }
}