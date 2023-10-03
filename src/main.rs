use rdev::listen;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::prelude::*;
use tray_icon::{
    menu::MenuEvent,
    TrayIconEvent,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

use server::{build_tray, callback, tcp_client, init_folders, init_status, net_server, p2p, p2p_chat};
use server::{LOG_FILE};
use server::Command;

fn main() {
    // init_folders();

    // *LOG_FILE.lock().unwrap() = init_status();

    let _tray_icon = build_tray();

    // std::thread::spawn(move || {
    //     if let Err(error) = listen(callback) {
    //         println!("Error: {:?}", error)
    //     }
    // });

    std::thread::spawn(move || {
        p2p_chat();
    });

    let event_loop = EventLoopBuilder::new().build();
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = tray_channel.try_recv() {
            match event.click_type {
                tray_icon::ClickType::Left => (),
                _ => ()
            }
        }
        if let Ok(event) = menu_channel.try_recv() {
            println!("menu event: {:?}", event);
            match event.id.as_ref() {
                "1001" => {
                    println!("user clicked menu #1 item.");
                },
                _ => ()
            }
        }
    });
}