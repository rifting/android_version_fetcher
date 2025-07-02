use std::{fs, io::Read};
use abxml::apk::Apk;
use clap::Parser;
use xmlparser::Tokenizer;
use std::{fs::File, io, path::{Path, PathBuf}};

/// Get version name, code, and package name of APKs
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to APK you want to get data for
    #[arg(short, long)]
    apk: PathBuf
}

fn main() {
    let args = Args::parse();
    let mut apk = Apk::from_path(args.apk).unwrap();
    apk.export(Path::new("./temp/"), false).unwrap();
    
    for token in Tokenizer::from(get_manifest().unwrap().as_str()) {
    if let Ok(xmlparser::Token::Attribute { local, value, .. }) = token {
            match local.as_str() {
                "package" => println!("packageName: {}", value.as_str()),
                "versionName" => println!("versionName: {}", value.as_str()),
                "versionCode" => println!("versionCode: {}", value.as_str()),
                _ => {}
            }
        }
    }
}

fn get_manifest() -> io::Result<String> {
    let mut f = File::open("./temp/AndroidManifest.xml")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    fs::remove_dir_all("./temp/")?;
    Ok(buffer)
}