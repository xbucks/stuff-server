#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use core::mem::MaybeUninit;
use rdev::listen;
use winapi::um::winuser;

use server::{build_tray, callback, init_folders, init_status, build_clock};
use server::{LOG_FILE};
use server::build_settings;
use server::Events;

fn main() {
    init_folders();

    *LOG_FILE.lock().unwrap() = init_status();

    let (_tray_icon, r) = build_tray();

    std::thread::spawn(move || {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });

    std::thread::spawn(move || {
        r.iter().for_each(|m| match m {
            Events::DoubleClickTrayIcon => {
                println!("Double click");
            }
            Events::ClickTrayIcon => {
                println!("Single click");
            }
            Events::Exit => {
                println!("Please exit");
            }
            Events::Item1 => {
                println!("Please item1");
            }
            Events::Item2 => {
                println!("Please item2");
            }
            Events::Item3 => {
                println!("Please item3");
            }
            e => {
                println!("{:?}", e);
            }
        })
    });

    // build_settings();
    build_clock();

    loop {
        unsafe {
            let mut msg = MaybeUninit::uninit();
            let bret = winuser::GetMessageA(msg.as_mut_ptr(), 0 as _, 0, 0);
            if bret > 0 {
                winuser::TranslateMessage(msg.as_ptr());
                winuser::DispatchMessageA(msg.as_ptr());
            } else {
                break;
            }
        }
    }
}