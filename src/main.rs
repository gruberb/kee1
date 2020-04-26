use serde::Deserialize;
use std::{env, error::Error, ffi::OsString, fs::File, process};

#[derive(Debug, Deserialize)]
struct KeepassRecord {
    #[serde(rename = "Group")]
    group: String,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Username")]
    username: String,
    #[serde(rename = "Password")]
    password: String,
    #[serde(rename = "URL")]
    url: String,
    #[serde(rename = "Notes")]
    notes: String,
}

#[derive(Debug, Deserialize)]
struct OnePasswordRecord {
    title: String,
    url: String,
    username: String,
    password: String,
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn run(file: File) -> Result<(), Box<dyn Error>> {
    let mut kee: Vec<KeepassRecord> = Vec::new();

    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: KeepassRecord = result?;
        kee.push(record);
    }

    write(kee)?;
    Ok(())
}

fn write(records: Vec<KeepassRecord>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("1password.csv")?;

    wtr.write_record(&["title", "url", "username", "password"])?;

    for record in records.iter() {
        wtr.write_record(&[
            &record.title,
            &record.url,
            &record.username,
            &record.password,
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;

    if let Err(err) = run(file) {
        println!("{}", err);
        process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, path::Path};

    fn create_vec(file: File) -> Result<Vec<OnePasswordRecord>, Box<dyn Error>> {
        let mut kee: Vec<OnePasswordRecord> = Vec::new();

        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.deserialize() {
            let record: OnePasswordRecord = result?;
            kee.push(record);
        }

        Ok(kee)
    }

    #[test]
    fn test_transforming_keepass_to_1pass() {
        let file = File::open("keepass_example.csv").expect("File not found");
        match run(file) {
            Ok(_) => {
                assert_eq!(true, Path::new("1password.csv").exists());
                let file = File::open("1password.csv").expect("File not found");
                let passwords = create_vec(file).expect("Could not read file");
                assert_eq!(passwords[0].title, "bank");
                assert_eq!(passwords[1].title, "PayPal");
                std::fs::remove_file("1password.csv").expect("test file could not be removed");
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
