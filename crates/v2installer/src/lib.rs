use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};
use symlink::symlink_file;
use zip::ZipArchive;

pub fn v2installer() {
    // Step 1: Create v2ray dir
    let dir_path = Path::new("C:/v2ray");
    if dir_path.exists() {
        let dirs = fs::read_dir(dir_path).expect("Failed to read v2ray dir");
        for entry in dirs {
            let entry = entry.expect("Failed to read v2ray dir entry");
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(&path)
                    .expect(&format!("Failed to remove {}", path.to_string_lossy()));
            } else {
                fs::remove_dir_all(&path)
                    .expect(&format!("Failed to remove {}", path.to_string_lossy()))
            }
        }
    } else {
        fs::create_dir(dir_path).expect("Failed to create v2ray dir");
    }

    println!("Getting info about latest release");

    // Step 2: Download v2ray
    let v2ray_zip_url = get_latest_download_url().expect("Failed to get latest release url");
    let v2ray_zip_path = format!("{}/v2ray.zip", dir_path.to_str().unwrap());

    println!("Downloading v2ray from {}", v2ray_zip_url);

    let client = Client::new();
    let mut response = client.get(v2ray_zip_url).send().unwrap();

    let total_size = response.content_length().unwrap();

    let pb = ProgressBar::new(total_size);

    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap().progress_chars("#>-")
    );

    let mut file = fs::File::create(&v2ray_zip_path).unwrap();

    let mut buffer = [0, 0];

    loop {
        let read_size = response.read(&mut buffer).unwrap();
        if read_size == 0 {
            break;
        }
        file.write_all(&buffer[..read_size]).unwrap();
        pb.inc(read_size as u64);
    }

    pb.finish_with_message("Download completed!");

    println!("Unzipping v2ray.zip");
    let zip_file = fs::File::open(&v2ray_zip_path).unwrap();
    let mut archive = ZipArchive::new(zip_file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let relative_path = Path::new(&file.name()).iter().skip(1).collect::<PathBuf>();
        let out_path = dir_path.clone().join(relative_path);

        if file.name().ends_with('/') {
            fs::create_dir_all(&out_path).unwrap();
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(&parent).unwrap()
            }
            let mut out_file = fs::File::create(&out_path).unwrap();
            io::copy(&mut file, &mut out_file).unwrap();
        }
    }
    println!("Unzipped v2ray.zip completed");

    fs::remove_file(v2ray_zip_path).unwrap();

    println!("Created desktop shortcut");

    let exe_path = dir_path.clone().join("v2rayN.exe");
    let shortcut_path = dirs::desktop_dir().unwrap().join("v2rayN.exe");
    symlink_file(exe_path, shortcut_path).unwrap();

    println!("Done");
}

fn get_latest_download_url() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let latest_release_url = "https://github.com/2dust/v2rayN/releases/latest";

    // Resolve the redirect to get the latest tag version
    let response = client.head(latest_release_url).send()?;
    let resolved_url = response.url().clone();

    // Build the URL for the latest zip file dynamically
    let version = resolved_url
        .path_segments()
        .and_then(|segments| segments.last())
        .ok_or("Failed to extract version from URL")?;
    let download_url = format!(
        "https://github.com/2dust/v2rayN/releases/download/{}/v2rayN-windows-64-desktop.zip",
        version
    );
    Ok(download_url)
}
