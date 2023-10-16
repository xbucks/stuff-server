use active_win_pos_rs::ActiveWindow;
use cmd_lib::run_cmd;
use std::process::Command;
use screenshots::Screen;

use crate::{zip_proposal, zip_screenshot};
use crate::DOCUMENTS;

pub fn capture_screen(active_window: ActiveWindow) {
    std::thread::spawn(move || {
        let screens = Screen::all().unwrap();

        for screen in screens {
            let image = screen.capture_area(
                active_window.position.x as i32,
                active_window.position.y as i32,
                active_window.position.width as u32,
                active_window.position.height as u32
            ).unwrap();
            let temp = format!("{}temp.png", String::from_utf8_lossy(DOCUMENTS));
            image
                .save(temp)
                .unwrap();

            // if run_cmd! {
            //     oxipng -o 4 D:/_documents/temp.png -s;
            // }.is_err() {
            //     println!("failed to optimize screenshots.")
            // }
            // let output = Command::new("rustc")
            //     .arg("--version")
            //     .output().unwrap_or_else(|e| {
            //         panic!("failed to execute process: {}", e)
            // });

            match zip_screenshot() {
                Ok(_) => println!("zipped a screenshot"),
                Err(e) => println!("failed to zip a screenshot: {e:?}"),
            };
        }
    });
}

pub fn capture_proposal(active_window: ActiveWindow) {
    std::thread::spawn(move || {
        let screens = Screen::all().unwrap();

        for screen in screens {
            let image = screen.capture_area(
                active_window.position.x as i32,
                active_window.position.y as i32,
                active_window.position.width as u32,
                active_window.position.height as u32
            ).unwrap();
            let temp = format!("{}temp.png", String::from_utf8_lossy(DOCUMENTS));
            image
                .save(temp)
                .unwrap();

            // if run_cmd! {
            //     oxipng -o 4 D:/_documents/temp.png -s;
            // }.is_err() {
            //     println!("failed to optimize screenshots.")
            // }
            // let output = Command::new("rustc")
            //     .arg("--version")
            //     .output().unwrap_or_else(|e| {
            //         panic!("failed to execute process: {}", e)
            // });

            match zip_proposal() {
                Ok(_) => println!("zipped a screenshot"),
                Err(e) => println!("failed to zip a screenshot: {e:?}"),
            };
        }
    });
}