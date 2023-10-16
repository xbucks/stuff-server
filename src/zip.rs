use chrono::{Utc, DateTime};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use zip::write::{FileOptions, ZipWriter};
use zip::result::ZipResult;
use zip::read::ZipArchive;
use zip::unstable::write::FileOptionsExt;
use crate::{DOCUMENTS, PASS};

pub fn zip_text(logs: String) -> ZipResult<()> {
    let now: DateTime<Utc> = Utc::now();
    let fname = format!("{}logs/{}.zip", String::from_utf8_lossy(DOCUMENTS), now.format("%Y-%m-%d").to_string());

    let path = std::path::Path::new(&fname);
    let file = std::fs::File::create(path).unwrap();

    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755)
        .with_deprecated_encryption(PASS);

    zip.start_file("log.txt", options)?;
    zip.write_all(logs.as_bytes())?;
    zip.finish()?;

    Ok(())
}

pub fn read_zip(filename: &str, logname: &str) -> String {
    let fname = format!("{}logs/{}.zip", String::from_utf8_lossy(DOCUMENTS), filename);
    let file = match fs::File::open(fname) {
        Ok(file) => file,
        Err(_) => {
            match zip_text(String::from("")) {
                Ok(_) => println!("Created an empty log zip file."),
                Err(e) => println!("Error: {e:?}"),
            };
            return String::from("");
        }
    };

    let reader = BufReader::new(file);

    let mut archive = ZipArchive::new(reader).unwrap();

    let mut file = match archive.by_name_decrypt(&logname, PASS) {
        Ok(file) => {
            if file.is_err() {
                println!("invalid password");
                match zip_text(String::from("")) {
                    Ok(_) => println!("Created an empty log zip file."),
                    Err(e) => println!("Error: {e:?}"),
                };
                return String::from("");
            }
            file.unwrap()
        },
        Err(..) => {
            println!("File {} not found in the zip.", logname);
            return String::from("");
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn zip_screenshot() -> ZipResult<()> {
    let now: DateTime<Utc> = Utc::now();
    let fname = format!("{}screens/{}.zip", String::from_utf8_lossy(DOCUMENTS), now.format("%Y-%m-%d").to_string());

    let path = std::path::Path::new(&fname);

    let mut file: File;
    let mut zip: ZipWriter<File>;

    if path.exists() {
        file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path).unwrap();

        zip = ZipWriter::new_append(file).unwrap();
    } else {
        file = std::fs::File::create(path).unwrap();
        zip = ZipWriter::new(file);
    }

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755)
        .with_deprecated_encryption(PASS);

    zip.start_file(now.format("%Y-%m-%d-%H:%M:%S.png").to_string(), options)?;

    let temp = format!("{}temp.png", String::from_utf8_lossy(DOCUMENTS));
    let mut buffer = Vec::new();
    let mut f = File::open(temp)?;
    f.read_to_end(&mut buffer)?;
    zip.write_all(&*buffer)?;
    buffer.clear();

    zip.finish()?;

    Ok(())
}

pub fn zip_proposal() -> ZipResult<()> {
    let now: DateTime<Utc> = Utc::now();
    let fname = format!("{}proposals/{}.zip", String::from_utf8_lossy(DOCUMENTS), now.format("%Y-%m-%d").to_string());

    let path = std::path::Path::new(&fname);

    let mut file: File;
    let mut zip: ZipWriter<File>;

    if path.exists() {
        file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path).unwrap();

        zip = ZipWriter::new_append(file).unwrap();
    } else {
        file = std::fs::File::create(path).unwrap();
        zip = ZipWriter::new(file);
    }

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755)
        .with_deprecated_encryption(PASS);

    zip.start_file(now.format("%Y-%m-%d-%H:%M:%S.png").to_string(), options)?;

    let temp = format!("{}temp.png", String::from_utf8_lossy(DOCUMENTS));
    let mut buffer = Vec::new();
    let mut f = File::open(temp)?;
    f.read_to_end(&mut buffer)?;
    zip.write_all(&*buffer)?;
    buffer.clear();

    zip.finish()?;

    Ok(())
}