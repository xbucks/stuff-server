mod tray;
mod event;
mod ocr;
mod screen;
mod tcp;
mod zip;

pub use screen::capture_screen;
pub use tcp::tcp_client;
pub use event::callback;
pub use tray::build_tray;
pub use zip::{read_zip, zip_screenshot, zip_text};
pub use ocr::read_screenshot;

use chrono::{Utc, DateTime};
use preferences::{AppInfo, PreferencesMap, Preferences};
use std::fs;
use std::path::PathBuf;

static APP_INFO: AppInfo = AppInfo{name: "monitor", author: "Hiroki Moto"};
static PREFES_KEY: &str = "info/docs/monitor";

pub fn init_folders() {
    let mut path = PathBuf::from("D:\\");
    path.push("_documents");
    if !path.exists() {
        match fs::create_dir("D:\\_documents") {
            Ok(..) => {
                match fs::create_dir("D:\\_documents/logs") {
                    Ok(..) => (),
                    Err(..) => {
                        print!("failed to create documents/logs folders.");
                    }
                };
                match fs::create_dir("D:\\_documents/screens") {
                    Ok(..) => (),
                    Err(..) => {
                        print!("failed to create documents/screens folders.");
                    }
                };
            },
            Err(..) => {
                print!("failed to create documents folders.");
                std::process::exit(0);
            }
        };
    }
}

pub fn init_status(status: &str) -> String {
    let logs = String::new();

    let now: DateTime<Utc> = Utc::now();
    let fname = now.format("%Y-%m-%d").to_string();
    logs = read_zip(&fname, "log.txt");

    let load_result = PreferencesMap::<String>::load(&APP_INFO, PREFES_KEY);
    match load_result {
        Ok(prefs) => {
            println!("{:?}", prefs.get("boot".into()).unwrap());
            let info: String = format!("<<<<<<<<<<<<<<<<<{}<<<<<<<<<<<<<<<<<\n", prefs.get("boot".into()).unwrap());
            logs += &info;
        },
        Err(..) => {}
    };


    let now = Utc::now();
    let x: String = format!("{}", now);
    let now_parsed: DateTime<Utc> = x.parse().unwrap();
    let info: String = format!(">>>>>>>>>>>>>>>>>{}>>>>>>>>>>>>>>>>>\n", now_parsed.to_string());
    logs += &info;

    match zip_text(logs) {
        Ok(_) => {
            println!("Monitor has recorded machine {} status.", status);
        },
        Err(e) => println!("Error: {e:?}"),
    };

    logs
}