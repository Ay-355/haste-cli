use clap::ArgMatches;
use std::fs::File;
use std::io::{self, Read};
mod app;
mod http;

fn handle_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut buf = stdin.lock();
    buf.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn get_input(matches: &ArgMatches) -> io::Result<String> {
    match matches.value_of("FILE") {
        Some(file_path) => {
            if file_path == "-" {
                handle_stdin()
            } else {
                let mut buffer = String::new();
                let mut file = File::open(file_path)?;
                file.read_to_string(&mut buffer)?;
                Ok(buffer)
            }
        }
        None => handle_stdin(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = app::build_app().get_matches();
    let content = get_input(&matches)?;
    let key = http::upload_content(content)?;
    let url = format!("https://hastebin.com/{}", key);
    println!("Success! {}", url);
    Ok(())
}
