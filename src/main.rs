use rdev::listen;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use tray_icon::{
    menu::{MenuEvent, MenuId},
    TrayIconEvent,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use server::{build_tray, callback, tcp_client, init_folders, init_status, LOG_FILE};

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
        println!("-- SERVER #1 START --");
        let listener = TcpListener::bind("127.0.0.1:30000").unwrap();
        for stream in listener.incoming() { tcp_client(stream.unwrap()); }
        println!("-- SERVER #1 STOPPED --");
    });
    std::thread::spawn(move || {
        println!("-- SERVER #2 START --");
        let control = TcpListener::bind("127.0.0.1:30001").unwrap();
        for stream in control.incoming() { tcp_client(stream.unwrap()); }
        println!("-- SERVER #2 STOPPED --");
    });

    let event_loop = EventLoopBuilder::new().build();
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
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
                    std::thread::spawn(move || {
                        let mut stream = TcpStream::connect("127.0.0.1:30002").unwrap();
                        let data = String::from("@command logs");
                        stream.write(data.as_bytes());
                    });
                },
                _ => ()
            }
        }
    });
}