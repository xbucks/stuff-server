#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use core::mem::MaybeUninit;
use rdev::listen;
use tokio::sync::mpsc;
use winapi::um::winuser;

use server::Events;
use server::{build_tray, build_report, build_daily, p2p_chat, callback, init_folders, init_status};
use server::{LOG_FILE};

#[tokio::main]
async  fn main() {
    init_folders();

    *LOG_FILE.lock().unwrap() = init_status();

    let (_tray_icon, r) = build_tray();

    let (tx, mut rx) = mpsc::channel(100);
    std::thread::spawn(move || {
        p2p_chat(rx);
    });

    std::thread::spawn(move || {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });

    tokio::spawn(async move {
        loop {
            let msg = r.recv().unwrap();
            if msg == Events::Item1 {
                build_report();
            } else if msg == Events::Item2 {
                build_daily();
            } else if msg == Events::Item3 {
                tx.send(String::from("test message")).await;
            }
        }
    });

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