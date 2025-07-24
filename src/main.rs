extern crate clap;

use colored::*;
use std::io::Read;
use clap::{App, Arg};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{Write};
use indicatif::{ProgressBar, ProgressStyle};
use console::style;

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("You <you@example.com>")
        .about("wget clone written in Rust")
        .arg(Arg::with_name("URL")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("url to download"))
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    download_file(url).unwrap();
}

fn download_file(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut response = client.get(url).send()?;

    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;

    let file_name = url.split("/").last().unwrap_or("output.file");
    let mut file = File::create(file_name)?;

    println!("{}", style(format!("Downloading {}...", file_name)).cyan());

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("##-"));
    pb.set_message("Downloading");

    let mut downloaded: u64 = 0;
    let mut buffer = [0; 8192];

    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    println!("{}", "Downloading...".yellow().bold());
    println!("{}", "Download complete!".green().bold());
    println!("Saved to:\n{}", file_name.cyan());


    Ok(())
}
